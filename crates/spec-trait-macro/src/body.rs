use crate::traits::{TraitBody, generate_trait_name};
use core::panic;
use proc_macro::TokenStream;
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

pub fn create_spec(impl_body: &ImplBody, trait_body: &TraitBody) -> (String, TokenStream) {
    let impl_generics = &impl_body.generics;
    let impl_trait = &impl_body.trait_;
    let impl_trait_with_generics = &impl_body.trait_with_generics;
    let impl_type = &impl_body.ty;
    let impl_fns = &impl_body.fns;

    let trait_name = &trait_body.name;
    let trait_generics = &trait_body.generics;
    let trait_fns = &trait_body.fns;

    if impl_trait != trait_name {
        panic!(
            "Trait in impl block does not match trait definition: {} != {}",
            impl_trait, trait_name
        );
    }

    let new_trait_name = generate_trait_name(trait_name);
    let new_impl_trait_with_generics =
        impl_trait_with_generics.replace(trait_name, &new_trait_name);

    let trait_impl_block = quote::quote! {
        trait #new_trait_name #trait_generics {
            #(#trait_fns)*
        }

        impl #impl_generics #new_impl_trait_with_generics for #impl_type {
            #(#impl_fns)*
        }
    };

    (new_trait_name, trait_impl_block.into())
}
