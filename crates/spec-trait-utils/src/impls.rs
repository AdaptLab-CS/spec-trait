use crate::conversions::{
    str_to_generics,
    str_to_trait_name,
    str_to_type_name,
    strs_to_impl_fns,
    to_hash,
    to_string,
    tokens_to_impl,
    trait_to_string,
};
use crate::conditions::WhenCondition;
use proc_macro2::TokenStream;
use serde::{ Deserialize, Serialize };
use syn::{ ItemImpl, Attribute };
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImplBody {
    pub condition: Option<WhenCondition>,
    pub impl_generics: String,
    pub trait_name: String,
    pub spec_trait_name: String,
    pub trait_generics: String,
    pub type_name: String,
    pub fns: Vec<String>,
}

pub fn parse(tokens: TokenStream, condition: &Option<WhenCondition>) -> ImplBody {
    let bod = tokens_to_impl(tokens);

    let impl_generics = to_string(&bod.generics);
    let trait_with_generics = trait_to_string(&bod.trait_);
    let trait_name = get_trait_name_without_generics(&trait_with_generics);
    let trait_generics = trait_with_generics.replace(&trait_name, "");
    let type_name = to_string(&bod.self_ty);
    let fns = bod.items.iter().map(to_string).collect();
    let spec_trait_name = get_spec_trait_name(condition, &trait_name, &type_name);

    ImplBody {
        condition: condition.clone(),
        impl_generics,
        trait_name,
        trait_generics,
        spec_trait_name,
        type_name,
        fns,
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
    format!("{}_{}_{}", trait_name, type_name, to_hash(condition))
}

pub fn create_spec(impl_body: &ImplBody) -> TokenStream {
    let impl_generics = str_to_generics(&impl_body.impl_generics);
    let trait_name = str_to_trait_name(&impl_body.spec_trait_name);
    let trait_generics = str_to_generics(&impl_body.trait_generics);
    let type_name = str_to_type_name(&impl_body.type_name);
    let fns = strs_to_impl_fns(&impl_body.fns);

    quote::quote! {
        impl #impl_generics #trait_name #trait_generics for #type_name {
            #(#fns)*
        }
    }
}

pub fn break_attr(impl_: &ItemImpl) -> (ItemImpl, Vec<Attribute>) {
    let attrs = impl_.attrs.clone();
    let mut impl_no_attrs = impl_.clone();
    impl_no_attrs.attrs.clear();
    (impl_no_attrs, attrs)
}
