use crate::conditions::WhenCondition;
use crate::conversions::{
    str_to_generics,
    str_to_trait_name,
    strs_to_trait_items,
    to_string,
    tokens_to_trait,
    trait_condition_to_generic_predicate,
};
use crate::impls::ImplBody;
use crate::parsing::{ handle_type_predicate, parse_generics };
use proc_macro2::TokenStream;
use serde::{ Deserialize, Serialize };
use syn::GenericParam;
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TraitBody {
    pub name: String,
    pub generics: String,
    pub items: Vec<String>,
}

impl TryFrom<TokenStream> for TraitBody {
    type Error = syn::Error;

    fn try_from(tokens: TokenStream) -> Result<Self, Self::Error> {
        let bod = tokens_to_trait(tokens)?;

        let name = bod.ident.to_string();
        let generics = to_string(&parse_generics(bod.generics));
        let items = bod.items.iter().map(to_string).collect();

        Ok(TraitBody { name, generics, items })
    }
}

impl From<&TraitBody> for TokenStream {
    fn from(trait_body: &TraitBody) -> Self {
        let name = str_to_trait_name(&trait_body.name);
        let generics = str_to_generics(&trait_body.generics);
        let items = strs_to_trait_items(&trait_body.items);

        quote! {
            trait #name #generics {
                #(#items)*
            }
        }
    }
}

impl TraitBody {
    pub fn find_fn(&self, fn_name: &str, args_len: usize) -> Option<TraitItemFn> {
        let fns = strs_to_trait_items(&self.items);

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

    pub fn apply_impl(&self, impl_body: &ImplBody) -> Self {
        let mut new_trait = self.clone();
        new_trait.name = impl_body.spec_trait_name.clone();
        new_trait
    }

    fn apply_condition(&mut self, condition: &WhenCondition) {
        match condition {
            WhenCondition::All(conds) => {
                for c in conds {
                    self.apply_condition(c);
                }
            }
            // replace generic
            WhenCondition::Type(generic, type_) => {
                let mut generics = str_to_generics(&self.generics);

                // remove from generics
                generics.params = generics.params
                    .into_iter()
                    .filter(
                        |param|
                            !matches!(param, GenericParam::Type(tp) if tp.ident.to_string() == *generic)
                    )
                    .collect();

                // replace generic with type in the trait items
                let items = strs_to_trait_items(&self.items);
                let new_items = items
                    .iter()
                    .map(|item| {
                        let item_str = to_string(&item);
                        let new_item_str = item_str.replace(generic, type_); // TODO: properly parse the items to replace correctly
                        syn::parse_str(&new_item_str).expect("Failed to parse trait item")
                    })
                    .collect::<Vec<TraitItem>>();

                self.generics = to_string(&generics);
                self.items = new_items.iter().map(to_string).collect();
            }
            // add trait bound
            WhenCondition::Trait(_, _) => {
                let mut generics = str_to_generics(&self.generics);
                let predicate = trait_condition_to_generic_predicate(condition);
                handle_type_predicate(&predicate, &mut generics);
                self.generics = to_string(&generics);
            }
            _ => {}
        }
    }
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
                    Some((quote! { #t }).to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_trait_body() -> TraitBody {
        TraitBody::try_from(
            quote! {
            trait Foo<T: Clone, U: Copy> {
                type Bar;
                fn foo(&self, arg1: T, arg2: U);
            }
        }
        ).unwrap()
    }

    #[test]
    fn apply_trait_condition() {
        let mut trait_body = get_trait_body();
        let condition = WhenCondition::Trait("T".into(), vec!["Copy".into(), "Clone".into()]);

        trait_body.apply_condition(&condition);

        assert_eq!(
            trait_body.generics.replace(" ", ""),
            "<T: Clone + Copy, U: Copy>".to_string().replace(" ", "")
        );
    }

    #[test]
    fn apply_type_condition() {
        let mut trait_body = get_trait_body();
        let condition = WhenCondition::Type("T".into(), "String".into());

        trait_body.apply_condition(&condition);

        assert_eq!(trait_body.generics.replace(" ", ""), "<U: Copy>".to_string().replace(" ", ""));
        assert_eq!(
            trait_body.items
                .into_iter()
                .map(|item| item.replace(" ", ""))
                .collect::<Vec<_>>(),
            vec![
                "type Bar;".to_string().replace(" ", ""),
                "fn foo(&self, arg1: String, arg2: U);".to_string().replace(" ", "")
            ]
        );
    }
}
