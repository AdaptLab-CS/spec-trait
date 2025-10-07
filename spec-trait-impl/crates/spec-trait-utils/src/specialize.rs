use std::collections::HashSet;

use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::visit_mut::{ self, VisitMut };
use syn::visit::Visit;
use syn::{ GenericParam, Generics, Ident, LifetimeParam, Type, TypeParam };
use crate::conversions::{ str_to_lifetime, str_to_type_name };
use crate::types::{
    replace_infers,
    replace_type,
    type_assignable,
    type_contains,
    type_contains_lifetime,
    Aliases,
};
use crate::conditions::WhenCondition;

// TODO: infer lifetimes as well

pub trait Specializable {
    fn resolve_item_generic(&self, other_generics: &Generics, impl_generic: &str) -> Option<String>;

    fn handle_items_replace<V: VisitMut>(&mut self, replacer: &mut V);

    fn handle_items_visit<V: for<'a> Visit<'a>>(&self, visitor: &mut V);
}

pub fn get_assignable_conditions(
    conditions: &[WhenCondition],
    generics: &str
) -> Vec<WhenCondition> {
    conditions
        .iter()
        .filter_map(|c| {
            match c {
                WhenCondition::Trait(_, _) => Some(c.clone()),
                WhenCondition::Type(g, t) => {
                    let types = get_generic_types_from_conditions(g, conditions);
                    let most_specific = types.last() == Some(t);
                    let diff_types = types
                        .iter()
                        .any(
                            |other_t|
                                !type_assignable(t, other_t, generics, &Aliases::default()) &&
                                !type_assignable(other_t, t, generics, &Aliases::default())
                        );

                    if diff_types || !most_specific {
                        None
                    } else {
                        Some(c.clone())
                    }
                }
                _ => None,
            }
        })
        .collect()
}

fn get_generic_types_from_conditions(generic: &str, conditions: &[WhenCondition]) -> Vec<String> {
    let mut types = conditions
        .iter()
        .filter_map(|c| {
            match c {
                WhenCondition::Type(g, t) if g == generic => Some(t.clone()),
                _ => None,
            }
        })
        .collect::<Vec<_>>();
    types.sort_by_key(|t| t.replace("_", "").len());
    types
}

pub struct TypeReplacer {
    pub generic: String,
    pub type_: Type,
}

impl VisitMut for TypeReplacer {
    fn visit_type_mut(&mut self, node: &mut Type) {
        replace_type(node, &self.generic, &self.type_);
        visit_mut::visit_type_mut(self, node);
    }
}

pub fn apply_type_condition<T: Specializable>(
    target: &mut T,
    generics: &mut Generics,
    other_generics: &mut Generics,
    impl_generic: &str,
    type_: &str
) -> Type {
    let item_generic = target
        .resolve_item_generic(other_generics, impl_generic)
        .unwrap_or_else(|| impl_generic.to_string());

    // replace infers in the type
    let mut new_type = str_to_type_name(type_);
    let mut existing_generics = collect_generics_types(generics);
    let mut counter = 0;
    let mut new_generics = vec![];

    replace_infers(&mut new_type, &mut existing_generics, &mut counter, &mut new_generics);

    // add new generic types
    for generic in new_generics {
        add_generic_type(generics, &generic);
        add_generic_type(other_generics, &generic);
    }

    // remove generic type
    remove_generic(generics, &item_generic);
    remove_generic(other_generics, impl_generic);

    // replace generic type with type in the items
    let mut replacer = TypeReplacer {
        generic: item_generic.clone(),
        type_: new_type.clone(),
    };

    target.handle_items_replace(&mut replacer);

    new_type
}

pub fn remove_generic(generics: &mut Generics, generic: &str) {
    generics.params = generics.params
        .clone()
        .into_iter()
        .filter(
            |param|
                !matches!(param, GenericParam::Type(tp) if tp.ident == generic) &&
                !matches!(param, GenericParam::Lifetime(lt) if lt.lifetime.to_string() == generic)
        )
        .collect();
}

pub fn collect_generics_types<T: FromIterator<String>>(generics: &Generics) -> T {
    generics.params
        .iter()
        .filter_map(|p| {
            match p {
                GenericParam::Type(tp) => Some(tp.ident.to_string()),
                _ => None,
            }
        })
        .collect()
}

pub fn collect_generics_lifetimes<T: FromIterator<String>>(generics: &Generics) -> T {
    generics.params
        .iter()
        .filter_map(|p| {
            match p {
                GenericParam::Lifetime(lt) => Some(lt.lifetime.to_string()),
                _ => None,
            }
        })
        .collect()
}

pub fn add_generic_type(generics: &mut Generics, generic: &str) {
    generics.params.push(
        GenericParam::Type(TypeParam {
            attrs: vec![],
            ident: Ident::new(generic, Span::call_site()),
            colon_token: None,
            bounds: Punctuated::new(),
            eq_token: None,
            default: None,
        })
    )
}

pub fn add_generic_lifetime(generics: &mut Generics, generic: &str) {
    generics.params.push(
        GenericParam::Lifetime(LifetimeParam {
            attrs: vec![],
            lifetime: str_to_lifetime(generic),
            colon_token: None,
            bounds: Punctuated::new(),
        })
    )
}

