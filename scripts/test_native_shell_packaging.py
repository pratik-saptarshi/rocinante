#!/usr/bin/env python3
"""Contract checks for supported native-shell build and package paths."""

from __future__ import annotations

import json
import re
import tomllib
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


class NativeShellPackagingContractTests(unittest.TestCase):
    def setUp(self) -> None:
        self.manifest = tomllib.loads((ROOT / "src-tauri/Cargo.toml").read_text())
        self.storage_manifest = tomllib.loads((ROOT / "src-tauri/crates/rocinante-storage/Cargo.toml").read_text())
        self.artifacts = json.loads((ROOT / "scripts/duckdb-prebuilt-artifacts.json").read_text())
        self.bom_md = (ROOT / "docs/bill-of-materials.md").read_text()
        self.bom_html = (ROOT / "docs/bill-of-materials.html").read_text()
        self.workflow = (ROOT / ".github/workflows/ci.yml").read_text()

    def test_supported_rust_package_has_no_tauri_host_or_build_script(self) -> None:
        package = self.manifest["package"]
        self.assertNotIn("build", package)
        self.assertNotIn("[[bin]]", (ROOT / "src-tauri/Cargo.toml").read_text())
        dependencies = {
            **self.manifest.get("dependencies", {}),
            **self.manifest.get("dev-dependencies", {}),
            **self.manifest.get("build-dependencies", {}),
        }
        self.assertFalse({"tauri", "tauri-build", "wry"} & dependencies.keys())
        for retired_path in (
            "src-tauri/build.rs",
            "src-tauri/src/main.rs",
            "src-tauri/src/app_support.rs",
            "src-tauri/tauri.conf.json",
            "src-tauri/tauri.windows.conf.json",
        ):
            self.assertFalse((ROOT / retired_path).exists(), retired_path)

    def test_only_verified_prebuilt_duckdb_artifacts_are_declared(self) -> None:
        cargo = (ROOT / "src-tauri/Cargo.toml").read_text()
        storage = (ROOT / "src-tauri/crates/rocinante-storage/Cargo.toml").read_text()
        provisioner = (ROOT / "scripts/provision_duckdb.py").read_text()
        self.assertIn('duckdb = { version = "=1.10506.1", default-features = false', cargo)
        self.assertIn('duckdb = { version = "=1.10506.1", default-features = false', storage)
        self.assertNotRegex(cargo + storage, r'(?i)duckdb[^\n]*(bundled|bundled-cmake)')
        self.assertIn("--stage-runtime-for-binary", provisioner)
        self.assertNotIn("stage-runtime-for-tauri-bundle", provisioner)

    def test_duckdb_rs_version_matches_cargo_pins_and_bom_surfaces(self) -> None:
        root_dependency = self.manifest["dependencies"]["duckdb"]
        root_pin = root_dependency["version"].lstrip("=") if isinstance(root_dependency, dict) else root_dependency.lstrip("=")
        storage_dependency = self.storage_manifest["dependencies"]["duckdb"]
        storage_pin = storage_dependency["version"].lstrip("=") if isinstance(storage_dependency, dict) else storage_dependency.lstrip("=")
        self.assertEqual(root_pin, storage_pin)
        self.assertEqual(self.artifacts["duckdb_rs_version"], root_pin)
        bom_md_controls = self.bom_md.split("## Dependency Controls and Security Gate Stack", 1)[1].split("\n## ", 1)[0]
        bom_md_match = re.search(r"`duckdb` `([^`]+)`", bom_md_controls)
        self.assertIsNotNone(bom_md_match)
        self.assertEqual(bom_md_match.group(1), root_pin)
        bom_html_crates = self.bom_html.split("### Rust / backend crates", 1)[1].split("\n### ", 1)[0]
        bom_html_match = re.search(r"`duckdb` `([^`]+)`", bom_html_crates)
        self.assertIsNotNone(bom_html_match)
        self.assertEqual(bom_html_match.group(1), root_pin)

    def test_all_supported_platforms_build_and_package_the_native_shell(self) -> None:
        job = re.split(
            r"\n  [a-z0-9_-]+:\n",
            self.workflow.split("  native-shell-package:\n", 1)[1],
            maxsplit=1,
        )[0]
        for runner in ("ubuntu-latest", "macos-latest", "windows-latest"):
            self.assertIn(f"runner: {runner}", job)
        self.assertIn("cargo build --locked --release --manifest-path src-tauri/Cargo.toml -p rocinante-desktop-shell", job)
        self.assertIn("--stage-runtime-for-binary", job)
        self.assertIn("packaging/linux/install-user.sh", job)
        self.assertIn("packaging/macos/build-app-bundle.sh", job)
        self.assertIn("codesign --verify --deep --strict", job)
        self.assertIn("packaging/windows/install-user.ps1", job)
        self.assertIn("duckdb.dll", job)
        self.assertNotIn("@tauri-apps/cli", self.workflow)
        self.assertNotIn("tauri-runtime-bundle", self.workflow)

    def test_package_matrix_is_a_required_aggregate_gate(self) -> None:
        test_job = re.split(
            r"\n  [a-z0-9_-]+:\n",
            self.workflow.split("  test:\n", 1)[1],
            maxsplit=1,
        )[0]
        self.assertIn("- native-shell-package", test_job)
        self.assertIn('needs.native-shell-package.result', test_job)
        self.assertIn('!= "success"', test_job)

    def test_installer_scripts_keep_platform_relative_runtime_contracts(self) -> None:
        linux = (ROOT / "src-tauri/crates/rocinante-desktop-shell/packaging/linux/install-user.sh").read_text()
        macos = (ROOT / "src-tauri/crates/rocinante-desktop-shell/packaging/macos/build-app-bundle.sh").read_text()
        windows = (ROOT / "src-tauri/crates/rocinante-desktop-shell/packaging/windows/install-user.ps1").read_text()
        self.assertIn("patchelf --set-rpath '$ORIGIN/../lib/rocinante'", linux)
        self.assertIn("installed_duckdb=$contents/Frameworks/libduckdb.dylib", macos)
        self.assertIn("@executable_path/../Frameworks", macos)
        self.assertIn("codesign --verify --deep --strict", macos)
        self.assertIn("duckdb.dll", windows)

    def test_active_exception_registry_and_audit_ignores_match_governed_set(self) -> None:
        registry = json.loads((ROOT / "docs/roadmap/security-advisory-exceptions.json").read_text())
        audit = tomllib.loads((ROOT / ".cargo/audit.toml").read_text())
        registry_ids = {entry["id"] for entry in registry}
        audit_ids = set(audit["advisories"]["ignore"])
        self.assertTrue(registry_ids)
        self.assertEqual(registry_ids, audit_ids)


if __name__ == "__main__":
    unittest.main()
