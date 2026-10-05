from __future__ import annotations

import hashlib
import importlib.util
import sys
import tempfile
import unittest
import json
import zipfile
from pathlib import Path
from subprocess import CompletedProcess
from unittest import mock


SCRIPT = Path(__file__).with_name("prepare-tauri-duckdb-bundle.py")
sys.dont_write_bytecode = True
SPEC = importlib.util.spec_from_file_location("prepare_tauri_duckdb_bundle", SCRIPT)
assert SPEC and SPEC.loader
bundle = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(bundle)


class BundlePreparationTests(unittest.TestCase):
    def setUp(self) -> None:
        self.commands: list[list[str]] = []

    def runner(self, command: list[str]) -> CompletedProcess[str]:
        self.commands.append(command)
        if command[:2] == ["otool", "-D"]:
            output = f"{command[-1]}:\n/temporary/cache/libduckdb.dylib\n"
        elif command[:2] == ["otool", "-L"]:
            output = f"{command[-1]}:\n/temporary/cache/libduckdb.dylib (compatibility version 1.0.0)\n"
        elif command[:2] == ["otool", "-l"]:
            output = (
                "Load command 0\n"
                " cmd LC_RPATH\n"
                " cmdsize 48\n"
                " path /tmp/target/duckdb-download/aarch64-apple-darwin (offset 12)\n"
                "Load command 1\n"
                " cmd LC_LOAD_DYLIB\n"
            )
        else:
            output = ""
        return CompletedProcess(command, 0, output, "")

    def test_linux_loader_points_at_tauri_resource_directory(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            binary, library = self.make_binary_and_library(Path(directory), "libduckdb.so")
            bundle.patch_binary_loader("x86_64-unknown-linux-gnu", binary, library, self.runner)

        self.assertIn(
            ["patchelf", "--set-rpath", "$ORIGIN/../lib/Rocinante Repo Analyzer", str(binary)],
            self.commands,
        )

    def test_macos_removes_duckdb_cache_rpath_and_adds_resources_rpath(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            binary, library = self.make_binary_and_library(Path(directory), "libduckdb.dylib")
            bundle.patch_binary_loader("aarch64-apple-darwin", binary, library, self.runner)

        self.assert_command("install_name_tool", "-id", "@rpath/libduckdb.dylib")
        self.assert_command("install_name_tool", "-change", "/temporary/cache/libduckdb.dylib")
        self.assert_command(
            "install_name_tool",
            "-delete_rpath",
            "/tmp/target/duckdb-download/aarch64-apple-darwin",
        )
        self.assert_command("install_name_tool", "-add_rpath", "@executable_path/../Resources")

    def test_windows_relies_on_tauri_resource_path_next_to_executable(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            binary, library = self.make_binary_and_library(Path(directory), "duckdb.dll")
            bundle.patch_binary_loader("x86_64-pc-windows-msvc", binary, library, self.runner)

        self.assertEqual(self.commands, [])

    def test_universal_macos_uses_prebuilt_artifact_and_universal_output_directory(self) -> None:
        target = "universal-apple-darwin"
        artifact_target = "aarch64-apple-darwin"
        manifest = bundle.provisioner.load_manifest(bundle.provisioner.DEFAULT_MANIFEST)
        self.assertEqual(bundle._artifact_target_for_bundle(target), artifact_target)
        self.assertEqual(
            manifest["targets"][artifact_target]["archive"],
            manifest["targets"]["x86_64-apple-darwin"]["archive"],
        )

        with tempfile.TemporaryDirectory() as directory:
            target_directory = Path(directory)
            executable = (
                target_directory
                / target
                / "release"
                / "rocinante-repo-analyzer"
            )
            executable.parent.mkdir(parents=True)
            executable.write_bytes(b"binary")
            cache_directory = target_directory / "cache"
            staged_library = target_directory / "tauri-resources" / "libduckdb.dylib"

            with (
                mock.patch.object(bundle.provisioner, "load_manifest", return_value=manifest),
                mock.patch.object(
                    bundle.provisioner,
                    "provision_target",
                    return_value=cache_directory,
                ) as provision,
                mock.patch.object(
                    bundle.provisioner,
                    "stage_runtime_to_directory",
                    return_value=staged_library,
                ) as stage,
                mock.patch.object(bundle, "patch_binary_loader") as patch_loader,
            ):
                result = bundle.prepare_bundle(target, target_directory)

        provision.assert_called_once_with(manifest, artifact_target, target_directory)
        stage.assert_called_once_with(
            manifest,
            artifact_target,
            cache_directory,
            bundle.TAURI_ROOT / "tauri-resources",
        )
        patch_loader.assert_called_once_with(
            target,
            executable,
            staged_library,
            bundle._run,
        )
        self.assertEqual(result, staged_library)

    def test_target_specific_executable_wins_over_stale_host_release_binary(self) -> None:
        target = "universal-apple-darwin"
        executable_name = "rocinante-repo-analyzer"
        with tempfile.TemporaryDirectory() as directory:
            target_directory = Path(directory)
            host_executable = target_directory / "release" / executable_name
            target_executable = target_directory / target / "release" / executable_name
            host_executable.parent.mkdir(parents=True)
            target_executable.parent.mkdir(parents=True)
            host_executable.write_bytes(b"stale host executable")
            target_executable.write_bytes(b"target executable")
            manifest = bundle.provisioner.load_manifest(bundle.provisioner.DEFAULT_MANIFEST)
            cache_directory = target_directory / "cache"
            staged_library = target_directory / "tauri-resources" / "libduckdb.dylib"

            with (
                mock.patch.object(bundle.provisioner, "load_manifest", return_value=manifest),
                mock.patch.object(bundle.provisioner, "provision_target", return_value=cache_directory),
                mock.patch.object(
                    bundle.provisioner,
                    "stage_runtime_to_directory",
                    return_value=staged_library,
                ),
                mock.patch.object(bundle, "patch_binary_loader") as patch_loader,
            ):
                bundle.prepare_bundle(target, target_directory)

        self.assertEqual(patch_loader.call_args.args[1], target_executable)

    def test_staging_checks_the_expected_runtime_hash(self) -> None:
        runtime = b"verified DuckDB runtime"
        expected_hash = hashlib.sha256(runtime).hexdigest()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cache_directory = root / "cache"
            cache_directory.mkdir()
            (cache_directory / "libduckdb.so").write_bytes(runtime)
            manifest = {
                "targets": {
                    "test-target": {
                        "files": {"libduckdb.so": expected_hash},
                    }
                }
            }

            staged = bundle.provisioner.stage_runtime_to_directory(
                manifest,
                "test-target",
                cache_directory,
                root / "tauri-resources",
            )

            self.assertEqual(staged.read_bytes(), runtime)
            self.assertEqual(bundle.provisioner.sha256_file(staged), expected_hash)

    def test_tampered_runtime_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cache_directory = root / "cache"
            cache_directory.mkdir()
            (cache_directory / "libduckdb.so").write_bytes(b"tampered")
            manifest = {
                "targets": {
                    "test-target": {
                        "files": {"libduckdb.so": hashlib.sha256(b"verified").hexdigest()},
                    }
                }
            }

            with self.assertRaisesRegex(bundle.provisioner.ProvisionError, "missing or corrupt"):
                bundle.provisioner.stage_runtime_to_directory(
                    manifest,
                    "test-target",
                    cache_directory,
                    root / "tauri-resources",
                )

    def test_tauri_config_bundles_runtime_and_runs_platform_hooks(self) -> None:
        tauri_root = SCRIPT.parent.parent / "src-tauri"
        config = json.loads((tauri_root / "tauri.conf.json").read_text())
        windows_config = json.loads((tauri_root / "tauri.windows.conf.json").read_text())
        workflow = (SCRIPT.parent.parent / ".github/workflows/ci.yml").read_text()
        security_workflow = (SCRIPT.parent.parent / ".github/workflows/security.yml").read_text()

        self.assertEqual(config["bundle"]["resources"], {"tauri-resources/": ""})
        self.assertEqual(
            config["bundle"]["icon"],
            [
                "crates/rocinante-desktop-shell/packaging/icons/rocinante.png",
                "crates/rocinante-desktop-shell/packaging/icons/Rocinante.icns",
                "crates/rocinante-desktop-shell/packaging/icons/Rocinante.ico",
            ],
        )
        for icon in config["bundle"]["icon"]:
            self.assertTrue((tauri_root / icon).is_file(), f"Missing Tauri bundle icon {icon}")
        stage_runtime_step = workflow.index("Stage verified DuckDB before loading Tauri resources")
        build_bundle_step = workflow.index("Build production Tauri bundle")
        self.assertLess(stage_runtime_step, build_bundle_step)
        self.assertIn("python3 scripts/provision_duckdb.py --stage-runtime-for-tauri-bundle", workflow)
        self.assertIn("python scripts/provision_duckdb.py --stage-runtime-for-tauri-bundle", workflow)
        self.assertIn(r"$contents | Where-Object { $_ -match 'duckdb\.dll' }", workflow)
        self.assertIn("duckdb-download", workflow)
        codeql_provision_step = security_workflow.index(
            "Verify and stage the DuckDB prebuilt for CodeQL"
        )
        codeql_autobuild_step = security_workflow.index(
            "github/codeql-action/autobuild@"
        )
        self.assertLess(codeql_provision_step, codeql_autobuild_step)
        self.assertIn(
            "python3 scripts/provision_duckdb.py --target x86_64-unknown-linux-gnu",
            security_workflow,
        )
        self.assertIn("CARGO_TARGET_DIR: ${{ github.workspace }}/src-tauri/target", security_workflow)
        self.assertTrue((tauri_root / "tauri-resources" / "README.txt").is_file())
        self.assertEqual(
            config["build"]["beforeBuildCommand"],
            "python3 scripts/provision_duckdb.py --stage-runtime-for-tauri-bundle",
        )
        self.assertEqual(
            config["build"]["beforeBundleCommand"],
            "python3 scripts/prepare-tauri-duckdb-bundle.py",
        )
        self.assertEqual(
            windows_config["build"]["beforeBuildCommand"],
            "python scripts/provision_duckdb.py --stage-runtime-for-tauri-bundle",
        )
        self.assertEqual(
            windows_config["build"]["beforeBundleCommand"],
            "python scripts/prepare-tauri-duckdb-bundle.py",
        )

    def test_readme_stages_runtime_after_each_native_shell_release_build(self) -> None:
        readme = (SCRIPT.parent.parent / "README.md").read_text()
        build_command = "cargo build --release --manifest-path src-tauri/Cargo.toml -p rocinante-desktop-shell"
        stage_command = "--stage-runtime-for-binary"

        self.assertEqual(readme.count(build_command), 4)
        self.assertEqual(readme.count(stage_command), 4)

        lines = readme.splitlines()
        build_line_numbers = [index for index, line in enumerate(lines) if build_command in line]
        for build_line in build_line_numbers:
            next_platform_install = next(
                (index for index in range(build_line + 1, len(lines)) if "install-user." in lines[index]),
                None,
            )
            self.assertIsNotNone(next_platform_install)
            self.assertTrue(
                any(stage_command in lines[index] for index in range(build_line + 1, next_platform_install)),
                f"Missing staged DuckDB command after README build example on line {build_line + 1}",
            )

    @staticmethod
    def make_binary_and_library(root: Path, library_name: str) -> tuple[Path, Path]:
        binary = root / "rocinante-repo-analyzer"
        library = root / library_name
        binary.write_bytes(b"binary")
        library.write_bytes(b"library")
        return binary, library

    def assert_command(self, *parts: str) -> None:
        self.assertTrue(any(all(part in command for part in parts) for command in self.commands), self.commands)


if __name__ == "__main__":
    unittest.main()
