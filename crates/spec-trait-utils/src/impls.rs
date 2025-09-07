use crate::conversions::{
    str_to_generics,
    str_to_trait_name,
    str_to_type_name,
    strs_to_impl_items,
    to_hash,
    to_string,
    tokens_to_impl,
    trait_condition_to_generic_predicate,
    trait_to_string,
};
use crate::conditions::WhenCondition;
use crate::parsing::{ handle_type_predicate, parse_generics, replace_infers, replace_type };
use proc_macro2::{ Span, TokenStream };
use serde::{ Deserialize, Serialize };
use syn::punctuated::Punctuated;
use syn::{ Attribute, GenericParam, Ident, ItemImpl, Type, TypeParam };
use std::collections::HashSet;
use std::fmt::Debug;
use quote::quote;
use syn::visit_mut::{ self, VisitMut };

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

impl ImplBody {
    pub fn specialize(&self) -> Self {
        let mut new_impl = self.clone();
        if let Some(condition) = &self.condition {
            new_impl.apply_condition(condition);
        }
        new_impl
    }

    fn apply_condition(&mut self, condition: &WhenCondition) {
        match condition {
            WhenCondition::All(conds) => {
                // pass multiple times to handle chained dependencies
                for _ in 0..conds.len() {
                    for c in conds {
                        self.apply_condition(c);
                    }
                }
            }

            // replace generic
            WhenCondition::Type(generic, type_) => {
                let mut impl_generics = str_to_generics(&self.impl_generics);
                let mut trait_generics = str_to_generics(&self.trait_generics);

                // remove generic from generics
                impl_generics.params = impl_generics.params
                    .into_iter()
                    .filter(
                        |param|
                            !matches!(param, GenericParam::Type(tp) if tp.ident.to_string() == *generic)
                    )
                    .collect();

                trait_generics.params = trait_generics.params
                    .into_iter()
                    .filter(
                        |param|
                            !matches!(param, GenericParam::Type(tp) if tp.ident.to_string() == *generic)
                    )
                    .collect();

                // replace infers in the type
                let mut new_ty = str_to_type_name(type_);
                let mut existing_generics = impl_generics.params
                    .iter()
                    .filter_map(|p| {
                        match p {
                            GenericParam::Type(tp) => Some(tp.ident.to_string()),
                            _ => None,
                        }
                    })
                    .collect::<HashSet<_>>();
                let mut counter = 0;
                let mut new_generics = vec![];

                replace_infers(
                    &mut new_ty,
                    &mut existing_generics,
                    &mut counter,
                    &mut new_generics
                );

                // add new generics
                for ident in new_generics {
                    let param = GenericParam::Type(TypeParam {
                        attrs: vec![],
                        ident: Ident::new(&ident, Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                        eq_token: None,
                        default: None,
                    });
                    impl_generics.params.push(param.clone());
                    trait_generics.params.push(param);
                }

                // replace generic with type in the impl items
                let items = strs_to_impl_items(&self.items);
                let mut new_items = vec![];

                struct TypeReplacer {
                    generic: String,
                    new_ty: Type,
                }

                impl VisitMut for TypeReplacer {
                    fn visit_type_mut(&mut self, node: &mut Type) {
                        replace_type(node, &self.generic, &self.new_ty);
                        visit_mut::visit_type_mut(self, node);
                    }
                }

                let mut replacer = TypeReplacer {
                    generic: generic.clone(),
                    new_ty: new_ty.clone(),
                };

                for mut item in items {
                    replacer.visit_impl_item_mut(&mut item);
                    new_items.push(item);
                }

                // update generics and items
                self.impl_generics = to_string(&impl_generics);
                self.trait_generics = to_string(&trait_generics);
                self.items = new_items.iter().map(to_string).collect();
            }

            // add trait bound
            WhenCondition::Trait(_, _) => {
                let mut generics = str_to_generics(&self.impl_generics);
                let predicate = trait_condition_to_generic_predicate(condition);
                handle_type_predicate(&predicate, &mut generics);
                self.impl_generics = to_string(&generics);
            }
            _ => {}
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_impl_body() -> ImplBody {
        ImplBody::try_from((
            quote! {
            impl <T: Clone, U: Copy> Foo<T, U> for MyType {
                type Bar = ();
                fn foo(&self, arg1: Vec<T>, arg2: U) -> T {
                    let x: T = arg1[0].clone();
                    x
                }
            }
        },
            None,
        )).unwrap()
    }

    #[test]
    fn apply_trait_condition() {
        let mut impl_body = get_impl_body();
        let condition = WhenCondition::Trait("T".into(), vec!["Copy".into(), "Clone".into()]);

        impl_body.apply_condition(&condition);

        assert_eq!(
            impl_body.impl_generics.replace(" ", ""),
            "<T: Clone + Copy, U: Copy>".to_string().replace(" ", "")
        );
    }

    #[test]
    fn apply_type_condition() {
        let mut impl_body = get_impl_body();
        let condition = WhenCondition::Type("T".into(), "String".into());

        impl_body.apply_condition(&condition);

        assert_eq!(
            impl_body.impl_generics.replace(" ", ""),
            "<U: Copy>".to_string().replace(" ", "")
        );
        assert_eq!(impl_body.trait_generics.replace(" ", ""), "<U>".to_string().replace(" ", ""));
        assert_eq!(
            impl_body.items
                .into_iter()
                .map(|item| item.replace(" ", ""))
                .collect::<Vec<_>>(),
            vec![
                "type Bar = ();".to_string().replace(" ", ""),
                "fn foo(&self, arg1: Vec<String>, arg2: U) -> String { let x: String = arg1[0].clone(); x }"
                    .to_string()
                    .replace(" ", "")
            ]
        );
    }

    #[test]
    fn apply_type_condition_with_wildcard() {
        let mut impl_body = get_impl_body();
        let condition = WhenCondition::Type("T".into(), "Vec<_>".into());

        impl_body.apply_condition(&condition);

        assert_eq!(
            impl_body.impl_generics.replace(" ", ""),
            "<U: Copy, __W0>".to_string().replace(" ", "")
        );
        assert_eq!(
            impl_body.trait_generics.replace(" ", ""),
            "<U, __W0>".to_string().replace(" ", "")
        );
        assert_eq!(
            impl_body.items
                .into_iter()
                .map(|item| item.replace(" ", ""))
                .collect::<Vec<_>>(),
            vec![
                "type Bar = ();".to_string().replace(" ", ""),
                "fn foo(&self, arg1: Vec<Vec<__W0>>, arg2: U) -> Vec<__W0> { let x: Vec<__W0> = arg1[0].clone(); x }"
                    .to_string()
                    .replace(" ", "")
            ]
        );
    }

    #[test]
    fn apply_type_condition_all() {
        let mut impl_body = get_impl_body();
        let condition = WhenCondition::All(
            vec![
                WhenCondition::Type("T".into(), "Vec<V>".into()),
                WhenCondition::Type("V".into(), "String".into())
            ]
        );

        impl_body.apply_condition(&condition);

        assert_eq!(
            impl_body.impl_generics.replace(" ", ""),
            "<U: Copy>".to_string().replace(" ", "")
        );
        assert_eq!(impl_body.trait_generics.replace(" ", ""), "<U>".to_string().replace(" ", ""));
        assert_eq!(
            impl_body.items
                .into_iter()
                .map(|item| item.replace(" ", ""))
                .collect::<Vec<_>>(),
            vec![
                "type Bar = ();".to_string().replace(" ", ""),
                "fn foo(&self, arg1: Vec<Vec<String>>, arg2: U) -> Vec<String> { let x: Vec<String> = arg1[0].clone(); x }"
                    .to_string()
                    .replace(" ", "")
            ]
        );
    }
}
