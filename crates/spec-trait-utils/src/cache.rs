use crate::parsing::get_generics;
use crate::traits::TraitBody;
use crate::impls::ImplBody;
use crate::env::get_cache_path;
use crate::types::{ types_equal, Aliases };
use serde::{ Deserialize, Serialize };
use std::fs;
use std::collections::{ HashMap, HashSet };

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CrateCache {
    pub traits: Vec<TraitBody>,
    pub impls: Vec<ImplBody>,
}

pub type Cache = HashMap<String, CrateCache>;

fn read_top_level_cache() -> Cache {
    let path = get_cache_path();
    let file_cache = fs::read(&path).unwrap_or_default();
    serde_json::from_slice::<Cache>(&file_cache).unwrap_or_default()
}

fn write_top_level_cache(cache: &Cache) {
    let path = get_cache_path();
    let serialized = serde_json::to_string(cache).expect("Failed to serialize cache");
    fs::write(&path, serialized).expect("Failed to write into cache");
}

pub fn read_cache(crate_name: Option<String>) -> CrateCache {
    let crate_name = crate_name.unwrap_or_else(|| std::env::var("CARGO_PKG_NAME").unwrap());
    let cache = read_top_level_cache();
    cache.get(&crate_name).cloned().unwrap_or_default()
}

pub fn write_cache(cache: &CrateCache, crate_name: Option<String>) {
    let crate_name = crate_name.unwrap_or_else(|| std::env::var("CARGO_PKG_NAME").unwrap());

    let mut top_level_cache = read_top_level_cache();
    top_level_cache.insert(crate_name, cache.clone());

    write_top_level_cache(&top_level_cache);
}

pub fn reset() {
    let empty_cache = Cache::new();
    write_top_level_cache(&empty_cache);
}

pub fn add_crate(crate_name: &str, crate_cache: CrateCache) {
    let mut cache = read_cache(Some(crate_name.to_string()));
    cache.traits.extend(crate_cache.traits);
    cache.impls.extend(crate_cache.impls);
    write_cache(&cache, Some(crate_name.to_string()));
}

pub fn add_trait(tr: TraitBody) {
    let mut cache = read_cache(None);
    cache.traits.push(tr);
    write_cache(&cache, None);
}

pub fn add_impl(imp: ImplBody) {
    let mut cache = read_cache(None);
    cache.impls.push(imp);
    write_cache(&cache, None);
}

pub fn get_trait_by_name(trait_name: &str) -> Option<TraitBody> {
    let cache = read_cache(None);
    cache.traits.into_iter().find(|tr| tr.name == trait_name)
}

pub fn get_traits_by_fn(fn_name: &str, args_len: usize) -> Vec<TraitBody> {
    let cache = read_cache(None);
    cache.traits
        .into_iter()
        .filter(|tr| tr.find_fn(fn_name, args_len).is_some())
        .collect()
}

pub fn get_impls_by_type_and_traits(
    type_name: &str,
    traits: &[TraitBody],
    aliases: &Aliases
) -> Vec<ImplBody> {
    let cache = read_cache(None);
    let traits_names = traits
        .iter()
        .map(|tr| &tr.name)
        .collect::<HashSet<_>>();
    cache.impls
        .into_iter()
        .filter(
            |imp|
                traits_names.contains(&imp.trait_name) &&
                types_equal(&imp.type_name, type_name, &get_generics(&imp.impl_generics), aliases)
        )
        .collect()
}
