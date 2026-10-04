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
            output = "Load command 0\n cmd LC_LOAD_DYLIB\n"
        else:
            output = ""
        return CompletedProcess(command, 0, output, "")

    def test_linux_loader_points_at_tauri_resource_directory(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            binary, library = self.make_binary_and_library(Path(directory), "libduckdb.so")
            bundle.patch_binary_loader("x86_64-unknown-linux-gnu", binary, library, self.runner)

        self.assertIn(
            ["patchelf", "--set-rpath", "$ORIGIN/../lib/rocinante-repo-analyzer", str(binary)],
            self.commands,
        )

    def test_macos_rewrites_duckdb_load_name_and_adds_resources_rpath(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            binary, library = self.make_binary_and_library(Path(directory), "libduckdb.dylib")
            bundle.patch_binary_loader("aarch64-apple-darwin", binary, library, self.runner)

        self.assert_command("install_name_tool", "-id", "@rpath/libduckdb.dylib")
        self.assert_command("install_name_tool", "-change", "/temporary/cache/libduckdb.dylib")
        self.assert_command("install_name_tool", "-add_rpath", "@executable_path/../Resources")

    def test_windows_relies_on_tauri_resource_path_next_to_executable(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            binary, library = self.make_binary_and_library(Path(directory), "duckdb.dll")
            bundle.patch_binary_loader("x86_64-pc-windows-msvc", binary, library, self.runner)

        self.assertEqual(self.commands, [])

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
                root / "target" / "tauri-resources",
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
                    root / "target" / "tauri-resources",
                )

    def test_tauri_config_bundles_runtime_and_runs_platform_hooks(self) -> None:
        tauri_root = SCRIPT.parent.parent / "src-tauri"
        config = json.loads((tauri_root / "tauri.conf.json").read_text())
        windows_config = json.loads((tauri_root / "tauri.windows.conf.json").read_text())

        self.assertEqual(config["bundle"]["resources"], {"target/tauri-resources/*": ""})
        self.assertEqual(config["build"]["beforeBuildCommand"], "python3 ../scripts/provision_duckdb.py")
        self.assertEqual(
            config["build"]["beforeBundleCommand"],
            "python3 ../scripts/prepare-tauri-duckdb-bundle.py",
        )
        self.assertEqual(
            windows_config["build"]["beforeBuildCommand"], "python ../scripts/provision_duckdb.py"
        )
        self.assertEqual(
            windows_config["build"]["beforeBundleCommand"],
            "python ../scripts/prepare-tauri-duckdb-bundle.py",
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
