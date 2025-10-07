[github-ci]: https://github.com/FedericoBruzzone/rusty-links/actions/workflows/ci.yml
[github-ci-shield]: https://github.com/FedericoBruzzone/rusty-links/actions/workflows/ci.yml/badge.svg

# Trait Specialization Instrumentation

[![GitHub CI][github-ci-shield]][github-ci]

> [!NOTE] Add description here.

## Installation and Usage

Note that for macOS, the `LD_LIBRARY_PATH` should be replaced with `DYLD_LIBRARY_PATH`.

### Setup

Setup the nightly toolchain:

```bash
rustup toolchain install nightly-2025-10-06
rustup component add --toolchain nightly-2025-10-06 rust-src rustc-dev llvm-tools-preview miri rust-analyzer clippy
```

### Test

Run tests on all example workspaces:

```bash
cargo test --no-fail-fast -- --test-threads=1 --nocapture
```

### Fast Test

```bash
cd tests/workspaces/first
clear && cargo clean && cargo build --manifest-path ../../../Cargo.toml && RUST_LOG_STYLE=always RUST_LOG=trace LD_LIBRARY_PATH=$(rustc --print sysroot)/lib ../../../target/debug/cargo-spec-trait-inst
```

### Cli (`cargo` wrapper)

> ℹ️  Additional logs can be enabled by setting the `RUST_LOG` environment variable to `debug`.

> ℹ️  The `RUST_LOG_STYLE` environment variable can be set to `always` to force the logs to be colored.

```bash
cd tests/workspaces/first
cargo run --manifest-path ../../../Cargo.toml --bin cargo-spec-trait-insts [--CARGO_ARG] -- [--PLUGIN_ARG]
```

or

```
cd tests/workspaces/first
LD_LIBRARY_PATH=$(rustc --print sysroot)/lib ../../../target/debug/cargo-spec-trait-insts [--PLUGIN_ARG] -- [--CARGO_ARG]
```

### Driver (`rustc` wrapper)

> ⚠️  It is not currently possible to pass the plugin args to the driver without using an environment variable. Using the CLI is advised.

TODO: Find a way to pass to the driver the plugin args using "PLUGIN_ARGS" environment variable

```bash
CARGO_PRIMARY_PACKAGE=1 cargo run --bin spec-trait-inst-driver -- ./tests/workspaces/first/src/main.rs [--RUSTC_ARG (e.g., --cfg 'feature="test"')]
```

or

```bash
cd tests/workspaces/first
CARGO_PRIMARY_PACKAGE=1 cargo run --manifest-path ../../Cargo.toml --bin spec-trait-insts-driver -- ./src/main.rs
```

## Usage in the Rust compiler

### `rustc` setup

```shell
cd tests
git clone git@github.com:rust-lang/rust.git --depth 1
cd rust
./x setup
./x build --stage 0
./x build --stage 1
./x build --stage 2 # Implies compilation of stage1's stdlib
```

### Setup

Set in `rust-toolchain` the `channel=stage1`.

```shell
cd ../..
cargo clean
cargo build
```

### Driver (`rustc` wrapper)

```shell
cd tests/rust
rm -rf target
RUSTC_BOOTSTRAP=1 CARGO_PRIMARY_PACKAGE=1 RUST_LOG_STYLE=always RUST_LOG=trace LD_LIBRARY_PATH=PATH/TO/spec-trait-inst/tests/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib ../../target/debug/cargo-spec-trait-inst --print-mir --filter-with-file "compiler/rustc/src/main.rs"
```

## Contact

If you have any questions, suggestions, or feedback, do not hesitate to [contact me](https://federicobruzzone.github.io/).
