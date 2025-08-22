mod env;
mod file;

use chrono::Local;
use std::fs;
use std::path::Path;

/// It is assumed to be used in `build.rs` or similar context.
pub fn handle_order() {
    println!("cargo:warning=Running spec-trait-order/build.rs at {}", Local::now().to_rfc3339());
    println!("cargo:rerun-if-changed={}", env::get_cache_path().to_string_lossy());
    println!("cargo:rerun-if-changed=.."); // TODO: remove after development

    let rs_files = file::get_rs_files(Path::new("."));
    println!("cargo:warning=Found {} .rs files", rs_files.len());

    let file_items = rs_files.iter().map(AsRef::as_ref).flat_map(file::parse).collect::<Vec<_>>();
    println!("cargo:warning=Found {} items in .rs files", file_items.len());

    fs::write(env::get_cache_path(), "{}").expect("Failed to write file cache");
    // Qui facciamo un dump su file di ciò che abbiamo collezionato. Something like:
    // ```
    // {
    //  crate1: {
    //   specializable: [ ... ]
    //   default_and_when: [ ... ] // Consideriamo di dividerli
    //   spec!: [ ... ] // For fl-macro, probably not needed
    //  },
    //  crate2: { ... }
    // }
    // ```
}
