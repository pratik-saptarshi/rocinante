#!/usr/bin/env python3
"""Verify and stage the official prebuilt DuckDB library for Cargo."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import shutil
import sys
import urllib.request
import zipfile
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_MANIFEST = Path(__file__).with_name("duckdb-prebuilt-artifacts.json")


class ProvisionError(RuntimeError):
    """Raised when a prebuilt DuckDB artifact cannot be verified."""


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def duckdb_version_from_binding(binding_version: str) -> str:
    parts = binding_version.split(".")
    if len(parts) != 3 or parts[0] != "1" or not parts[1].isdigit():
        raise ProvisionError(f"Unsupported DuckDB Rust binding version: {binding_version}")
    encoded = int(parts[1])
    return f"{encoded // 10000}.{(encoded // 100) % 100}.{encoded % 100}"


def host_target() -> str:
    system = platform.system().lower()
    machine = platform.machine().lower()
    if system == "linux" and machine in {"x86_64", "amd64"}:
        return "x86_64-unknown-linux-gnu"
    if system == "darwin" and machine in {"x86_64", "amd64"}:
        return "x86_64-apple-darwin"
    if system == "darwin" and machine in {"arm64", "aarch64"}:
        return "aarch64-apple-darwin"
    if system == "windows" and machine in {"amd64", "x86_64"}:
        return "x86_64-pc-windows-msvc"
    raise ProvisionError(f"Unsupported DuckDB target: {system}/{machine}")


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        manifest = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ProvisionError(f"Could not read DuckDB artifact manifest {path}: {error}") from error
    return validate_manifest_data(manifest)


def validate_manifest_data(manifest: dict[str, Any]) -> dict[str, Any]:
    if duckdb_version_from_binding(manifest["duckdb_rs_version"]) != manifest["duckdb_version"]:
        raise ProvisionError(
            "DuckDB Rust binding and native library version do not match: "
            f"{manifest['duckdb_rs_version']} vs {manifest['duckdb_version']}"
        )
    return manifest


def download_archive(url: str, destination: Path, expected_sha256: str) -> None:
    temporary = destination.with_suffix(destination.suffix + ".download")
    destination.parent.mkdir(parents=True, exist_ok=True)
    try:
        request = urllib.request.Request(url, headers={"User-Agent": "rocinante-duckdb-provisioner"})
        with urllib.request.urlopen(request, timeout=180) as response, temporary.open("wb") as output:
            shutil.copyfileobj(response, output)
        actual_sha256 = sha256_file(temporary)
        if actual_sha256 != expected_sha256:
            raise ProvisionError(
                f"DuckDB archive checksum mismatch: expected {expected_sha256}, got {actual_sha256}"
            )
        os.replace(temporary, destination)
    except Exception:
        temporary.unlink(missing_ok=True)
        raise


def ensure_archive(
    asset: dict[str, Any],
    release_url: str,
    cache_dir: Path,
    archive_override: Path | None = None,
) -> Path:
    archive_path = cache_dir / asset["archive"]
    if archive_override is not None:
        archive_path.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(archive_override, archive_path)
    if archive_path.exists():
        actual_sha256 = sha256_file(archive_path)
        if actual_sha256 != asset["archive_sha256"]:
            raise ProvisionError(
                f"Cached DuckDB archive checksum mismatch: expected {asset['archive_sha256']}, "
                f"got {actual_sha256}; remove {archive_path} and retry"
            )
        return archive_path
    url = f"{release_url.rstrip('/')}/{asset['archive']}"
    download_archive(url, archive_path, asset["archive_sha256"])
    return archive_path


def extract_verified_files(archive_path: Path, asset: dict[str, Any], cache_dir: Path) -> None:
    try:
        with zipfile.ZipFile(archive_path) as archive:
            members = set(archive.namelist())
            for name, expected_sha256 in asset["files"].items():
                if Path(name).name != name:
                    raise ProvisionError(f"Unsafe artifact member path: {name}")
                if name not in members:
                    raise ProvisionError(f"DuckDB archive is missing expected file: {name}")
                destination = cache_dir / name
                if destination.is_file() and sha256_file(destination) == expected_sha256:
                    continue
                temporary = destination.with_suffix(destination.suffix + ".extracting")
                with archive.open(name) as source, temporary.open("wb") as output:
                    shutil.copyfileobj(source, output)
                actual_sha256 = sha256_file(temporary)
                if actual_sha256 != expected_sha256:
                    temporary.unlink(missing_ok=True)
                    raise ProvisionError(
                        f"DuckDB file checksum mismatch for {name}: expected {expected_sha256}, "
                        f"got {actual_sha256}"
                    )
                os.replace(temporary, destination)
    except (OSError, zipfile.BadZipFile, KeyError) as error:
        raise ProvisionError(f"Could not extract verified DuckDB files: {error}") from error


def provision_target(
    manifest: dict[str, Any],
    target: str,
    target_dir: Path,
    *,
    cargo_target_layout: bool = False,
    archive_override: Path | None = None,
) -> Path:
    try:
        asset = manifest["targets"][target]
    except KeyError as error:
        raise ProvisionError(f"No prebuilt DuckDB artifact is pinned for target {target}") from error

    cache_root = target_dir / target if cargo_target_layout else target_dir
    cache_dir = cache_root / "duckdb-download" / target / manifest["duckdb_version"]
    cache_dir.mkdir(parents=True, exist_ok=True)
    archive_path = ensure_archive(asset, manifest["release_url"], cache_dir, archive_override)
    extract_verified_files(archive_path, asset, cache_dir)
    return cache_dir


def stage_runtime_for_binary(
    manifest: dict[str, Any],
    target: str,
    cache_dir: Path,
    binary_path: Path,
) -> Path:
    asset = manifest["targets"][target]
    runtime_names = [
        name for name in ("libduckdb.so", "libduckdb.dylib", "duckdb.dll")
        if name in asset["files"]
    ]
    if len(runtime_names) != 1:
        raise ProvisionError(f"Expected exactly one DuckDB runtime artifact for target {target}")

    runtime_name = runtime_names[0]
    source = cache_dir / runtime_name
    expected_sha256 = asset["files"][runtime_name]
    if not source.is_file() or sha256_file(source) != expected_sha256:
        raise ProvisionError(f"Verified DuckDB runtime is missing or corrupt: {source}")

    destination_directory = Path(binary_path).resolve().parent / "deps"
    destination = destination_directory / runtime_name
    temporary = destination.with_name(f".{destination.name}.staging")
    destination_directory.mkdir(parents=True, exist_ok=True)
    try:
        shutil.copyfile(source, temporary)
        if sha256_file(temporary) != expected_sha256:
            raise ProvisionError(f"Staged DuckDB runtime failed checksum verification: {temporary}")
        os.replace(temporary, destination)
    except OSError as error:
        temporary.unlink(missing_ok=True)
        raise ProvisionError(f"Could not stage DuckDB runtime beside {binary_path}: {error}") from error
    return destination


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--target", default=None, help="Rust target triple (defaults to this host)")
    parser.add_argument("--target-dir", type=Path, default=None, help="Cargo target directory")
    parser.add_argument(
        "--cargo-target-layout",
        action="store_true",
        help="Stage under target-dir/<triple> for cargo commands using --target <triple>",
    )
    parser.add_argument("--print-dir", action="store_true", help="Print only the verified cache directory")
    parser.add_argument(
        "--stage-runtime-for-binary",
        type=Path,
        default=None,
        metavar="BINARY",
        help="Copy the verified native runtime to BINARY's sibling deps directory",
    )
    args = parser.parse_args()

    try:
        manifest = load_manifest(args.manifest)
        target = args.target or host_target()
        target_dir = args.target_dir or os.environ.get("CARGO_TARGET_DIR") or REPO_ROOT / "src-tauri" / "target"
        target_dir = Path(target_dir)
        if not target_dir.is_absolute():
            target_dir = REPO_ROOT / target_dir
        cache_dir = provision_target(
            manifest,
            target,
            target_dir,
            cargo_target_layout=args.cargo_target_layout,
        )
        staged_runtime = None
        if args.stage_runtime_for_binary is not None:
            staged_runtime = stage_runtime_for_binary(manifest, target, cache_dir, args.stage_runtime_for_binary)
    except (ProvisionError, KeyError, OSError) as error:
        print(f"DuckDB provisioning failed: {error}", file=sys.stderr)
        return 1

    if args.print_dir:
        print(cache_dir)
    else:
        print(f"Verified DuckDB {manifest['duckdb_version']} prebuilt for {target}: {cache_dir}")
        if staged_runtime is not None:
            print(f"Staged verified DuckDB runtime beside desktop binary: {staged_runtime}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
