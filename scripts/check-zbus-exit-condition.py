#!/usr/bin/env python3
"""Monitor the entry-point limb of the governed zbus exceptions' exit condition.

The registry at ``docs/roadmap/security-advisory-exceptions.json`` states the
exit condition for the governed trio in prose:

    "a notify-rust release declaring zbus ^6, and a ksni release declaring
    zbus ^6 that a tray-icon release can consume, or the native-ui features
    are reworked — whichever first"

The ``WATCH`` constant below is the machine-readable form of the
*entry-point limb* of that prose for the current trio: notify-rust on
zbus ^6 AND ksni on zbus ^6. This monitor checks only that limb against the
crates.io API. Whether a tray-icon release can actually consume a future
ksni release is a human judgment about an unreleased combination and is
NOT auto-verified here, so a firing verdict is worded "entry-point limb
met — human tray-icon consumability check required before retiring the
exceptions", never "exit condition met".

Known limitation: a future governed exception with a different exit
condition needs its own watch entry. To keep that from failing silently,
the monitor maps every registry entry's ``exit_condition`` before checking
anything: an entry whose exit condition it cannot map (it does not name
both watched entry points declaring the watched dependency at ^6) makes
the monitor fail loudly with outcome 2 (could-not-determine) rather than
silently not monitoring that entry.

Outcomes (exit codes):
    0  entry-point limb not met — at least one watched crate's newest
       stable release does not declare the watched dependency at ^6.
    1  entry-point limb met — every watched crate's newest stable release
       declares the watched dependency at ^6 (normal dependencies only).
    2  could-not-determine — HTTP error, malformed payload, missing or
       malformed registry, or an unmapped registry entry. Network failure
       is never reported as met or not-met.

crates.io API contract:
    GET https://crates.io/api/v1/crates/{crate}
        -> take ``crate.max_version``; fall back through the ``versions``
           list to the newest stable unyanked version when max_version is
           yanked or a pre-release.
    GET https://crates.io/api/v1/crates/{crate}/{version}/dependencies
        -> only dependencies with ``kind == "normal"`` count (ksni lists
           zbus as both a normal and a dev dependency; dev/build are
           excluded).
    A ``User-Agent`` header identifying this repository is mandatory on
    every live request (crates.io returns 403 without one).

Fixture injection (``--fixture-dir <path>``): recorded API responses, one
file per endpoint, so tests run with zero live network. Naming scheme,
relative to the fixture directory:
    ``{crate}.json``
        Body of ``GET /crates/{crate}``.
    ``{crate}--{version}--dependencies.json``
        Body of ``GET /crates/{crate}/{version}/dependencies``, where
        ``{version}`` is the version this script selected as the newest
        stable unyanked release.
    ``{crate}.status`` / ``{crate}--{version}--dependencies.status``
        Optional. When present, its integer content is the endpoint's
        HTTP status; any status other than 200 is treated exactly like
        the corresponding live HTTP error (outcome 2). The body file may
        be absent in that case.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path

# Machine-readable form of the registry's exit_condition entry-point limb
# for the current governed trio: (entry-point crate, watched dependency).
WATCH = [("notify-rust", "zbus"), ("ksni", "zbus")]

API_BASE = "https://crates.io/api/v1/crates"
USER_AGENT = (
    "rocinante-check-zbus-exit-condition "
    "(https://github.com/pratik-saptarshi/rocinante)"
)
# The watched dependency must be declared at this major version (^N).
WATCHED_MAJOR = 6

LIMB_MET_MESSAGE = (
    "entry-point limb met — human tray-icon consumability check required "
    "before retiring the exceptions"
)


class CouldNotDetermine(Exception):
    """Raised for any condition that maps to outcome 2."""


# --------------------------------------------------------------------------
# Registry mapping (loud failure on unmapped entries)
# --------------------------------------------------------------------------

def load_registry(registry_path: Path) -> list[dict[str, object]]:
    try:
        payload = json.loads(registry_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise CouldNotDetermine(
            f"could not read exception registry {registry_path}: {error}"
        ) from error
    if not isinstance(payload, list) or not payload:
        raise CouldNotDetermine("exception registry is not a non-empty JSON array")
    for index, entry in enumerate(payload):
        if not isinstance(entry, dict) or not isinstance(entry.get("exit_condition"), str):
            raise CouldNotDetermine(f"registry entry {index} has no exit_condition text")
    return payload


def check_registry_mapping(entries: list[dict[str, object]]) -> None:
    """Fail loudly if any registry exit condition cannot be mapped to WATCH.

    The mappable form names both watched entry-point crates and the watched
    dependency at ^6. Anything else (a different crate pair, a different
    dependency, a different major) is a condition this monitor does not
    watch, and silently skipping it would be a false "not met".
    """
    watched_crates = {crate for crate, _dep in WATCH}
    watched_deps = {dep for _crate, dep in WATCH}
    for entry in entries:
        condition = str(entry["exit_condition"])
        names_crates = all(crate in condition for crate in watched_crates)
        names_dep_at_major = any(
            f"{dep} ^{WATCHED_MAJOR}" in condition for dep in watched_deps
        )
        if not (names_crates and names_dep_at_major):
            raise CouldNotDetermine(
                "unmapped registry entry: "
                f"{entry.get('id', '<no id>')} has an exit_condition this "
                f"monitor cannot map to WATCH={WATCH!r}: {condition!r}. "
                "Add a watch entry for it instead of silently not "
                "monitoring it."
            )


# --------------------------------------------------------------------------
# API access (live or fixture)
# --------------------------------------------------------------------------

def fetch_json(url_path: str, fixture_dir: Path | None, fixture_stem: str) -> object:
    """Return the parsed JSON body for one endpoint, or raise CouldNotDetermine."""
    if fixture_dir is not None:
        status_path = fixture_dir / f"{fixture_stem}.status"
        if status_path.exists():
            try:
                status = int(status_path.read_text(encoding="utf-8").strip())
            except (OSError, ValueError) as error:
                raise CouldNotDetermine(
                    f"malformed fixture status file {status_path}: {error}"
                ) from error
            if status != 200:
                raise CouldNotDetermine(
                    f"HTTP {status} from crates.io endpoint {url_path} (fixture)"
                )
        body_path = fixture_dir / f"{fixture_stem}.json"
        try:
            raw = body_path.read_text(encoding="utf-8")
        except OSError as error:
            raise CouldNotDetermine(
                f"missing fixture response file {body_path}: {error}"
            ) from error
    else:
        request = urllib.request.Request(
            f"{API_BASE}/{url_path}",
            headers={"User-Agent": USER_AGENT, "Accept": "application/json"},
        )
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                raw = response.read().decode("utf-8")
        except urllib.error.HTTPError as error:
            raise CouldNotDetermine(
                f"HTTP {error.code} from crates.io endpoint {url_path}"
            ) from error
        except (urllib.error.URLError, OSError, UnicodeDecodeError) as error:
            raise CouldNotDetermine(
                f"could not fetch crates.io endpoint {url_path}: {error}"
            ) from error
    try:
        return json.loads(raw)
    except json.JSONDecodeError as error:
        raise CouldNotDetermine(
            f"malformed JSON from crates.io endpoint {url_path}: {error}"
        ) from error


# --------------------------------------------------------------------------
# Version selection and dependency evaluation
# --------------------------------------------------------------------------

def parse_version(text: str) -> tuple[int, ...] | None:
    parts = text.split(".")
    numbers = []
    for part in parts:
        if not part.isdigit():
            return None
        numbers.append(int(part))
    return tuple(numbers) if numbers else None


def is_stable(version: str) -> bool:
    """A stable release has no pre-release suffix (and a numeric core)."""
    return "-" not in version and parse_version(version) is not None


def select_newest_stable(crate_payload: object, crate: str) -> str:
    if not isinstance(crate_payload, dict) or not isinstance(crate_payload.get("crate"), dict):
        raise CouldNotDetermine(f"malformed crate payload for {crate}: no 'crate' object")
    max_version = crate_payload["crate"].get("max_version")
    if not isinstance(max_version, str) or not max_version:
        raise CouldNotDetermine(f"malformed crate payload for {crate}: no crate.max_version")

    versions = crate_payload.get("versions")
    yanked: dict[str, bool] = {}
    if isinstance(versions, list):
        for item in versions:
            if isinstance(item, dict) and isinstance(item.get("num"), str):
                yanked[item["num"]] = bool(item.get("yanked", False))

    # Fast path: max_version is itself the newest stable unyanked release.
    if is_stable(max_version) and not yanked.get(max_version, False):
        return max_version

    # Fallback: newest stable unyanked entry in the versions list.
    candidates = [
        num for num, is_yanked in yanked.items() if is_stable(num) and not is_yanked
    ]
    if not candidates:
        raise CouldNotDetermine(
            f"no stable unyanked version found for {crate} "
            f"(max_version={max_version!r})"
        )
    candidates.sort(key=lambda num: parse_version(num) or ())
    return candidates[-1]


def requirement_is_watched_major(req: object) -> bool:
    """True when a cargo requirement is a ^6-style requirement on major 6.

    Handles the shapes crates.io returns for the watched crates: ``^6``,
    ``^6.0``, ``6``, ``6.0`` (a bare version is a caret requirement in
    cargo semantics). Anything else (ranges, wildcards, other majors)
    does not by itself declare the dependency at ^6.
    """
    if not isinstance(req, str):
        return False
    text = req.strip()
    if text.startswith("^"):
        text = text[1:].strip()
    match = re.match(r"(\d+)(?:\.|$)", text)
    return bool(match) and int(match.group(1)) == WATCHED_MAJOR


def crate_declares_watched_major(
    crate: str, dep: str, fixture_dir: Path | None
) -> tuple[bool, str, str]:
    """Return (declares ^6, selected version, selected requirement or '')."""
    payload = fetch_json(crate, fixture_dir, crate)
    version = select_newest_stable(payload, crate)
    deps_payload = fetch_json(
        f"{crate}/{version}/dependencies",
        fixture_dir,
        f"{crate}--{version}--dependencies",
    )
    if not isinstance(deps_payload, dict) or not isinstance(
        deps_payload.get("dependencies"), list
    ):
        raise CouldNotDetermine(
            f"malformed dependencies payload for {crate} {version}"
        )
    for item in deps_payload["dependencies"]:
        if not isinstance(item, dict):
            continue
        if item.get("crate_id") != dep or item.get("kind") != "normal":
            continue
        req = item.get("req")
        return requirement_is_watched_major(req), version, str(req)
    return False, version, ""


# --------------------------------------------------------------------------
# Main
# --------------------------------------------------------------------------

def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--registry",
        type=Path,
        default=repo_root / "docs/roadmap/security-advisory-exceptions.json",
        help="exception registry whose exit conditions are mapped to WATCH",
    )
    parser.add_argument(
        "--fixture-dir",
        type=Path,
        default=None,
        help="directory of recorded crates.io responses (see module docstring "
        "for the naming scheme); when given, no live network is used",
    )
    args = parser.parse_args()

    try:
        entries = load_registry(args.registry)
        check_registry_mapping(entries)
        results = []
        for crate, dep in WATCH:
            declares, version, req = crate_declares_watched_major(
                crate, dep, args.fixture_dir
            )
            results.append((crate, dep, declares, version, req))
    except CouldNotDetermine as error:
        print(f"could-not-determine: {error}", file=sys.stderr)
        return 2

    for crate, dep, declares, version, req in results:
        state = f"declares {dep} {req!r}" if req else f"does not declare {dep}"
        print(f"{crate} {version}: {state} (normal dependencies only)")

    if all(declares for _c, _d, declares, _v, _r in results):
        print(LIMB_MET_MESSAGE)
        return 1

    unmet = [crate for crate, _d, declares, _v, _r in results if not declares]
    print(
        "entry-point limb not met: not every watched crate declares the "
        f"watched dependency at ^{WATCHED_MAJOR} in its newest stable "
        f"release (unmet: {', '.join(unmet)})."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
