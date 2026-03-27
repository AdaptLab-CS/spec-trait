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

        # Criterion layout is typically: group/approach/size/new/estimates.json
        # Keep compatibility with older layout: group/approach/new/estimates.json
        for estimates_file in sorted(group_dir.rglob("new/estimates.json")):
            rel = estimates_file.relative_to(group_dir)
            parts = rel.parts

            if len(parts) < 3:
                continue

            if len(parts) >= 4:
                # approach/size/new/estimates.json
                approach = parts[0]
                size = parts[1]
            else:
                # approach/new/estimates.json
                approach = parts[0]
                size = ""

            with open(estimates_file, "r") as f:
                data = json.load(f)

            # Extract median point estimate (nanoseconds)
            median_ns = data.get("median", {}).get("point_estimate", None)
            mean_ns = data.get("mean", {}).get("point_estimate", None)
            std_dev = data.get("mean", {}).get("standard_error", None)

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
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with open(args.output, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)

    print(f"[INFO] Wrote {len(rows)} rows to {args.output}")


if __name__ == "__main__":
    main()
