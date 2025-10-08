mod aliases;
mod crates;
mod files;

use spec_trait_utils::cache;
use spec_trait_utils::env::get_cache_path;
use std::path::Path;

/// It is assumed to be used in `build.rs` or similar context.
pub fn handle_order() {
    println!("cargo:rerun-if-changed={}", get_cache_path().display());
    println!("cargo:rerun-if-changed=.");

    cache::reset();

    crates::get_crates(Path::new("."))
        .into_iter()
        .for_each(|crate_| {
            cache::add_crate(&crate_.name, crate_.content);
        });
}
