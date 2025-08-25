use std::path::Path;
use std::fs;

use spec_trait_utils::conditions::{ self, WhenCondition };
use spec_trait_utils::impls::{ self, ImplBody };
use spec_trait_utils::traits::{ self, TraitBody };
use spec_trait_utils::cache::CrateCache;
use syn::{ Attribute, Item, Meta };
use quote::quote;

pub fn flatten_contents(contents: &[CrateCache]) -> CrateCache {
    CrateCache {
        traits: contents
            .iter()
            .flat_map(|c| c.traits.clone())
            .collect(),
        impls: contents
            .iter()
            .flat_map(|c| c.impls.clone())
            .collect(),
    }
}

pub fn parse(path: &Path) -> CrateCache {
    let content = fs::read_to_string(path).expect("failed to read file");
    let file = syn::parse_file(&content).expect("failed to parse content");

    CrateCache {
        traits: get_traits(&file.items),
        impls: get_impls(&file.items),
    }
}

fn get_traits(items: &[Item]) -> Vec<TraitBody> {
    let traits = items
        .iter()
        .filter_map(|item| {
            if let Item::Trait(trait_item) = item { Some(trait_item.clone()) } else { None }
        })
        .collect::<Vec<_>>();

    traits
        .iter()
        .map(|trait_| {
            let (trait_no_attrs, _) = traits::break_attr(trait_);
            let token_stream = quote! { #trait_no_attrs };
            traits::parse(token_stream)
        })
        .collect()
}

fn get_impls(items: &[Item]) -> Vec<ImplBody> {
    let impls = items
        .iter()
        .filter_map(|item| {
            if let Item::Impl(impl_item) = item { Some(impl_item.clone()) } else { None }
        })
        .collect::<Vec<_>>();

    impls
        .iter()
        .map(|impl_| {
            let (impl_no_attrs, impl_attrs) = impls::break_attr(impl_);
            let tokens = quote! { #impl_no_attrs };
            let condition = get_condition(&impl_attrs);
            impls::parse(tokens, &condition)
        })
        .collect()
}

pub fn get_condition(attrs: &[Attribute]) -> Option<WhenCondition> {
    let condition_attr = attrs.iter().find(|attr| is_condition(attr.path()));

    if condition_attr.is_none() {
        return None;
    }

    match condition_attr.unwrap().clone().meta {
        Meta::List(meta_list) => {
            let macro_ = meta_list.path;
            if !is_condition(&macro_) {
                return None;
            }
            let params = meta_list.tokens;
            let tokens = quote! { #params };
            let condition = conditions::parse(tokens);
            Some(condition)
        }
        _ => None,
    }
}

fn is_condition(path: &syn::Path) -> bool {
    path.is_ident("when")
}
