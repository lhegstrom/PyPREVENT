"""Query OSV for every crates.io package pinned in Cargo.lock and save full records.

Example: python docs/audit/scan_rust_osv.py --output /tmp/pyprevent-rust-audit.json
"""

import argparse
import json
import urllib.request
from pathlib import Path
from typing import Any

import tomllib

# ============================================================
# OSV Settings
# ============================================================
OSV_BATCH_QUERY_URL = "https://api.osv.dev/v1/querybatch"
OSV_VULNERABILITY_URL_TEMPLATE = "https://api.osv.dev/v1/vulns/{vulnerability_id}"
REQUEST_TIMEOUT_SECONDS = 60


def load_registry_packages_from_lock(lock_path: Path) -> list[dict[str, Any]]:
    """Return Cargo.lock packages that come from a registry, not local paths."""
    lock_packages = tomllib.loads(lock_path.read_text())["package"]
    return [
        package
        for package in lock_packages
        if package.get("source", "").startswith("registry+")
    ]


def fetch_json_from_url(request: urllib.request.Request | str) -> Any:
    with urllib.request.urlopen(request, timeout=REQUEST_TIMEOUT_SECONDS) as response:
        return json.load(response)


def query_vulnerabilities_for_packages(
    registry_packages: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    """Batch-query OSV, then fetch the full record for every match."""
    query_body = {
        "queries": [
            {
                "package": {"name": package["name"], "ecosystem": "crates.io"},
                "version": package["version"],
            }
            for package in registry_packages
        ]
    }
    batch_request = urllib.request.Request(
        OSV_BATCH_QUERY_URL,
        data=json.dumps(query_body).encode(),
        headers={"Content-Type": "application/json"},
    )
    batch_results = fetch_json_from_url(request=batch_request)["results"]
    return [
        {
            "name": package["name"],
            "version": package["version"],
            "vulns": [
                fetch_json_from_url(
                    request=OSV_VULNERABILITY_URL_TEMPLATE.format(
                        vulnerability_id=vulnerability["id"]
                    )
                )
                for vulnerability in result.get("vulns", [])
            ],
        }
        for package, result in zip(registry_packages, batch_results)
    ]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--lock", type=Path, default=Path("Cargo.lock"))
    parser.add_argument("--output", type=Path, required=True)
    arguments = parser.parse_args()
    vulnerability_records = query_vulnerabilities_for_packages(
        registry_packages=load_registry_packages_from_lock(lock_path=arguments.lock)
    )
    arguments.output.write_text(json.dumps(vulnerability_records, indent=2))
    print("Scanned", len(vulnerability_records), "registry packages")
    for record in vulnerability_records:
        for vulnerability in record["vulns"]:
            print(
                record["name"],
                record["version"],
                vulnerability["id"],
                vulnerability.get("summary"),
                vulnerability.get("affected"),
            )


if __name__ == "__main__":
    main()
