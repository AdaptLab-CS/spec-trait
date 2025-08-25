mod crates;
mod files;

use spec_trait_utils::cache;
use std::path::Path;
use spec_trait_utils::env::get_cache_path;

/// It is assumed to be used in `build.rs` or similar context.
pub fn handle_order() {
    println!("cargo:rerun-if-changed={}", get_cache_path().display());
    println!("cargo:rerun-if-changed=.");

    cache::reset();

    crates
        ::get_crates(Path::new("."))
        .into_iter()
        .for_each(|crate_| {
            cache::add_crate(&crate_.name, crate_.content);
        });
}
