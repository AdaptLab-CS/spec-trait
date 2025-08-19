use std::fs;
use chrono::Local;
include!("src/env.rs");

fn main() {
    fs::write(&get_cache_path(), "{}").expect("Failed to write file cache");

    println!("cargo:warning=Running build.rs at {}", Local::now().to_rfc3339());
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed={}", get_cache_path().to_string_lossy());
}
