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
use syn::{
    token::Comma,
    punctuated::Punctuated,
    Attribute,
    FnArg,
    ItemTrait,
    TraitItem,
    TraitItemFn,
};
use quote::quote;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraitBody {
    pub name: String,
    pub generics: String,
    pub fns: Vec<String>,
}

impl TryFrom<TokenStream> for TraitBody {
    type Error = syn::Error;

    fn try_from(tokens: TokenStream) -> Result<Self, Self::Error> {
        let bod = tokens_to_trait(tokens)?;

        let name = bod.ident.to_string();
        let generics = to_string(&bod.generics);
        let fns = bod.items.iter().map(to_string).collect();

        Ok(TraitBody { name, generics, fns })
    }
}

impl From<&TraitBody> for TokenStream {
    fn from(trait_body: &TraitBody) -> Self {
        let name = str_to_trait_name(&trait_body.name);
        let generics = str_to_generics(&trait_body.generics);
        let fns = strs_to_trait_fns(&trait_body.fns);

        quote! {
            trait #name #generics {
                #(#fns)*
            }
        }
    }
}

pub fn find_fn(trait_body: &TraitBody, fn_name: &str, args_len: usize) -> Option<TraitItemFn> {
    let fns = strs_to_trait_fns(&trait_body.fns);

    fns.iter().find_map(|f| {
        match f {
            TraitItem::Fn(fn_) if
                fn_.sig.ident == fn_name &&
                count_fn_args(&fn_.sig.inputs) == args_len
            => {
                Some(fn_.clone())
            }
            _ => None,
        }
    })
}

fn count_fn_args(inputs: &Punctuated<FnArg, Comma>) -> usize {
    inputs
        .iter()
        .filter(|arg| matches!(**arg, FnArg::Typed(_)))
        .count()
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

/// from an ItemTrait returns the ItemTrait without attributes and the attributes as a Vec
pub fn break_attr(trait_: &ItemTrait) -> (ItemTrait, Vec<Attribute>) {
    let attrs = trait_.attrs.clone();
    let mut trait_no_attrs = trait_.clone();
    trait_no_attrs.attrs.clear();
    (trait_no_attrs, attrs)
}
