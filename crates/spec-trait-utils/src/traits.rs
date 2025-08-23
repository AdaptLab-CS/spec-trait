use crate::conversions::{
    str_to_generics,
    str_to_trait_name,
    strs_to_trait_fns,
    to_string,
    tokens_to_trait,
};
use proc_macro2::TokenStream;
use serde::{ Deserialize, Serialize };
use std::fmt::Debug;
use syn::{ FnArg, TraitItem, TraitItemFn };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraitBody {
    pub name: String,
    pub generics: String,
    pub fns: Vec<String>,
}

pub fn parse(tokens: TokenStream) -> TraitBody {
    let bod = tokens_to_trait(tokens);

    let name = bod.ident.to_string();
    let generics = to_string(&bod.generics);
    let fns = bod.items.iter().map(to_string).collect();

    TraitBody { name, generics, fns }
}

pub fn create_spec(trait_body: &TraitBody, spec_trait_name: &str) -> TokenStream {
    let name = str_to_trait_name(spec_trait_name);
    let generics = str_to_generics(&trait_body.generics);
    let fns = strs_to_trait_fns(&trait_body.fns);

    quote::quote! {
        trait #name #generics {
            #(#fns)*
        }
    }
}

pub fn find_fn(trait_body: &TraitBody, fn_name: &str, args_len: usize) -> Option<TraitItemFn> {
    let fns = strs_to_trait_fns(&trait_body.fns);
    fns.iter().find_map(|f| {
        match f {
            TraitItem::Fn(fn_) => {
                let name = fn_.sig.ident.to_string();
                let args = fn_.sig.inputs
                    .iter()
                    .filter(|arg| {
                        match *arg {
                            FnArg::Receiver(_) => false,
                            FnArg::Typed(_) => true,
                        }
                    })
                    .count();
                if name == fn_name && args == args_len {
                    Some(fn_.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    })
}

pub fn get_param_types(trait_fn: &TraitItemFn) -> Vec<String> {
    trait_fn.sig.inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                FnArg::Typed(pat_type) => {
                    let t = &pat_type.ty;
                    Some((quote::quote! { #t }).to_string())
                }
                _ => None,
            }
        })
        .collect()
}
