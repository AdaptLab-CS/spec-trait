use proc_macro::TokenStream;
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use syn::ItemTrait;

#[derive(Serialize, Deserialize, Debug)]
pub struct TraitBody {
    pub name: String,
    pub generics: String,
    pub fns: Vec<String>,
    pub raw: String,
}

pub fn parse(tokens: TokenStream) -> TraitBody {
    let bod = syn::parse::<ItemTrait>(tokens.into()).expect("Failed to parse ItemTrait");

    let trait_name = &bod.ident;
    let trait_generics = &bod.generics;
    let trait_items = &bod.items;

    let raw_str = quote::quote! { #bod }.to_string();
    let name_str = trait_name.to_string();
    let generics_str = quote::quote! { #trait_generics }.to_string();
    let fns = trait_items
        .iter()
        .map(|item| quote::quote! { #item }.to_string())
        .collect();

    TraitBody {
        name: name_str,
        generics: generics_str,
        fns,
        raw: raw_str,
    }
}

pub fn generate_trait_name(old_name: &String) -> String {
    let random_suffix: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{}_{}", *old_name, random_suffix)
}
