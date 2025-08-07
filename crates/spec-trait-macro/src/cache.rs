use crate::conditions::WhenCondition;
use crate::env::get_cache_path;
use crate::traits::{self, TraitBody};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Impl {
    pub condition: Option<WhenCondition>,
    pub trait_name: String,
    pub spec_trait_name: String,
    pub type_name: String,
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

pub fn get_trait_by_name(trait_name: &str) -> Option<TraitBody> {
    let cache = read_cache();
    cache.traits.into_iter().find(|tr| tr.name == trait_name)
}

pub fn get_traits_by_fn(fn_name: &str, args_len: usize) -> Vec<TraitBody> {
    let cache = read_cache();
    cache
        .traits
        .into_iter()
        .filter(|tr| traits::filter_by_fn(tr, fn_name, args_len).len() > 0)
        .collect()
}

pub fn get_impls_by_type(type_name: &str) -> Vec<Impl> {
    let cache = read_cache();
    cache
        .impls
        .into_iter()
        .filter(|imp| imp.type_name == type_name)
        .collect()
}

pub fn get_impls_by_type_and_traits(type_name: &str, traits: &Vec<TraitBody>) -> Vec<Impl> {
    let cache = read_cache();
    let traits_names: Vec<&str> = traits.iter().map(|tr| tr.name.as_str()).collect();
    cache
        .impls
        .into_iter()
        .filter(|imp| imp.type_name == type_name && traits_names.contains(&imp.trait_name.as_str()))
        .collect()
}
