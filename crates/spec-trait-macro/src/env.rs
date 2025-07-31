use std::path::{Path, PathBuf};

pub const FOLDER_CACHE: &str = "/tmp";
pub const FILE_CACHE: &str = "spec_trait_macro_cache.json";

pub fn get_cache_path() -> PathBuf {
    Path::new(&FOLDER_CACHE).join(&FILE_CACHE)
}
