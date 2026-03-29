#!/usr/bin/env python3
"""Collect Criterion benchmark results into a single CSV.

Walks the Criterion output directory and extracts timing estimates
from each benchmark's estimates.json files.

Usage:
    python3 collect_metrics.py --criterion-dir <path> --output <csv>

Output CSV columns:
    example              - example identifier (e.g. "ex01")
    operation            - operation name (e.g. "extend")
    group                - full Criterion group name (e.g. "ex01_extend")
    approach             - dispatch variant (spec, naive, enum, runtime_typeid)
    size                 - input size parameter (e.g. 128, 1024)
    mean_ns              - mean time per iteration (ns)
    mean_se_ns           - standard error of the mean (ns)
    mean_ci_lower_ns     - 95% confidence interval lower bound for mean (ns)
    mean_ci_upper_ns     - 95% confidence interval upper bound for mean (ns)
    median_ns            - median time per iteration (ns)
    median_se_ns         - standard error of the median (ns)
    median_ci_lower_ns   - 95% confidence interval lower bound for median (ns)
    median_ci_upper_ns   - 95% confidence interval upper bound for median (ns)
    slope_ns             - linear-regression slope estimate (ns/iter, most robust)
    slope_se_ns          - standard error of the slope (ns)
    slope_ci_lower_ns    - 95% confidence interval lower bound for slope (ns)
    slope_ci_upper_ns    - 95% confidence interval upper bound for slope (ns)
    std_dev_ns           - standard deviation of iteration times (ns)
    median_abs_dev_ns    - median absolute deviation, robust dispersion measure (ns)
"""
import argparse
import json
import csv
import re
from pathlib import Path


def collect(criterion_dir: Path, name_filter: str = ""):
    rows = []
    if not criterion_dir.exists():
        print(f"[WARN] Criterion directory not found: {criterion_dir}")
        return rows

    for group_dir in sorted(criterion_dir.iterdir()):
        if not group_dir.is_dir() or group_dir.name.startswith("."):
            continue
        group_name = group_dir.name

        if name_filter and name_filter not in group_name:
            continue

        for estimates_file in sorted(group_dir.rglob("new/estimates.json")):
            rel = estimates_file.relative_to(group_dir)
            parts = rel.parts

            if len(parts) < 3:
                continue

            if len(parts) >= 4:
                approach = parts[0]
                size = parts[1]
            else:
                approach = parts[0]
                size = ""

            with open(estimates_file, "r") as f:
                data = json.load(f)

            def _get(block, key):
                return data.get(block, {}).get(key, None)

            def _ci(block, bound):
                return data.get(block, {}).get("confidence_interval", {}).get(bound, None)

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
                "mean_ns": _get("mean", "point_estimate"),
                "mean_se_ns": _get("mean", "standard_error"),
                "mean_ci_lower_ns": _ci("mean", "lower_bound"),
                "mean_ci_upper_ns": _ci("mean", "upper_bound"),
                "median_ns": _get("median", "point_estimate"),
                "median_se_ns": _get("median", "standard_error"),
                "median_ci_lower_ns": _ci("median", "lower_bound"),
                "median_ci_upper_ns": _ci("median", "upper_bound"),
                "slope_ns": _get("slope", "point_estimate"),
                "slope_se_ns": _get("slope", "standard_error"),
                "slope_ci_lower_ns": _ci("slope", "lower_bound"),
                "slope_ci_upper_ns": _ci("slope", "upper_bound"),
                "std_dev_ns": _get("std_dev", "point_estimate"),
                "median_abs_dev_ns": _get("median_abs_dev", "point_estimate"),
            })
    return rows


def main():
    parser = argparse.ArgumentParser(description="Collect Criterion results")
    parser.add_argument("--criterion-dir", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--filter", default="",
                        help="Only include groups whose name contains this substring")
    args = parser.parse_args()

    rows = collect(args.criterion_dir, name_filter=args.filter)
    if not rows:
        print("[WARN] No results found.")
        return

    fieldnames = ["example", "operation", "group", "approach", "size",
                  "mean_ns", "mean_se_ns", "mean_ci_lower_ns", "mean_ci_upper_ns",
                  "median_ns", "median_se_ns", "median_ci_lower_ns", "median_ci_upper_ns",
                  "slope_ns", "slope_se_ns", "slope_ci_lower_ns", "slope_ci_upper_ns",
                  "std_dev_ns", "median_abs_dev_ns"]
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with open(args.output, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)

    print(f"[INFO] Wrote {len(rows)} rows to {args.output}")


if __name__ == "__main__":
    main()