pub struct TypeVisitor {
    unused_generics: HashSet<String>,
    used_generics: HashSet<String>,
}

impl Visit<'_> for TypeVisitor {
    fn visit_type(&mut self, t: &Type) {
        let mut to_remove = vec![];

        for g in self.unused_generics.iter() {
            if
                (g.starts_with("'") && type_contains_lifetime(t, g)) ||
                (!g.starts_with("'") && type_contains(t, g))
            {
                self.used_generics.insert(g.clone());
                to_remove.push(g.clone());
            }
        }

        for g in to_remove {
            self.unused_generics.remove(&g);
        }
    }
}

pub fn get_used_generics<T: Specializable>(target: &T, generics: &Generics) -> HashSet<String> {
    let generic_types: HashSet<_> = collect_generics_types(generics);
    let generic_lifetimes: HashSet<_> = collect_generics_lifetimes(generics);
    let mut visitor = TypeVisitor {
        unused_generics: generic_types.union(&generic_lifetimes).cloned().collect(),
        used_generics: HashSet::new(),
    };

    target.handle_items_visit(&mut visitor);

    visitor.used_generics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversions::{ str_to_generics, to_string };
    use syn::{ Type, Generics };

    #[test]
    fn collect_add_remove_generics() {
        let mut gens = str_to_generics("<T, U: Trait>");
        let collected: Vec<_> = collect_generics_types(&gens);
        assert_eq!(collected, vec!["T".to_string(), "U".to_string()]);

        remove_generic(&mut gens, "T");
        let collected: Vec<_> = collect_generics_types(&gens);
        assert_eq!(collected, vec!["U".to_string()]);

        add_generic_type(&mut gens, "V");
        let collected: Vec<_> = collect_generics_types(&gens);
        assert_eq!(collected, vec!["U".to_string(), "V".to_string()]);
    }

    #[test]
    fn type_replacer() {
        let mut replacer = TypeReplacer { generic: "T".into(), type_: str_to_type_name("u32") };
        let mut type_ = str_to_type_name("Vec<T>");

        replacer.visit_type_mut(&mut type_);

        assert_eq!(to_string(&type_).replace(" ", ""), "Vec<u32>");
    }

    struct TestTarget {
        pub type_: Type,
    }

    impl Specializable for TestTarget {
        fn resolve_item_generic(&self, _: &Generics, _: &str) -> Option<String> {
            Some("T".to_string())
        }

        fn handle_items_replace<V: VisitMut>(&mut self, replacer: &mut V) {
            replacer.visit_type_mut(&mut self.type_);
        }

        fn handle_items_visit<V: for<'a> Visit<'a>>(&self, visitor: &mut V) {
            visitor.visit_type(&self.type_);
        }
    }

    #[test]
    fn test_apply_type_condition() {
        let mut target = TestTarget { type_: str_to_type_name("T") };
        let mut generics = str_to_generics("<T>");
        let mut other_generics = str_to_generics("<T>");
        let impl_generic = "T";
        let type_ = "String";

        apply_type_condition(&mut target, &mut generics, &mut other_generics, impl_generic, type_);

        assert_eq!(to_string(&target.type_), type_.to_string());

        let remaining: Vec<_> = collect_generics_types(&generics);
        assert!(remaining.is_empty());

        let remaining_other: Vec<_> = collect_generics_types(&other_generics);
        assert!(remaining_other.is_empty());
    }

    #[test]
    fn get_assignable_conditions_simple() {
        let conditions = vec![
            WhenCondition::Trait("T".into(), vec!["Clone".into()]),
            WhenCondition::Type("T".into(), "String".into())
        ];

        let res = get_assignable_conditions(&conditions, "<T>");

        assert_eq!(res.len(), 2);
    }

    #[test]
    fn get_assignable_conditions_conflicting_types() {
        let conditions = vec![
            WhenCondition::Trait("T".into(), vec!["Copy".into()]),
            WhenCondition::Type("T".into(), "A".into()),
            WhenCondition::Type("T".into(), "B".into())
        ];

        let res = get_assignable_conditions(&conditions, "<T>");

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], WhenCondition::Trait("T".into(), vec!["Copy".into()]));
    }

    #[test]
    fn get_generic_types_from_conditions_ordering_and_filtering() {
        let conditions = vec![
            WhenCondition::Type("T".into(), "A".into()),
            WhenCondition::Type("T".into(), "Vec<_>".into()),
            WhenCondition::Type("T".into(), "Vec<String>".into()),
            WhenCondition::Type("U".into(), "Foo".into())
        ];

        let types_t = get_generic_types_from_conditions("T", &conditions);
        assert_eq!(types_t, vec!["A".to_string(), "Vec<_>".to_string(), "Vec<String>".to_string()]);

        let types_u = get_generic_types_from_conditions("U", &conditions);
        assert_eq!(types_u, vec!["Foo".to_string()]);

        let types_v = get_generic_types_from_conditions("V", &conditions);
        assert!(types_v.is_empty());
    }
}
