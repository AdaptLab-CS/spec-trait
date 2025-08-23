use std::path::Path;
use std::fs;

use syn::Item;

struct FileContent {
    traits: Vec<ItemTrait>,
    impls: Vec<ItemImpl>,
}

pub fn parse(path: &Path) -> Vec<Item> {
    let content = fs::read_to_string(path).expect("failed to read file");
    let file = syn::parse_file(&content).expect("failed to parse content");
    file.items // TODO: parse each item properly
}
