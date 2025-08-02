use crate::conditions::WhenCondition;
use crate::env::get_cache_path;
use crate::traits::TraitBody;
use serde::{Deserialize, Serialize};
use std::fs;

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

pub fn read_cache() -> Cache {
    let path = get_cache_path();
    let file_cache = fs::read(&path).expect("Failed to read from cache");
    serde_json::from_slice(&file_cache).unwrap_or_else(|_| Cache {
        traits: Vec::new(),
        impls: Vec::new(),
    })
}

pub fn write_cache(cache: &Cache) {
    let path = get_cache_path();
    let serialized = serde_json::to_string(cache).expect("Failed to serialize cache");
    fs::write(&path, serialized).expect("Failed to write into cache");
}

pub fn add_trait(tr: TraitBody) {
    let mut cache = read_cache();
    cache.traits.push(tr);
    write_cache(&cache);
}

pub fn add_impl(imp: Impl) {
    let mut cache = read_cache();
    cache.impls.push(imp);
    write_cache(&cache);
}

pub fn get_trait(trait_name: &String) -> Option<TraitBody> {
    let cache = read_cache();
    cache.traits.into_iter().find(|tr| tr.name == *trait_name)
}
