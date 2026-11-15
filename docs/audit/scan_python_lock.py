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


def query(package):
    name, version = package
    with urllib.request.urlopen(
        f"https://pypi.org/pypi/{name}/{version}/json", timeout=60
    ) as response:
        record = json.load(response)
    return {
        "name": name,
        "version": version,
        "vulns": [
            v for v in record.get("vulnerabilities", []) if not v.get("withdrawn")
        ],
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--lock", type=Path, default=Path("requirements-audit.txt"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    packages = sorted(
        set(re.findall(r"^([\w.-]+)==([^\s;\\]+)", args.lock.read_text(), re.MULTILINE))
    )
    if not packages:
        raise SystemExit("No pinned package versions found")
    with ThreadPoolExecutor(max_workers=8) as pool:
        records = list(pool.map(query, packages))
    args.output.write_text(json.dumps({"dependencies": records}, indent=2) + "\n")
    affected = [p for p in records if p["vulns"]]
    print(
        f"Checked {len(records)} package/version pairs across all lockfile markers; {len(affected)} affected"
    )
    raise SystemExit(bool(affected))
