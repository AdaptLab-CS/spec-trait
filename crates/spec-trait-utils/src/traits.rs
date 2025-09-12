use crate::conditions::WhenCondition;
use crate::conversions::{
    str_to_generics,
    str_to_trait_name,
    str_to_type_name,
    strs_to_trait_items,
    to_string,
    tokens_to_trait,
};
use crate::impls::ImplBody;
use crate::parsing::{ get_generics, parse_generics };
use crate::specialize::{
    add_generic,
    apply_type_condition,
    get_assignable_conditions,
    Specializable,
    TypeReplacer,
};
use crate::types::get_unique_generic_name;
use proc_macro2::TokenStream;
use serde::{ Deserialize, Serialize };
use syn::{ GenericParam, Generics };
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
use syn::visit_mut::VisitMut;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TraitBody {
    pub name: String,
    pub generics: String,
    pub items: Vec<String>,
    pub specialized: Option<Box<TraitBody>>,
}

impl TryFrom<TokenStream> for TraitBody {
    type Error = syn::Error;

    fn try_from(tokens: TokenStream) -> Result<Self, Self::Error> {
        let bod = tokens_to_trait(tokens)?;

        let name = bod.ident.to_string();
        let generics = to_string(&parse_generics(bod.generics));
        let items = bod.items.iter().map(to_string).collect();

        Ok(TraitBody { name, generics, items, specialized: None })
    }
}

impl From<&TraitBody> for TokenStream {
    fn from(trait_body: &TraitBody) -> Self {
        let trait_body = trait_body.specialized.as_ref().expect("TraitBody not specialized");

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

impl Specializable for TraitBody {
    fn resolve_item_generic(&self, impl_generics: &Generics, impl_generic: &str) -> Option<String> {
        self.get_corresponding_generic(impl_generics, impl_generic)
    }

    fn handle_items_replace<V: VisitMut>(&mut self, replacer: &mut V) {
        let mut new_items = vec![];

        for mut item in strs_to_trait_items(&self.items) {
            replacer.visit_trait_item_mut(&mut item);
            new_items.push(to_string(&item));
        }

        self.items = new_items;
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
        let mut specialized = new_trait.clone();

        // set specialized trait name
        specialized.name = impl_body.specialized.as_ref().unwrap().trait_name.clone();

        // replace generics with unique generic name
        let mut new_generics = vec![];
        let mut generics = get_generics(&specialized.generics);
        let mut counter = 0;

        for generic in get_generics::<Vec<String>>(&specialized.generics) {
            let new_generic_name = get_unique_generic_name(&mut generics, &mut counter);
            let type_ = str_to_type_name(&new_generic_name);

            new_generics.push(type_.clone());

            let mut replacer = TypeReplacer { generic: generic.to_owned(), type_ };
            specialized.handle_items_replace(&mut replacer);
        }

        if !new_generics.is_empty() {
            specialized.generics = (quote! { <#(#new_generics),*> }).to_string();
        }

        // apply condition
        if let Some(condition) = &impl_body.condition {
            let mut impl_generics = str_to_generics(&impl_body.trait_generics);
            specialized.apply_condition(&mut impl_generics, condition);
        }

        // set missing generics
        let mut generics = str_to_generics(&specialized.generics);
        let impl_generics = &impl_body.specialized.as_ref().unwrap().trait_generics;
        let specialized_impl_generics = str_to_generics(impl_generics);
        for generic in get_generics::<Vec<_>>(impl_generics) {
            if
                specialized
                    .get_corresponding_generic(&specialized_impl_generics, &generic)
                    .is_none()
            {
                add_generic(&mut generics, &generic);
            }
        }
        specialized.generics = to_string(&generics);

        new_trait.specialized = Some(Box::new(specialized));
        new_trait
    }

    // TODO: clean unused generics at the end
    fn apply_condition(&mut self, impl_generics: &mut Generics, condition: &WhenCondition) {
        match condition {
            WhenCondition::All(inner) => {
                let assignable = get_assignable_conditions(inner, &self.generics);

                // pass multiple times to handle chained dependencies
                for _ in 0..assignable.len() {
                    for c in &assignable {
                        self.apply_condition(impl_generics, c);
                    }
                }
            }

            WhenCondition::Type(impl_generic, type_) => {
                let mut generics = str_to_generics(&self.generics);

                apply_type_condition(self, &mut generics, impl_generics, impl_generic, type_);

                self.generics = to_string(&generics);
            }

            _ => {}
        }
    }

    /**
        get the generic in the trait corresponding to the impl_generic in the impl
        # Example:
        for trait `TraitName<A, B>` and impl `impl<T, U> TraitName<T, U> for MyType`
        - impl_generic = T -> trait_generic = A
        - impl_generic = U -> trait_generic = B
     */
    pub fn get_corresponding_generic(
        &self,
        impl_generics: &Generics,
        impl_generic: &str
    ) -> Option<String> {
        let trait_generics = str_to_generics(&self.generics);

        let impl_generic_param = impl_generics.params
            .iter()
            .position(|param| matches!(param, GenericParam::Type(tp) if tp.ident == impl_generic))?;

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
            trait Foo<S, U> {
                type Bar;
                fn foo(&self, arg1: Vec<S>, arg2: U) -> S;
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

        assert_eq!(trait_body.generics.replace(" ", ""), "<S, U>".to_string().replace(" ", ""));
    }

    #[test]
    fn apply_type_condition() {
        let mut trait_body = get_trait_body();
        let mut impl_trait_generics = str_to_generics("<T, A>");
        let condition = WhenCondition::Type("T".into(), "String".into());

        trait_body.apply_condition(&mut impl_trait_generics, &condition);

        assert_eq!(trait_body.generics.replace(" ", ""), "<U>".to_string().replace(" ", ""));
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

        assert_eq!(trait_body.generics.replace(" ", ""), "<U, __W0>".to_string().replace(" ", ""));
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
                WhenCondition::Type("V".into(), "String".into()),
                WhenCondition::Type("T".into(), "Vec<_>".into())
            ]
        );

        trait_body.apply_condition(&mut impl_trait_generics, &condition);

        assert_eq!(trait_body.generics.replace(" ", ""), "<U>".to_string().replace(" ", ""));
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

    #[test]
    fn apply_type_condition_unsuccessful() {
        let mut trait_body = get_trait_body();
        let mut impl_trait_generics = str_to_generics("<T, A>");
        let condition = WhenCondition::All(
            vec![
                WhenCondition::Type("T".into(), "MyType".into()),
                WhenCondition::Type("T".into(), "OtherType".into())
            ]
        );

        trait_body.apply_condition(&mut impl_trait_generics, &condition);

        assert_eq!(trait_body.generics.replace(" ", ""), "<S, U>".to_string().replace(" ", ""));
        assert_eq!(
            trait_body.items
                .into_iter()
                .map(|item| item.replace(" ", ""))
                .collect::<Vec<_>>(),
            vec![
                "type Bar;".to_string().replace(" ", ""),
                "fn foo(&self, arg1: Vec<S>, arg2: U) -> S;".to_string().replace(" ", "")
            ]
        );
    }
}
