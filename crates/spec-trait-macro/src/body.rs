use core::panic;
use proc_macro::TokenStream;
use std::fmt::Debug;
use syn::ItemImpl;

#[derive(Debug)]
pub struct ImplBody {
    generics: String,
    trait_: String,
    ty: String,
    fns: Vec<String>,
    raw: String,
}

pub fn parse(tokens: TokenStream) -> ImplBody {
    let bod = syn::parse::<ItemImpl>(tokens.into()).expect("Failed to parse ItemImpl");

    let impl_generics = &bod.generics;
    let impl_trait = &bod.trait_;
    let impl_type = &bod.self_ty;
    let impl_items = &bod.items;

    let raw_str = quote::quote! { #bod }.to_string();
    let generics_str = quote::quote! { #impl_generics }.to_string();
    let trait_str = if let Some((_, path, _)) = impl_trait {
        quote::quote! { #path }.to_string()
    } else {
        panic!("Trait not specified in impl block")
    };
    let ty_str = quote::quote! { #impl_type }.to_string();
    let fns = impl_items
        .iter()
        .map(|item| quote::quote! { #item }.to_string())
        .collect();

    ImplBody {
        generics: generics_str,
        trait_: trait_str,
        ty: ty_str,
        fns,
        raw: raw_str,
    }
}
