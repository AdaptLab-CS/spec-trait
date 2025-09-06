use crate::conversions::{
    str_to_generics,
    str_to_trait_name,
    str_to_type_name,
    strs_to_impl_items,
    to_hash,
    to_string,
    tokens_to_impl,
    trait_to_string,
};
use crate::conditions::WhenCondition;
use crate::parsing::parse_generics;
use proc_macro2::TokenStream;
use serde::{ Deserialize, Serialize };
use syn::{ ItemImpl, Attribute };
use std::fmt::Debug;
use quote::quote;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImplBody {
    pub condition: Option<WhenCondition>,
    pub impl_generics: String,
    pub trait_name: String,
    pub spec_trait_name: String,
    pub trait_generics: String,
    pub type_name: String,
    pub items: Vec<String>,
}

impl TryFrom<(TokenStream, Option<WhenCondition>)> for ImplBody {
    type Error = syn::Error;

    fn try_from((tokens, condition): (TokenStream, Option<WhenCondition>)) -> Result<
        Self,
        Self::Error
    > {
        let bod = tokens_to_impl(tokens)?;

        let impl_generics = to_string(&parse_generics(bod.generics.clone()));
        let trait_with_generics = trait_to_string(&bod.trait_);
        let trait_name = get_trait_name_without_generics(&trait_with_generics);
        let trait_generics = trait_with_generics.replace(&trait_name, "");
        let type_name = to_string(&bod.self_ty);
        let items = bod.items.iter().map(to_string).collect();
        let spec_trait_name = get_spec_trait_name(&condition, &trait_name, &type_name);

        Ok(ImplBody {
            condition,
            impl_generics,
            trait_name,
            trait_generics,
            spec_trait_name,
            type_name,
            items,
        })
    }
}

fn get_trait_name_without_generics(trait_with_generics: &str) -> String {
    trait_with_generics.split('<').next().unwrap_or(trait_with_generics).trim().to_string()
}

fn get_spec_trait_name(
    condition: &Option<WhenCondition>,
    trait_name: &str,
    type_name: &str
) -> String {
    match condition {
        Some(c) => format!("{}_{}_{}", trait_name, type_name, to_hash(c)), // TODO: check if we need the type_name here
        None => trait_name.to_owned(),
    }
}

impl From<&ImplBody> for TokenStream {
    fn from(impl_body: &ImplBody) -> Self {
        let impl_generics = str_to_generics(&impl_body.impl_generics);
        let trait_name = str_to_trait_name(&impl_body.spec_trait_name);
        let trait_generics = str_to_generics(&impl_body.trait_generics);
        let type_name = str_to_type_name(&impl_body.type_name);
        let items = strs_to_impl_items(&impl_body.items);

        quote! {
        impl #impl_generics #trait_name #trait_generics for #type_name {
            #(#items)*
        }
    }
    }
}

/// from an ItemImpl returns the ItemImpl without attributes and the attributes as a Vec
pub fn break_attr(impl_: &ItemImpl) -> (ItemImpl, Vec<Attribute>) {
    let attrs = impl_.attrs.clone();
    let mut impl_no_attrs = impl_.clone();
    impl_no_attrs.attrs.clear();
    (impl_no_attrs, attrs)
}

pub fn assert_lifetimes_constraints(impls: &[ImplBody]) {
    for impl_ in impls {
        let violating = impls
            .iter()
            .find(|other| {
                impl_.type_name == other.type_name &&
                    impl_.trait_name == other.trait_name &&
                    impl_.impl_generics != other.impl_generics
            });

        if violating.is_some() {
            panic!(
                "Impl for type '{}' and trait '{}' has conflicting lifetimes constraints: '{}' vs '{}'",
                impl_.type_name,
                impl_.trait_name,
                impl_.impl_generics,
                violating.unwrap().impl_generics
            );
        }
    }
}
