#!/usr/bin/env python3
"""Fixture tests for scripts/check-zbus-exit-condition.py (stdlib only, no network).

Each test records fake crates.io API responses into a temporary fixture
directory and runs the monitor as a subprocess with ``--fixture-dir``.
Fixture naming scheme (one file per endpoint, documented in the monitor's
module docstring):

    {crate}.json
        Body of GET /crates/{crate}.
    {crate}--{version}--dependencies.json
        Body of GET /crates/{crate}/{version}/dependencies for the version
        the monitor is expected to select (newest stable unyanked).
    {crate}.status / {crate}--{version}--dependencies.status
        Optional HTTP status override; a non-200 status is treated exactly
        like the corresponding live HTTP error.
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True

SCRIPT = Path(__file__).with_name("check-zbus-exit-condition.py")
REPO_ROOT = Path(__file__).resolve().parents[1]

LIMB_MET_WORDING = (
    "entry-point limb met — human tray-icon consumability check required "
    "before retiring the exceptions"
)


def crate_payload(max_version: str, versions: list[tuple[str, bool]]) -> dict:
    return {
        "crate": {"id": "fixture", "max_version": max_version},
        "versions": [{"num": num, "yanked": yanked} for num, yanked in versions],
    }


def deps_payload(entries: list[tuple[str, str, str]]) -> dict:
    return {
        "dependencies": [
            {"crate_id": crate_id, "req": req, "kind": kind}
            for crate_id, req, kind in entries
        ]
    }


def crate_files(crate, max_version, versions, selected_version, entries):
    return {
        f"{crate}.json": crate_payload(max_version, versions),
        f"{crate}--{selected_version}--dependencies.json": deps_payload(entries),
    }


ZB5 = [("zbus", "^5", "normal")]
ZB6 = [("zbus", "^6", "normal")]


class ZbusExitConditionFixtureTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.fixture_dir = Path(self.tmp.name)

    def tearDown(self) -> None:
        self.tmp.cleanup()

    def write_fixtures(self, files: dict[str, object]) -> None:
        for name, content in files.items():
            path = self.fixture_dir / name
            if isinstance(content, str):
                path.write_text(content, encoding="utf-8")
            else:
                path.write_text(json.dumps(content), encoding="utf-8")

    def run_monitor(self, *extra_args: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [
                sys.executable,
                str(SCRIPT),
                "--fixture-dir",
                str(self.fixture_dir),
                *extra_args,
            ],
            stdout=subprocess.PIPE,
            text=True,
            stderr=subprocess.STDOUT,
            timeout=30,
            check=False,
        )

    def both(self, notify_entries, ksni_entries,
             notify=("4.18.2", [("4.18.2", False)], "4.18.2"),
             ksni=("0.3.7", [("0.3.7", False)], "0.3.7")) -> None:
        files: dict[str, object] = {}
        files.update(crate_files("notify-rust", notify[0], notify[1], notify[2], notify_entries))
        files.update(crate_files("ksni", ksni[0], ksni[1], ksni[2], ksni_entries))
        self.write_fixtures(files)

    # Case 1: both entry points on zbus ^5 -> limb not met (exit 0).
    # ksni additionally lists zbus ^6 as a *dev* dependency; only normal
    # dependencies count, so this must not fire the limb.
    def test_both_on_5_returns_0(self) -> None:
        self.both(ZB5, [("zbus", "^5", "normal"), ("zbus", "^6", "dev")])
        result = self.run_monitor()
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertIn("entry-point limb not met", result.stdout)

    # Case 2: both entry points on zbus ^6 -> limb met (exit 1), with the
    # honest wording that stops short of claiming consumability.
    def test_both_on_6_returns_1_with_limb_met_wording(self) -> None:
        self.both(
            ZB6, ZB6,
            notify=("4.19.0", [("4.19.0", False)], "4.19.0"),
            ksni=("0.4.0", [("0.4.0", False)], "0.4.0"),
        )
        result = self.run_monitor()
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn(LIMB_MET_WORDING, result.stdout)

    # Case 3: HTTP 403 from crates.io -> could-not-determine (exit 2).
    def test_http_403_returns_2(self) -> None:
        self.write_fixtures({"notify-rust.status": "403\n"})
        result = self.run_monitor()
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn("403", result.stdout)

    # Case 4: malformed JSON -> could-not-determine (exit 2).
    def test_malformed_json_returns_2(self) -> None:
        self.write_fixtures({"notify-rust.json": "{ this is not json"})
        result = self.run_monitor()
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn("malformed JSON", result.stdout)

    # Case 5: ksni's newest version is yanked; the monitor falls back to
    # the newest stable unyanked release (0.3.7, zbus ^5) -> exit 0.
    def test_ksni_newest_yanked_falls_back_to_stable_unyanked(self) -> None:
        self.both(
            ZB5, ZB5,
            ksni=("0.4.0", [("0.4.0", True), ("0.3.7", False), ("0.3.6", False)], "0.3.7"),
        )
        result = self.run_monitor()
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertIn("ksni 0.3.7", result.stdout)

    # Case 6: notify-rust declares zbus ^6 only on a pre-release; the
    # newest stable release (4.18.2) is on ^5 -> exit 0.
    def test_notify_6_only_on_prerelease_returns_0(self) -> None:
        self.both(
            ZB5, ZB5,
            notify=("5.0.0-beta.1", [("5.0.0-beta.1", False), ("4.18.2", False)], "4.18.2"),
        )
        result = self.run_monitor()
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertIn("notify-rust 4.18.2", result.stdout)

    # Case 7: a registry entry whose exit condition the monitor cannot map
    # must fail loudly (exit 2), never be silently unmonitored.
    def test_unmapped_registry_entry_returns_2(self) -> None:
        self.both(ZB5, ZB5)
        registry = [
            {
                "id": "RUSTSEC-9999-0001",
                "exit_condition": "a some-other-crate release declaring wbus ^9",
            }
        ]
        registry_path = self.fixture_dir / "unmapped-registry.json"
        registry_path.write_text(json.dumps(registry), encoding="utf-8")
        result = self.run_monitor("--registry", str(registry_path))
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn("unmapped registry entry", result.stdout)

    # Guard: the real repo registry must stay mappable by WATCH, so the
    # production (schedule) run does not degrade to a permanent warning.
    def test_repo_registry_is_mappable(self) -> None:
        self.both(ZB5, ZB5)
        result = self.run_monitor(
            "--registry",
            str(REPO_ROOT / "docs/roadmap/security-advisory-exceptions.json"),
        )
        self.assertEqual(result.returncode, 0, result.stdout)


if __name__ == "__main__":
    unittest.main()
