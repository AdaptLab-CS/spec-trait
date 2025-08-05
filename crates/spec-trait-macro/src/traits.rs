use crate::conversions::{str_to_generics, str_to_trait, strs_to_trait_fns};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use syn::{FnArg, ItemTrait, TraitItem};

#[derive(Serialize, Deserialize, Debug)]
pub struct TraitBody {
    pub name: String,
    pub generics: String,
    pub fns: Vec<String>,
    pub _raw: String,
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
        _raw: raw_str,
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

pub fn create_spec(trait_body: &TraitBody, spec_trait_name: &str) -> TokenStream2 {
    let name = str_to_trait(&spec_trait_name);
    let generics = str_to_generics(&trait_body.generics);
    let fns = strs_to_trait_fns(&trait_body.fns);

    quote::quote! {
        trait #name #generics {
            #(#fns)*
        }
    }
}

pub fn filter_by_fn(trait_body: &TraitBody, fn_name: &str, args_len: usize) -> Vec<String> {
    let fns = strs_to_trait_fns(&trait_body.fns);
    fns.iter()
        .filter_map(|f| match f {
            TraitItem::Fn(fn_) => {
                let name = fn_.sig.ident.to_string();
                let args = fn_
                    .sig
                    .inputs
                    .iter()
                    .filter(|arg| match *arg {
                        FnArg::Receiver(_) => false,
                        FnArg::Typed(_) => true,
                    })
                    .count();
                if name == fn_name && args == args_len {
                    Some(name)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}
