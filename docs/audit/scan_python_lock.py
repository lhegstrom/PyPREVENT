"""Audit every pinned variant in a uv requirements lock, including other platforms.

This queries PyPI version advisories without installing packages. Use pip-audit
as well to validate the resolved/installed environment. Exit nonzero on findings
or failed queries; do not interpret a partial query as a clean audit.
"""

import argparse
import json
import re
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Any

# ============================================================
# Audit Settings
# ============================================================
PYPI_RELEASE_URL_TEMPLATE = "https://pypi.org/pypi/{name}/{version}/json"
REQUEST_TIMEOUT_SECONDS = 60
PARALLEL_REQUEST_COUNT = 8
PINNED_REQUIREMENT_PATTERN = re.compile(r"^([\w.-]+)==([^\s;\\]+)", re.MULTILINE)


def fetch_advisories_for_pinned_package(
    pinned_package: tuple[str, str],
) -> dict[str, Any]:
    """Return the non-withdrawn PyPI advisories for one package version."""
    name, version = pinned_package
    with urllib.request.urlopen(
        PYPI_RELEASE_URL_TEMPLATE.format(name=name, version=version),
        timeout=REQUEST_TIMEOUT_SECONDS,
    ) as response:
        release_record = json.load(response)
    return {
        "name": name,
        "version": version,
        "vulns": [
            advisory
            for advisory in release_record.get("vulnerabilities", [])
            if not advisory.get("withdrawn")
        ],
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--lock", type=Path, default=Path("requirements-audit.txt"))
    parser.add_argument("--output", type=Path, required=True)
    arguments = parser.parse_args()
    pinned_packages = sorted(
        set(PINNED_REQUIREMENT_PATTERN.findall(arguments.lock.read_text()))
    )
    if not pinned_packages:
        raise SystemExit("No pinned package versions found")
    with ThreadPoolExecutor(max_workers=PARALLEL_REQUEST_COUNT) as pool:
        advisory_records = list(
            pool.map(fetch_advisories_for_pinned_package, pinned_packages)
        )
    arguments.output.write_text(
        json.dumps({"dependencies": advisory_records}, indent=2) + "\n"
    )
    affected_records = [record for record in advisory_records if record["vulns"]]
    print(
        f"Checked {len(advisory_records)} package/version pairs across all lockfile markers;"
        f" {len(affected_records)} affected"
    )
    raise SystemExit(bool(affected_records))


if __name__ == "__main__":
    main()
