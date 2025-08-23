use proc_macro2::TokenStream;
use syn::{ Expr, Generics, ImplItem, ItemImpl, ItemTrait, Path, TraitItem, Type };
use quote::ToTokens;
use std::hash::{ DefaultHasher, Hasher, Hash };

pub fn str_to_generics(str: &str) -> Generics {
    syn::parse_str::<Generics>(str).expect("Failed to parse generics")
}

pub fn str_to_trait_name(str: &str) -> Path {
    syn::parse_str::<Path>(str).expect("Failed to parse path")
}

pub fn str_to_type_name(str: &str) -> Type {
    syn::parse_str::<Type>(str).expect("Failed to parse type")
}

pub fn strs_to_impl_fns(strs: &[String]) -> Vec<ImplItem> {
    strs.iter()
        .map(|f| syn::parse_str::<ImplItem>(f).expect("Failed to parse impl item"))
        .collect()
}

pub fn strs_to_trait_fns(strs: &[String]) -> Vec<TraitItem> {
    strs.iter()
        .map(|f| syn::parse_str::<TraitItem>(f).expect("Failed to parse trait item"))
        .collect()
}

pub fn str_to_expr(str: &str) -> Expr {
    syn::parse_str::<Expr>(str).expect("Failed to parse expr")
}

pub fn tokens_to_trait(tokens: TokenStream) -> ItemTrait {
    syn::parse::<ItemTrait>(tokens.into()).expect("Failed to parse ItemTrait")
}

pub fn tokens_to_impl(tokens: TokenStream) -> ItemImpl {
    syn::parse::<ItemImpl>(tokens.into()).expect("Failed to parse ItemImpl")
}

pub fn to_string<T: ToTokens>(item: &T) -> String {
    (quote::quote! { #item }).to_string()
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
