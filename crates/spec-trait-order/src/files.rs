use std::path::Path;
use std::fs;

use spec_trait_utils::conditions::{ self, WhenCondition };
use spec_trait_utils::impls::{ self, ImplBody };
use spec_trait_utils::traits::{ self, TraitBody };
use syn::{ Attribute, Item, Meta };
use quote::quote;

#[derive(Debug)]
pub struct FileContent {
    traits: Vec<TraitBody>,
    impls: Vec<ImplBody>,
}

pub fn parse(path: &Path) -> FileContent {
    let content = fs::read_to_string(path).expect("failed to read file");
    let file = syn::parse_file(&content).expect("failed to parse content");

    FileContent {
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
        .filter_map(|impl_| {
            let (impl_no_attrs, impl_attrs) = impls::break_attr(impl_);
            let token_stream = quote! { #impl_no_attrs };
            let condition = get_condition(&impl_attrs);
            if let Some(c) = condition {
                Some(impls::parse(token_stream, &c))
            } else {
                None
            }
        })
        .collect()
}

// TODO: we might not need spec_default anymore, in that case we can simplify this function
pub fn get_condition(attrs: &[Attribute]) -> Option<Option<WhenCondition>> {
    let condition_attr = attrs.iter().find_map(|attr| {
        if is_condition(attr.path()) { Some(attr) } else { None }
    });

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
            Some(Some(condition))
        }
        Meta::Path(path) => {
            if is_condition(&path) { Some(None) } else { None }
        }
        _ => None,
    }
}

fn is_condition(path: &syn::Path) -> bool {
    path.is_ident("when") || path.is_ident("spec_default")
}
