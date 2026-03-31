[github-ci]: https://github.com/AdaptLab-CS/spec-traits/actions/workflows/ci.yml
[github-ci-shield]: https://github.com/AdaptLab-CS/spec-trait/actions/workflows/ci.yml/badge.svg

# Meta-monomorphizing Specializations

[![GitHub CI][github-ci-shield]][github-ci]

This repository contains the artifact for the paper "Meta-monomorphizing Specializations".
The artifact consists of three main components: a prototype implementation, an instrumentation tool for empirical analysis, and a benchmark evaluation suite.

---

## Table of Contents

- [Overview](#overview)
- [Repository Structure](#repository-structure)
- [Requirements](#requirements)
- [Getting Started](#getting-started)
  - [Prototype Implementation (`spec-trait-impl`)](#prototype-implementation-spec-trait-impl)
  - [Instrumentation Tool (`spec-trait-inst`)](#instrumentation-tool-spec-trait-inst)
  - [Evaluation Benchmarks (`spec-trait-evaluation`)](#evaluation-benchmarks-spec-trait-evaluation)
- [Reproducing the Empirical Evaluation](#reproducing-the-empirical-evaluation)
- [Pre-computed Results](#pre-computed-results)
- [Contact](#contact)

---

## Overview

This artifact provides:

1. **`spec-trait-impl/`** — A prototype implementation of trait specialization in Rust, featuring procedural macros and compile-time trait ordering.

2. **`spec-trait-inst/`** — An instrumentation tool built as a `cargo`/`rustc` wrapper that statically analyzes Rust projects to identify trait specialization opportunities.

3. **`spec-trait-evaluation/`** — A benchmark suite that compares `spec!`/`#[when]` compile-time specialization against alternative dispatch strategies (naive, enum, runtime `TypeId`), measuring runtime performance, build time, and binary size. See the [evaluation README](spec-trait-evaluation/README.md) for details.

---

## Repository Structure

```
spec-trait/
├── spec-trait-impl/              # Prototype implementation
│   └── crates/
│       ├── spec-trait-bin/       # Binary crate (entry point)
│       ├── spec-trait-macro/     # Procedural macros for specialization
│       ├── spec-trait-order/     # Compile-time trait ordering
│       └── spec-trait-utils/     # Shared utilities
│
├── spec-trait-inst/              # Instrumentation tool
│   ├── src/                      # Source code
│   └── tests/
│
└── spec-trait-evaluation/        # Benchmark evaluation suite
    ├── benches-crate/            # Criterion benchmarks
    ├── notebooks/                # Analysis notebooks
    ├── results/                  # Pre-computed CSV results
    └── scripts/                  # Metric collection scripts
        ├── scripts/              # Analysis scripts and results
        │   ├── analysis.sh       # Main analysis script
        │   ├── repos.txt         # List of repositories analyzed
        │   ├── summary-90.csv    # Pre-computed results (90th percentile)
        │   └── summary-99.csv    # Pre-computed results (99th percentile)
        ├── snippets/             # Test snippets
        └── workspaces/           # Test workspaces
```

---

## Requirements

- **Rust nightly toolchain** (`nightly-2026-01-01`)
- **Git** (for cloning repositories during analysis)
- **Bash** (for running the analysis script)

---

## Getting Started

### Prototype Implementation (`spec-trait-impl`)

The prototype implementation showcases our approach to trait specialization through procedural macros.

#### Build

```bash
cd spec-trait-impl
cargo build
```

#### Run Tests

```bash
cd spec-trait-impl
cargo test
```

---

### Instrumentation Tool (`spec-trait-inst`)

The instrumentation tool analyzes Rust projects to identify existing and potential trait specialization patterns.

#### Setup

First, install the required nightly toolchain with all necessary components:

```bash
rustup toolchain install nightly-2026-01-01
rustup component add --toolchain nightly-2026-01-01 \
    rust-src rustc-dev llvm-tools-preview miri rust-analyzer clippy
```

#### Build

```bash
cd spec-trait-inst
cargo build
```

#### Run Tests

Execute the test suite on all example workspaces:

```bash
cd spec-trait-inst
cargo test --no-fail-fast -- --test-threads=1 --nocapture
```

#### Running on a Single Project

To analyze a specific Rust project:

```bash
cd /path/to/target/project

# On Linux:
LD_LIBRARY_PATH=$(rustc --print sysroot)/lib \
    /path/to/spec-trait-inst/target/debug/cargo-spec-trait-inst

# On macOS:
DYLD_LIBRARY_PATH=$(rustc --print sysroot)/lib \
    /path/to/spec-trait-inst/target/debug/cargo-spec-trait-inst
```

> **Note:** Additional logging can be enabled by setting `RUST_LOG=debug` or `RUST_LOG=trace`. Use `RUST_LOG_STYLE=always` to force colored output.

---

## Reproducing the Empirical Evaluation

The empirical evaluation from the paper can be fully reproduced using the provided analysis script. This script clones repositories from a curated list, runs the instrumentation tool on each, and aggregates the results.

### Running the Analysis

```bash
cd spec-trait-inst/tests
./scripts/analysis.sh
```

### Script Options

```
Usage: analysis.sh [-f <file>] [-w <workdir>] [-l <logs>] [-k] [-c] [-h]
  -f  File with newline-separated repo URLs (default: ./scripts/repos.txt)
  -w  Working directory for cloned repos (default: ./workspaces)
  -l  Logs directory (default: ./logs)
  -c  Continue mode: append to existing summary file
  -k  Keep cloned repositories after analysis
  -h  Show help message
```

### Example: Analyze a Custom Set of Repositories

```bash
cd spec-trait-inst/tests
./scripts/analysis.sh -f /path/to/custom-repos.txt -k
```

---

## Pre-computed Results

For convenience, we provide pre-computed analysis results in the `spec-trait-inst/tests/scripts/` directory:

| File             | Description                                                                                |
| ---------------- | ------------------------------------------------------------------------------------------ |
| `summary-90.csv` | Analysis results with 90th percentile threshold                                            |
| `summary-99.csv` | Analysis results with 99th percentile threshold                                            |
| `repos.txt`      | List of analyzed repositories (top Rust crates from crates.io and popular GitHub projects) |

These results were generated by running `analysis.sh` on the repositories listed in `repos.txt`. The CSV files contain detailed metrics including:

- Repository metadata (crate name, downloads, stars)
- Function and trait counts
- Specialization opportunity metrics (existing and potential)
- Analysis performance data (time, memory usage)


