use crate::conversions::{
    str_to_generics,
    str_to_trait,
    str_to_type,
    strs_to_impl_fns,
    to_string,
    tokens_to_impl,
    trait_to_string,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ImplBody {
    pub generics: String,
    pub trait_: String,
    pub trait_with_generics: String,
    pub type_: String,
    pub fns: Vec<String>,
}

pub fn parse(tokens: TokenStream) -> ImplBody {
    let bod = tokens_to_impl(tokens);

    let generics = to_string(&bod.generics);
    let trait_with_generics = trait_to_string(&bod.trait_);
    let trait_ = get_trait_name_without_generics(&trait_with_generics);
    let type_ = to_string(&bod.self_ty);
    let fns = bod.items.iter().map(to_string).collect();

    ImplBody { generics, trait_, trait_with_generics, type_, fns }
}

fn get_trait_name_without_generics(trait_with_generics: &str) -> String {
    trait_with_generics.split('<').next().unwrap_or(trait_with_generics).trim().to_string()
}

pub fn create_spec(impl_body: &ImplBody, spec_trait_name: &str) -> TokenStream2 {
    let generics = str_to_generics(&impl_body.generics);
    let trait_with_generics = impl_body.trait_with_generics.replace(
        &impl_body.trait_,
        spec_trait_name
    );
    let trait_ = str_to_trait(&trait_with_generics);
    let type_ = str_to_type(&impl_body.type_);
    let fns = strs_to_impl_fns(&impl_body.fns);

    quote::quote! {
        impl #generics #trait_ for #type_ {
            #(#fns)*
        }
    }
}
