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
    pub ty: String,
    pub fns: Vec<String>,
    pub raw: String,
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
        ty: ty_str,
        fns,
        raw: raw_str,
    }
}

pub fn create_spec(impl_body: &ImplBody, spec_trait_name: &str) -> TokenStream2 {
    let generics = syn::parse_str::<syn::Generics>(&impl_body.generics).unwrap();
    let trait_with_generics = impl_body
        .trait_with_generics
        .replace(&impl_body.trait_, spec_trait_name);
    let trait_ = syn::parse_str::<syn::Path>(&trait_with_generics).unwrap();
    let type_ = syn::parse_str::<syn::Type>(&impl_body.ty).unwrap();
    let fns: Vec<syn::ImplItem> = impl_body
        .fns
        .iter()
        .map(|f| syn::parse_str::<syn::ImplItem>(f).unwrap())
        .collect();

    quote::quote! {
        impl #generics #trait_ for #type_ {
            #(#fns)*
        }
    }
}
