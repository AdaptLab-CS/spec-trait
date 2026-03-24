#!/usr/bin/env python3
"""Collect Criterion benchmark results into a single CSV.

Walks the Criterion output directory and extracts timing estimates
from each benchmark's estimates.json files.

Usage:
    python3 collect_metrics.py --criterion-dir <path> --output <csv>
"""
import argparse
import json
import csv
import re
from pathlib import Path


def parse_benchmark_id(group: str, variant_dir: str):
    """Parse group name and variant from directory structure."""
    return group, variant_dir


def collect(criterion_dir: Path):
    rows = []
    if not criterion_dir.exists():
        print(f"[WARN] Criterion directory not found: {criterion_dir}")
        return rows

    for group_dir in sorted(criterion_dir.iterdir()):
        if not group_dir.is_dir() or group_dir.name.startswith("."):
            continue
        group_name = group_dir.name  # e.g. "ex01_double_u8"

        for variant_dir in sorted(group_dir.iterdir()):
            if not variant_dir.is_dir():
                continue
            variant_name = variant_dir.name  # e.g. "spec/256"

            estimates_file = variant_dir / "new" / "estimates.json"
            if not estimates_file.exists():
                continue

            with open(estimates_file, "r") as f:
                data = json.load(f)

            # Extract median point estimate (nanoseconds)
            median_ns = data.get("median", {}).get("point_estimate", None)
            mean_ns = data.get("mean", {}).get("point_estimate", None)
            std_dev = data.get("mean", {}).get("standard_error", None)

            # Parse variant and size from variant_name
            parts = variant_name.split("/")
            if len(parts) == 2:
                approach, size = parts
            else:
                approach = variant_name
                size = ""

            # Extract example number from group name
            match = re.match(r"(ex\d+)_(.+)", group_name)
            if match:
                example = match.group(1)
                operation = match.group(2)
            else:
                example = group_name
                operation = ""

            rows.append({
                "example": example,
                "operation": operation,
                "group": group_name,
                "approach": approach,
                "size": size,
                "mean_ns": mean_ns,
                "median_ns": median_ns,
                "std_error_ns": std_dev,
            })
    return rows


def main():
    parser = argparse.ArgumentParser(description="Collect Criterion results")
    parser.add_argument("--criterion-dir", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    rows = collect(args.criterion_dir)
    if not rows:
        print("[WARN] No results found.")
        return

    fieldnames = ["example", "operation", "group", "approach", "size",
                  "mean_ns", "median_ns", "std_error_ns"]
    with open(args.output, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)

    print(f"[INFO] Wrote {len(rows)} rows to {args.output}")


if __name__ == "__main__":
    main()
