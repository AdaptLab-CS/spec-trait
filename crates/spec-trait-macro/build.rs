use std::fs;
include!("src/env.rs");

fn main() {
    fs::write(&get_cache_path(), "{}").expect("Failed to write file cache");

    println!("cargo::rerun-if-changed=");
}
