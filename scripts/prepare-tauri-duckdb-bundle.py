#!/usr/bin/env python3
"""Stage the verified DuckDB runtime and make the Tauri executable relocatable."""

from __future__ import annotations

import importlib.util
import os
import subprocess
import sys
from pathlib import Path
from typing import Callable, Sequence


REPO_ROOT = Path(__file__).resolve().parent.parent
TAURI_ROOT = REPO_ROOT / "src-tauri"
sys.dont_write_bytecode = True
PROVISIONER_SPEC = importlib.util.spec_from_file_location(
    "rocinante_duckdb_provisioner", Path(__file__).with_name("provision_duckdb.py")
)
if PROVISIONER_SPEC is None or PROVISIONER_SPEC.loader is None:
    raise RuntimeError("Could not load the DuckDB provisioner")
provisioner = importlib.util.module_from_spec(PROVISIONER_SPEC)
PROVISIONER_SPEC.loader.exec_module(provisioner)


class BundlePreparationError(RuntimeError):
    """Raised when the Tauri bundle cannot be made self-contained."""


CommandRunner = Callable[[Sequence[str]], subprocess.CompletedProcess[str]]
UNIVERSAL_MACOS_TARGET = "universal-apple-darwin"
UNIVERSAL_MACOS_ARTIFACT_TARGET = "aarch64-apple-darwin"


def _run(command: Sequence[str]) -> subprocess.CompletedProcess[str]:
    try:
        return subprocess.run(command, check=True, capture_output=True, text=True)
    except (OSError, subprocess.CalledProcessError) as error:
        detail = getattr(error, "stderr", None) or str(error)
        raise BundlePreparationError(f"Command failed ({' '.join(command)}): {detail}") from error


def _output(runner: CommandRunner, command: Sequence[str]) -> str:
    try:
        result = runner(command)
    except (OSError, subprocess.CalledProcessError) as error:
        detail = getattr(error, "stderr", None) or str(error)
        raise BundlePreparationError(f"Command failed ({' '.join(command)}): {detail}") from error
    return result.stdout or ""


def _macos_load_commands(binary: Path, library: Path, runner: CommandRunner) -> None:
    library_id_output = _output(runner, ["otool", "-D", str(library)])
    library_id_lines = [line.strip() for line in library_id_output.splitlines()[1:] if line.strip()]
    if not library_id_lines:
        raise BundlePreparationError(f"Could not read DuckDB install name from {library}")
    library_id = library_id_lines[0]
    expected_name = "@rpath/libduckdb.dylib"
    if library_id != expected_name:
        runner(["install_name_tool", "-id", expected_name, str(library)])

    linked_libraries = _output(runner, ["otool", "-L", str(binary)])
    duckdb_load_names = [
        line.strip().split(" (", 1)[0]
        for line in linked_libraries.splitlines()[1:]
        if "libduckdb.dylib" in line
    ]
    if not duckdb_load_names:
        raise BundlePreparationError(f"The Tauri executable does not link DuckDB: {binary}")
    for load_name in duckdb_load_names:
        if load_name != expected_name:
            runner(["install_name_tool", "-change", load_name, expected_name, str(binary)])

    load_commands = _output(runner, ["otool", "-l", str(binary)])
    rpaths: list[str] = []
    awaiting_path = False
    for line in load_commands.splitlines():
        stripped = line.strip()
        if stripped == "cmd LC_RPATH":
            awaiting_path = True
        elif awaiting_path and stripped.startswith("path "):
            rpaths.append(stripped.split()[1])
            awaiting_path = False
    bundle_rpath = "@executable_path/../Resources"
    for rpath in rpaths:
        if "duckdb-download" in rpath:
            runner(["install_name_tool", "-delete_rpath", rpath, str(binary)])
    if bundle_rpath not in rpaths:
        runner(["install_name_tool", "-add_rpath", bundle_rpath, str(binary)])


def patch_binary_loader(target: str, binary: Path, library: Path, runner: CommandRunner = _run) -> None:
    if not binary.is_file():
        raise BundlePreparationError(f"Tauri release executable was not built: {binary}")
    if not library.is_file():
        raise BundlePreparationError(f"Staged DuckDB runtime is missing: {library}")

    if target.endswith("-linux-gnu"):
        runner(
            [
                "patchelf",
                "--set-rpath",
                "$ORIGIN/../lib/Rocinante Repo Analyzer",
                str(binary),
            ]
        )
    elif "-apple-darwin" in target:
        _macos_load_commands(binary, library, runner)
    elif target.endswith("-pc-windows-msvc"):
        # Tauri's Windows resource directory is the executable directory, which
        # is also in the operating system's DLL search path.
        return
    else:
        raise BundlePreparationError(f"Unsupported Tauri DuckDB bundle target: {target}")


def _target_for_bundle() -> str:
    return os.environ.get("TAURI_ENV_TARGET_TRIPLE") or provisioner.host_target()


def _target_directory() -> Path:
    configured = os.environ.get("CARGO_TARGET_DIR")
    if configured:
        path = Path(configured)
        return path if path.is_absolute() else TAURI_ROOT / path
    return TAURI_ROOT / "target"


def _artifact_target_for_bundle(target: str) -> str:
    if target == UNIVERSAL_MACOS_TARGET:
        # Both architecture entries reference the same verified universal dylib.
        return UNIVERSAL_MACOS_ARTIFACT_TARGET
    return target


def prepare_bundle(target: str, target_directory: Path, runner: CommandRunner = _run) -> Path:
    manifest = provisioner.load_manifest(provisioner.DEFAULT_MANIFEST)
    artifact_target = _artifact_target_for_bundle(target)
    cache_directory = provisioner.provision_target(manifest, artifact_target, target_directory)
    resource_directory = TAURI_ROOT / "tauri-resources"
    staged_library = provisioner.stage_runtime_to_directory(
        manifest, artifact_target, cache_directory, resource_directory
    )

    executable_name = "rocinante-repo-analyzer.exe" if target.endswith("-pc-windows-msvc") else "rocinante-repo-analyzer"
    candidates = [
        target_directory / target / "release" / executable_name,
        target_directory / "release" / executable_name,
    ]
    executable = next((candidate for candidate in candidates if candidate.is_file()), candidates[0])
    patch_binary_loader(target, executable, staged_library, runner)
    return staged_library


def main() -> int:
    try:
        target = _target_for_bundle()
        staged = prepare_bundle(target, _target_directory())
    except (BundlePreparationError, provisioner.ProvisionError, OSError, KeyError) as error:
        print(f"Tauri DuckDB bundle preparation failed: {error}", file=sys.stderr)
        return 1
    version = provisioner.load_manifest(provisioner.DEFAULT_MANIFEST)["duckdb_version"]
    print(f"Prepared verified DuckDB {version} for Tauri: {staged}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
