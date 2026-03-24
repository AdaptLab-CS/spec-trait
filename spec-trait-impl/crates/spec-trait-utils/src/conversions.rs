use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use std::hash::{DefaultHasher, Hash, Hasher};
use syn::{
    Expr, Generics, ImplItem, ItemImpl, ItemTrait, Lifetime, Path, PredicateType, Result,
    TraitItem, Type, WherePredicate,
};

use crate::conditions::WhenCondition;

pub fn str_to_generics(str: &str) -> Generics {
    syn::parse_str(str).expect("Failed to parse generics")
}

pub fn str_to_trait_name(str: &str) -> Path {
    syn::parse_str(str).expect("Failed to parse path")
}

pub fn str_to_type_name(str: &str) -> Type {
    syn::parse_str(str).expect("Failed to parse type")
}

pub fn str_to_lifetime(str: &str) -> Lifetime {
    syn::parse_str(str).expect("Failed to parse lifetime")
}

pub fn strs_to_impl_items(strs: &[String]) -> Vec<ImplItem> {
    strs.iter()
        .map(|f| syn::parse_str(f).expect("Failed to parse impl item"))
        .collect()
}

pub fn strs_to_trait_items(strs: &[String]) -> Vec<TraitItem> {
    strs.iter()
        .map(|f| syn::parse_str(f).expect("Failed to parse trait item"))
        .collect()
}

pub fn str_to_expr(str: &str) -> Expr {
    syn::parse_str(str).expect("Failed to parse expr")
}

pub fn tokens_to_trait(tokens: TokenStream) -> Result<ItemTrait> {
    syn::parse2(tokens)
}

pub fn tokens_to_impl(tokens: TokenStream) -> Result<ItemImpl> {
    syn::parse2(tokens)
}

pub fn to_string<T: ToTokens>(item: &T) -> String {
    (quote! { #item }).to_string()
}

pub fn trait_to_string<T, U>(trait_: &Option<(T, Path, U)>) -> String {
    trait_
        .as_ref()
        .map(|(_, path, _)| to_string(path))
        .expect("Failed to parse path")
}

pub fn to_hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

pub fn trait_condition_to_generic_predicate(condition: &WhenCondition) -> PredicateType {
    match condition {
        WhenCondition::Trait(generic, trait_) => {
            let predicate_str = format!("{}: {}", generic, trait_.join(" + "));
            let predicate = syn::parse_str(&predicate_str).expect("Failed to parse predicate");
            match predicate {
                WherePredicate::Type(p) => p,
                _ => panic!("Expected WherePredicate::Type"),
            }
        }
        _ => panic!("Expected WhenCondition::Trait"),
    }
}
