use crate::conditions::WhenCondition;
use crate::env::{FILE_CACHE, FOLDER_CACHE};
use crate::traits::TraitBody;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Impl {
    pub condition: WhenCondition,
    pub trait_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    pub traits: Vec<TraitBody>,
    pub impls: Vec<Impl>,
}

fn get_path() -> PathBuf {
    Path::new(&FOLDER_CACHE).join(&FILE_CACHE)
}

fn get_cache(path: &Path) -> Cache {
    let file_cache = fs::read(&path).expect("Failed to read file cache");

    serde_json::from_slice(&file_cache).unwrap_or_else(|_| Cache {
        traits: Vec::new(),
        impls: Vec::new(),
    })
}

pub fn add_trait(tr: TraitBody) {
    let path = get_path();
    let mut cache = get_cache(&path);

    cache.traits.push(tr);

    let serialized = serde_json::to_string(&cache).expect("Failed to serialize cache");

    fs::write(&path, serialized).expect("Failed to write file cache");
}
