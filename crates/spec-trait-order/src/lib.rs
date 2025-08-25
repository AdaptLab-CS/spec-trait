mod crates;
mod files;

use chrono::Local;
use spec_trait_utils::cache;
use std::path::Path;
use spec_trait_utils::env::get_cache_path;

/// It is assumed to be used in `build.rs` or similar context.
pub fn handle_order() {
    println!("cargo:warning=Running spec-trait-order/build.rs at {}", Local::now().to_rfc3339());
    println!("cargo:rerun-if-changed={}", get_cache_path().to_string_lossy());
    println!("cargo:rerun-if-changed=."); // TODO: remove after development

    let dir = Path::new(".");
    let crates = crates::get_crates(&dir);

    cache::reset();
    for crate_ in crates {
        cache::add_crate(&crate_.name, crate_.content);
    }
}
