use crate::conversions::{str_to_generics, str_to_trait, str_to_type, strs_to_impl_fns};
use core::panic;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::fmt::Debug;
use syn::ItemImpl;

#[derive(Debug)]
pub struct ImplBody {
    pub generics: String,
    pub trait_: String,
    pub trait_with_generics: String,
    pub type_: String,
    pub fns: Vec<String>,
    pub _raw: String,
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
    let trait_ = trait_str
        .split('<')
        .next()
        .unwrap_or(&trait_str)
        .trim()
        .to_string();
    let ty_str = quote::quote! { #impl_type }.to_string();
    let fns = impl_items
        .iter()
        .map(|item| quote::quote! { #item }.to_string())
        .collect();

    ImplBody {
        generics: generics_str,
        trait_,
        trait_with_generics: trait_str,
        type_: ty_str,
        fns,
        _raw: raw_str,
    }
}

pub fn create_spec(impl_body: &ImplBody, spec_trait_name: &str) -> TokenStream2 {
    let generics = str_to_generics(&impl_body.generics);
    let trait_with_generics = impl_body
        .trait_with_generics
        .replace(&impl_body.trait_, spec_trait_name);
    let trait_ = str_to_trait(&trait_with_generics);
    let type_ = str_to_type(&impl_body.type_);
    let fns = strs_to_impl_fns(&impl_body.fns);

    quote::quote! {
        impl #generics #trait_ for #type_ {
            #(#fns)*
        }
    }
}
