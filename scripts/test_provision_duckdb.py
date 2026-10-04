from __future__ import annotations

import hashlib
import importlib.util
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path


SCRIPT = Path(__file__).with_name("provision_duckdb.py")
sys.dont_write_bytecode = True
SPEC = importlib.util.spec_from_file_location("provision_duckdb", SCRIPT)
assert SPEC and SPEC.loader
provisioner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(provisioner)


class DuckDbProvisioningTests(unittest.TestCase):
    def make_archive(self, folder: Path, contents: dict[str, bytes]) -> tuple[Path, dict[str, str]]:
        archive_path = folder / "duckdb-test.zip"
        with zipfile.ZipFile(archive_path, "w") as archive:
            for name, content in contents.items():
                archive.writestr(name, content)
        file_hashes = {name: hashlib.sha256(content).hexdigest() for name, content in contents.items()}
        return archive_path, file_hashes

    def make_manifest(self, archive: Path, file_hashes: dict[str, str]) -> dict[str, object]:
        return {
            "duckdb_rs_version": "1.10506.0",
            "duckdb_version": "1.5.6",
            "release_url": "https://example.invalid/duckdb",
            "targets": {
                "test-target": {
                    "archive": "duckdb-test.zip",
                    "archive_sha256": hashlib.sha256(archive.read_bytes()).hexdigest(),
                    "files": file_hashes,
                }
            },
        }

    def test_binding_and_engine_versions_must_match(self) -> None:
        with self.assertRaises(provisioner.ProvisionError):
            provisioner.validate_manifest_data(
                {"duckdb_rs_version": "1.10506.0", "duckdb_version": "1.5.5"}
            )

    def test_verified_archive_extracts_expected_prebuilt_files(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            folder = Path(temp_dir)
            archive, files = self.make_archive(folder, {"libduckdb.so": b"native", "duckdb.h": b"header"})
            target_dir = folder / "target"
            manifest = self.make_manifest(archive, files)

            result = provisioner.provision_target(
                manifest, "test-target", target_dir, archive_override=archive
            )

            self.assertEqual((result / "libduckdb.so").read_bytes(), b"native")
            self.assertEqual((result / "duckdb.h").read_bytes(), b"header")
            self.assertEqual(provisioner.sha256_file(result / "libduckdb.so"), files["libduckdb.so"])

    def test_corrupt_archive_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            folder = Path(temp_dir)
            archive, files = self.make_archive(folder, {"libduckdb.so": b"native"})
            manifest = self.make_manifest(archive, files)
            manifest["targets"]["test-target"]["archive_sha256"] = "0" * 64

            with self.assertRaisesRegex(provisioner.ProvisionError, "archive checksum mismatch"):
                provisioner.provision_target(
                    manifest, "test-target", folder / "target", archive_override=archive
                )

    def test_missing_expected_file_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            folder = Path(temp_dir)
            archive, files = self.make_archive(folder, {"duckdb.h": b"header"})
            files["libduckdb.so"] = hashlib.sha256(b"native").hexdigest()
            manifest = self.make_manifest(archive, files)

            with self.assertRaisesRegex(provisioner.ProvisionError, "missing expected file"):
                provisioner.provision_target(
                    manifest, "test-target", folder / "target", archive_override=archive
                )

    def test_tampered_extracted_library_is_replaced_from_verified_archive(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            folder = Path(temp_dir)
            archive, files = self.make_archive(folder, {"libduckdb.so": b"native"})
            manifest = self.make_manifest(archive, files)
            result = provisioner.provision_target(
                manifest, "test-target", folder / "target", archive_override=archive
            )
            (result / "libduckdb.so").write_bytes(b"tampered")

            provisioner.provision_target(manifest, "test-target", folder / "target")

            self.assertEqual((result / "libduckdb.so").read_bytes(), b"native")


if __name__ == "__main__":
    unittest.main()
