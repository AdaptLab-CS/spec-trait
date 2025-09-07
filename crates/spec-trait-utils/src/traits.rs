use crate::conditions::WhenCondition;
use crate::conversions::{
    str_to_generics,
    str_to_trait_name,
    str_to_type_name,
    strs_to_trait_items,
    to_string,
    tokens_to_trait,
    trait_condition_to_generic_predicate,
};
use crate::impls::ImplBody;
use crate::parsing::{ handle_type_predicate, parse_generics, replace_type, replace_infers };
use proc_macro2::{ Span, TokenStream };
use serde::{ Deserialize, Serialize };
use syn::{ GenericParam, Generics, Ident, Type, TypeParam };
use std::collections::HashSet;
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
use syn::visit_mut::{ self, VisitMut };

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

    pub fn specialize(&self, impl_body: &ImplBody) -> Self {
        let mut new_trait = self.clone();
        new_trait.name = impl_body.spec_trait_name.clone();
        // if let Some(condition) = &impl_body.condition {
        //     let impl_generics = str_to_generics(&impl_body.trait_generics);
        //     new_trait.apply_condition(&impl_generics, condition);
        // }
        new_trait
    }

    fn apply_condition(&mut self, impl_trait_generics: &mut Generics, condition: &WhenCondition) {
        match condition {
            WhenCondition::All(conds) => {
                // pass multiple times to handle chained dependencies
                for _ in 0..conds.len() {
                    for c in conds {
                        self.apply_condition(impl_trait_generics, c);
                    }
                }
            }

            // replace generic
            WhenCondition::Type(impl_generic, type_) => {
                let mut generics = str_to_generics(&self.generics);
                let trait_generic = self
                    .get_corresponding_generic(&impl_trait_generics, impl_generic)
                    .unwrap_or_else(|| impl_generic.clone());

                // remove generic from generics (no-op if trait_generic not present)
                generics.params = generics.params
                    .into_iter()
                    .filter(
                        |param|
                            !matches!(param, GenericParam::Type(tp) if tp.ident.to_string() == *trait_generic)
                    )
                    .collect();
                impl_trait_generics.params = impl_trait_generics.params
                    .clone()
                    .into_iter()
                    .filter(
                        |param|
                            !matches!(param, GenericParam::Type(tp) if tp.ident.to_string() == *impl_generic)
                    )
                    .collect();

                // replace infers in the type
                let mut new_ty = str_to_type_name(type_);
                let mut existing_generics = generics.params
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
                    generics.params.push(param.clone());
                    impl_trait_generics.params.push(param);
                }

                // replace generic with type in the trait items
                let items = strs_to_trait_items(&self.items);
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
                    generic: trait_generic.clone(),
                    new_ty: new_ty.clone(),
                };

                for mut item in items {
                    replacer.visit_trait_item_mut(&mut item);
                    new_items.push(item);
                }

                // update generics and items
                self.generics = to_string(&generics);
                self.items = new_items.iter().map(to_string).collect();
            }

            // add trait bound
            WhenCondition::Trait(impl_generic, traits) => {
                let mut generics = str_to_generics(&self.generics);
                let trait_generic = self
                    .get_corresponding_generic(&impl_trait_generics, impl_generic)
                    .unwrap_or_else(|| impl_generic.clone());

                let predicate = trait_condition_to_generic_predicate(
                    &WhenCondition::Trait(trait_generic, traits.clone())
                );
                handle_type_predicate(&predicate, &mut generics);
                self.generics = to_string(&generics);
            }
            _ => {}
        }
    }

    fn get_corresponding_generic(
        &self,
        impl_generics: &Generics,
        impl_generic: &str
    ) -> Option<String> {
        let trait_generics = str_to_generics(&self.generics);

        let impl_generic_param = impl_generics.params
            .iter()
            .position(
                |param|
                    matches!(param, GenericParam::Type(tp) if tp.ident.to_string() == *impl_generic)
            )?;

        match trait_generics.params.iter().nth(impl_generic_param) {
            Some(GenericParam::Type(tp)) => Some(tp.ident.to_string()),
            _ => None,
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
                fn foo(&self, arg1: Vec<T>, arg2: U) -> T;
            }
        }
        ).unwrap()
    }

    #[test]
    fn apply_trait_condition() {
        let mut trait_body = get_trait_body();
        let mut impl_trait_generics = str_to_generics("<T, A>");
        let condition = WhenCondition::Trait("T".into(), vec!["Copy".into(), "Clone".into()]);

        trait_body.apply_condition(&mut impl_trait_generics, &condition);

        assert_eq!(
            trait_body.generics.replace(" ", ""),
            "<T: Clone + Copy, U: Copy>".to_string().replace(" ", "")
        );
    }

    #[test]
    fn apply_type_condition() {
        let mut trait_body = get_trait_body();
        let mut impl_trait_generics = str_to_generics("<T, A>");
        let condition = WhenCondition::Type("T".into(), "String".into());

        trait_body.apply_condition(&mut impl_trait_generics, &condition);

        assert_eq!(trait_body.generics.replace(" ", ""), "<U: Copy>".to_string().replace(" ", ""));
        assert_eq!(
            trait_body.items
                .into_iter()
                .map(|item| item.replace(" ", ""))
                .collect::<Vec<_>>(),
            vec![
                "type Bar;".to_string().replace(" ", ""),
                "fn foo(&self, arg1: Vec<String>, arg2: U) -> String;".to_string().replace(" ", "")
            ]
        );
    }

    #[test]
    fn apply_type_condition_with_wildcard() {
        let mut trait_body = get_trait_body();
        let mut impl_trait_generics = str_to_generics("<T, A>");
        let condition = WhenCondition::Type("T".into(), "Vec<_>".into());

        trait_body.apply_condition(&mut impl_trait_generics, &condition);

        assert_eq!(
            trait_body.generics.replace(" ", ""),
            "<U: Copy, __W0>".to_string().replace(" ", "")
        );
        assert_eq!(
            trait_body.items
                .into_iter()
                .map(|item| item.replace(" ", ""))
                .collect::<Vec<_>>(),
            vec![
                "type Bar;".to_string().replace(" ", ""),
                "fn foo(&self, arg1: Vec<Vec<__W0>>, arg2: U) -> Vec<__W0>;"
                    .to_string()
                    .replace(" ", "")
            ]
        );
    }

    #[test]
    fn apply_type_condition_all() {
        let mut trait_body = get_trait_body();
        let mut impl_trait_generics = str_to_generics("<T, A>");
        let condition = WhenCondition::All(
            vec![
                WhenCondition::Type("T".into(), "Vec<V>".into()),
                WhenCondition::Type("V".into(), "String".into())
            ]
        );

        trait_body.apply_condition(&mut impl_trait_generics, &condition);

        assert_eq!(trait_body.generics.replace(" ", ""), "<U: Copy>".to_string().replace(" ", ""));
        assert_eq!(
            trait_body.items
                .into_iter()
                .map(|item| item.replace(" ", ""))
                .collect::<Vec<_>>(),
            vec![
                "type Bar;".to_string().replace(" ", ""),
                "fn foo(&self, arg1: Vec<Vec<String>>, arg2: U) -> Vec<String>;"
                    .to_string()
                    .replace(" ", "")
            ]
        );
    }
}
