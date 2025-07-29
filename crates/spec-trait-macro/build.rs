use std::fs;
use std::path::Path;

include!("src/cache.rs");

fn main() {
    let dest_path = Path::new(&FOLDER_CACHE).join(&FILE_CACHE);

    fs::write(&dest_path, "{}").expect("Failed to write file cache");

    println!("cargo::rerun-if-changed=build.rs");
}
