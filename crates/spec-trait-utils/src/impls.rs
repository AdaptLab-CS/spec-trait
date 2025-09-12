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
use crate::parsing::{ get_generics, handle_type_predicate, parse_generics };
use crate::specialize::{
    add_generic,
    apply_type_condition,
    get_assignable_conditions,
    Specializable,
};
use proc_macro2::TokenStream;
use serde::{ Deserialize, Serialize };
use syn::{ Attribute, GenericParam, Generics, ItemImpl };
use std::collections::HashSet;
use std::fmt::Debug;
use quote::quote;
use syn::visit_mut::VisitMut;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImplBody {
    pub condition: Option<WhenCondition>,
    pub impl_generics: String,
    pub trait_name: String,
    pub trait_generics: String,
    pub type_name: String,
    pub items: Vec<String>,
    pub specialized: Option<Box<ImplBody>>,
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

        Ok(
            (ImplBody {
                condition,
                impl_generics,
                trait_name,
                trait_generics,
                type_name,
                items,
                specialized: None,
            }).specialize()
        )
    }
}

fn get_trait_name_without_generics(trait_with_generics: &str) -> String {
    trait_with_generics.split('<').next().unwrap_or(trait_with_generics).trim().to_string()
}

impl From<&ImplBody> for TokenStream {
    fn from(impl_body: &ImplBody) -> Self {
        let impl_body = impl_body.specialized.as_ref().expect("ImplBody not specialized");

        let impl_generics = str_to_generics(&impl_body.impl_generics);
        let trait_name = str_to_trait_name(&impl_body.trait_name);
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

impl Specializable for ImplBody {
    fn resolve_item_generic(&self, _: &Generics, impl_generic: &str) -> Option<String> {
        Some(impl_generic.to_string())
    }

    fn handle_items_replace<V: VisitMut>(&mut self, replacer: &mut V) {
        let mut new_items = vec![];

        for mut item in strs_to_impl_items(&self.items) {
            replacer.visit_impl_item_mut(&mut item);
            new_items.push(to_string(&item));
        }

        self.items = new_items;
    }
}

impl ImplBody {
    fn get_spec_trait_name(&self) -> String {
        match &self.condition {
            Some(c) => format!("{}_{}_{}", self.trait_name, self.type_name, to_hash(c)),
            None => self.trait_name.to_owned(),
        }
    }

    pub fn specialize(&mut self) -> Self {
        let mut new_impl = self.clone();
        let mut specialized = new_impl.clone();

        // set specialized trait name
        specialized.trait_name = specialized.get_spec_trait_name();

        // apply condition
        if let Some(condition) = &self.condition {
            specialized.apply_condition(condition);
        }

        // set missing generics
        let mut trait_generics = str_to_generics(&specialized.trait_generics);
        let curr_generics = get_generics::<HashSet<_>>(&specialized.trait_generics);
        for generic in get_generics::<Vec<_>>(&specialized.impl_generics) {
            if !curr_generics.contains(&generic) {
                add_generic(&mut trait_generics, &generic);
            }
        }
        specialized.trait_generics = to_string(&trait_generics);

        new_impl.specialized = Some(Box::new(specialized));
        new_impl
    }

    // TODO: clean unused generics at the end
    fn apply_condition(&mut self, condition: &WhenCondition) {
        match condition {
            WhenCondition::All(inner) => {
                let assignable = get_assignable_conditions(inner, &self.impl_generics);

                // pass multiple times to handle chained dependencies
                for _ in 0..assignable.len() {
                    for c in &assignable {
                        self.apply_condition(c);
                    }
                }
            }

            WhenCondition::Type(generic, type_) => {
                let mut generics = str_to_generics(&self.impl_generics);
                let mut other_generics = str_to_generics(&self.trait_generics);

                apply_type_condition(self, &mut generics, &mut other_generics, generic, type_);

                self.impl_generics = to_string(&generics);
                self.trait_generics = to_string(&other_generics);
            }

            WhenCondition::Trait(_, _) => {
                let mut generics = str_to_generics(&self.impl_generics);
                let predicate = trait_condition_to_generic_predicate(&condition);

                handle_type_predicate(&predicate, &mut generics);

                self.impl_generics = to_string(&generics);
            }

            _ => {}
        }
    }

    /**
        get the generic in the trait corresponding to the impl_generic in the impl
        # Example:
        for trait `TraitName<A, B>` and impl `impl<T, U> TraitName<T, U> for MyType`
        - trait_generic = A -> trait_generic = T
        - trait_generic = B -> trait_generic = U
     */
    pub fn get_corresponding_generic(
        &self,
        trait_generics: &Generics,
        trait_generic: &str
    ) -> Option<String> {
        let impl_generics = str_to_generics(&self.trait_generics);

        let trait_generic_param = trait_generics.params
            .iter()
            .position(
                |param| matches!(param, GenericParam::Type(tp) if tp.ident == trait_generic)
            )?;

        match impl_generics.params.iter().nth(trait_generic_param) {
            Some(GenericParam::Type(tp)) => Some(tp.ident.to_string()),
            _ => None,
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
        let violating = impls.iter().find(|other| {
            let a = str_to_generics(&impl_.impl_generics);
            let b = str_to_generics(&other.impl_generics);

            let lifetimes_a = a.params
                .iter()
                .filter_map(|p| {
                    match p {
                        GenericParam::Lifetime(ld) => Some(to_string(ld)),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>();

            let lifetimes_b = b.params
                .iter()
                .filter_map(|p| {
                    match p {
                        GenericParam::Lifetime(ld) => Some(to_string(ld)),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>();

            impl_.type_name == other.type_name &&
                impl_.trait_name == other.trait_name &&
                lifetimes_a != lifetimes_b
        });

        if let Some(v) = violating {
            panic!(
                "Impl for type '{}' and trait '{}' has conflicting lifetimes constraints: '{}' vs '{}'",
                impl_.type_name,
                impl_.trait_name,
                impl_.impl_generics,
                v.impl_generics
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
                WhenCondition::Type("V".into(), "String".into()),
                WhenCondition::Type("T".into(), "Vec<_>".into())
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

    #[test]
    fn apply_type_condition_unsuccessful() {
        let mut impl_body = get_impl_body();
        let condition = WhenCondition::All(
            vec![
                WhenCondition::Type("T".into(), "MyType".into()),
                WhenCondition::Type("T".into(), "OtherType".into())
            ]
        );

        impl_body.apply_condition(&condition);

        assert_eq!(
            impl_body.impl_generics.replace(" ", ""),
            "<T: Clone, U: Copy>".to_string().replace(" ", "")
        );
        assert_eq!(
            impl_body.trait_generics.replace(" ", ""),
            "<T, U>".to_string().replace(" ", "")
        );
        assert_eq!(
            impl_body.items
                .into_iter()
                .map(|item| item.replace(" ", ""))
                .collect::<Vec<_>>(),
            vec![
                "type Bar = ();".to_string().replace(" ", ""),
                "fn foo(&self, arg1: Vec<T>, arg2: U) -> T { let x: T = arg1[0].clone(); x }"
                    .to_string()
                    .replace(" ", "")
            ]
        );
    }
}
