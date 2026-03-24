use std::collections::{HashMap, HashSet};

use crate::conditions::WhenCondition;
use crate::conversions::{str_to_generics, str_to_lifetime, str_to_type_name, to_string};
use crate::types::{
    Aliases, TypeReplacer, replace_infers, type_assignable, type_contains, type_contains_lifetime,
};
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{GenericParam, Generics, Ident, LifetimeParam, Type, TypeParam, TypeParamBound};

// TODO: infer lifetimes as well

pub trait Specializable {
    fn resolve_item_generic(&self, other_generics: &Generics, impl_generic: &str)
    -> Option<String>;

    fn handle_items_replace<V: VisitMut>(&mut self, replacer: &mut V);

    fn handle_items_visit<V: for<'a> Visit<'a>>(&self, visitor: &mut V);
}

/// Filters the given conditions to return only those that are assignable.
/// If there are multiple type conditions for the same generic, only the most specific one is kept.
/// If there are conflicting types that are not mutually assignable, they are not included.
pub fn get_assignable_conditions(
    conditions: &[WhenCondition],
    generics: &str,
) -> Vec<WhenCondition> {
    let conditions = merge_ref_and_lifetime(conditions.to_vec());
    conditions
        .iter()
        .filter_map(|c| match c {
            WhenCondition::Trait(_, _) => Some(c.clone()),
            WhenCondition::Type(g, t) => {
                let types = get_generic_types_from_conditions(g, &conditions);
                let most_specific = types.last() == Some(t);
                let diff_types = types.iter().any(|other_t| {
                    !type_assignable(t, other_t, generics, &Aliases::default())
                        && !type_assignable(other_t, t, generics, &Aliases::default())
                });

                // if there are no conflicts, keep only the most specific type
                if diff_types || !most_specific {
                    None
                } else {
                    Some(c.clone())
                }
            }
            _ => None,
        })
        .collect()
}

/// Splits a reference type `& 'a Type` into its lifetime and inner type components.
/// Returns `None` if the type is not a reference.
fn split_ref_type(ty: &str) -> Option<(Option<String>, String)> {
    let ty = str_to_type_name(ty);
    if let syn::Type::Reference(type_ref) = ty {
        let lifetime = type_ref.lifetime.as_ref().map(|lt| lt.to_string());
        let inner_type = to_string(&type_ref.elem);
        Some((lifetime, inner_type))
    } else {
        None
    }
}

/// Merge pairs of type conditions of the form
/// `T = &Type` and `T = & 'a _` into a single `T = & 'a Type`.
fn merge_ref_and_lifetime(conditions: Vec<WhenCondition>) -> Vec<WhenCondition> {
    let mut generic_to_lifetimes: HashMap<String, Vec<usize>> = HashMap::new();
    let mut generic_to_types: HashMap<String, Vec<usize>> = HashMap::new();

    // classify conditions
    for (idx, cond) in conditions.iter().enumerate() {
        if let WhenCondition::Type(g, t) = cond
            && let Some((lt, inner)) = split_ref_type(t)
        {
            if lt.is_some() && inner == "_" {
                generic_to_lifetimes.entry(g.clone()).or_default().push(idx);
            } else if lt.is_none() && inner != "_" {
                generic_to_types.entry(g.clone()).or_default().push(idx);
            }
        }
    }

    let mut removed = HashSet::new();
    let mut new_conditions = Vec::new();

    // for each generic, merge only when there is exactly one lifetime and one type
    for (generic, lifetime_indices) in generic_to_lifetimes {
        let type_indices = match generic_to_types.get(&generic) {
            Some(v) if !v.is_empty() => v,
            _ => continue,
        };

        // collect distinct lifetimes
        let mut lifetimes = HashSet::new();
        for &idx in &lifetime_indices {
            if let WhenCondition::Type(_, t) = &conditions[idx]
                && let Some((Some(lt), _)) = split_ref_type(t)
            {
                lifetimes.insert(lt);
            }
        }

        // collect distinct inner types
        let mut types = HashSet::new();
        for &idx in type_indices {
            if let WhenCondition::Type(_, t) = &conditions[idx]
                && let Some((_, inner)) = split_ref_type(t)
            {
                types.insert(inner);
            }
        }

        // if more than one distinct lifetime or type -> conflicting, no merge
        if lifetimes.len() != 1 || types.len() != 1 {
            continue;
        }

        // we know we can merge one pair
        let lifetime_idx = lifetime_indices[0];
        let type_idx = type_indices[0];

        if let Some(merged) =
            build_merged_ref_condition(&conditions, lifetime_idx, type_idx, &generic)
        {
            removed.insert(lifetime_idx);
            removed.insert(type_idx);
            new_conditions.push(merged);
        }
    }

    // keep all non-removed originals, then append merged ones
    let mut result = vec![];
    for (idx, cond) in conditions.into_iter().enumerate() {
        if !removed.contains(&idx) {
            result.push(cond);
        }
    }
    result.extend(new_conditions);
    result
}

/// Build `T = &'a Type` from indices of `T = &'a _` and `T = &Type`.
fn build_merged_ref_condition(
    conditions: &[WhenCondition],
    lifetime_idx: usize,
    type_idx: usize,
    generic: &str,
) -> Option<WhenCondition> {
    let lifetime = match &conditions[lifetime_idx] {
        WhenCondition::Type(_, t) => t,
        _ => return None,
    };
    let type_ = match &conditions[type_idx] {
        WhenCondition::Type(_, t) => t,
        _ => return None,
    };

    let (lt_opt, _) = split_ref_type(lifetime)?;
    let (_, ty) = split_ref_type(type_)?;

    let lt = lt_opt?;

    Some(WhenCondition::Type(
        generic.to_string(),
        format!("& {} {}", lt, ty),
    ))
}

/// Returns the list of types assigned to the given generic in the conditions,
/// ordered from least specific to most specific.
fn get_generic_types_from_conditions(generic: &str, conditions: &[WhenCondition]) -> Vec<String> {
    let mut types = conditions
        .iter()
        .filter_map(|c| match c {
            WhenCondition::Type(g, t) if g == generic => Some(t.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    // `Vec<_>` is less specific than `Vec<String>`
    types.sort_by_key(|t| t.replace("_", "").len());
    types
}

pub fn apply_type_condition<T: Specializable>(
    target: &mut T,
    generics: &mut Generics,
    other_generics: &mut Generics,
    impl_generic: &str,
    type_: &str,
) -> Type {
    let item_generic = target
        .resolve_item_generic(other_generics, impl_generic)
        .unwrap_or_else(|| impl_generic.to_string());

    // replace infers in the type
    let mut new_type = str_to_type_name(type_);
    let mut existing_generics = collect_generics_types(generics);
    let mut counter = 0;
    let mut new_generics = vec![];

    replace_infers(
        &mut new_type,
        &mut existing_generics,
        &mut counter,
        &mut new_generics,
    );

    // add new generic types
    for generic in new_generics {
        if is_direct_reference_to_generic(&new_type, &generic) {
            add_generic_type_relaxed(generics, &generic);
        } else {
            add_generic_type(generics, &generic);
        }
        add_generic_type(other_generics, &generic);
        // TODO: add generic to impl Trait types
    }

    // remove generic type
    remove_generic(generics, &item_generic);
    remove_generic(other_generics, impl_generic);
    // TODO: remove generic from impl Trait types

    // replace generic type with type in the items
    let mut replacer = TypeReplacer {
        prev: item_generic.clone(),
        new: new_type.clone(),
    };

    target.handle_items_replace(&mut replacer);

    new_type
}

fn is_direct_reference_to_generic(ty: &Type, generic: &str) -> bool {
    struct Finder<'a> {
        generic: &'a str,
        found: bool,
    }

    impl Visit<'_> for Finder<'_> {
        fn visit_type_reference(&mut self, node: &syn::TypeReference) {
            if let Type::Path(path) = node.elem.as_ref()
                && path.qself.is_none()
                && path.path.segments.len() == 1
                && path.path.segments[0].ident == self.generic
            {
                self.found = true;
                return;
            }

            syn::visit::visit_type_reference(self, node);
        }
    }

    let mut finder = Finder {
        generic,
        found: false,
    };
    finder.visit_type(ty);
    finder.found
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
    generics
        .params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Type(tp) => Some(tp.ident.to_string()),
            _ => None,
        })
        .collect()
}

pub fn collect_generics_lifetimes<T: FromIterator<String>>(generics: &Generics) -> T {
    generics
        .params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Lifetime(lt) => Some(lt.lifetime.to_string()),
            _ => None,
        })
        .collect()
}

pub fn add_generic_type(generics: &mut Generics, generic: &str) {
    generics.params.push(GenericParam::Type(TypeParam {
        attrs: vec![],
        ident: Ident::new(generic, Span::call_site()),
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    }))
}

pub fn add_generic_type_relaxed(generics: &mut Generics, generic: &str) {
    let mut bounds = Punctuated::new();
    bounds.push(TypeParamBound::Verbatim(quote::quote!(?Sized)));

    generics.params.push(GenericParam::Type(TypeParam {
        attrs: vec![],
        ident: Ident::new(generic, Span::call_site()),
        colon_token: None,
        bounds,
        eq_token: None,
        default: None,
    }))
}

pub fn add_generic_lifetime(generics: &mut Generics, generic: &str) {
    generics.params.push(GenericParam::Lifetime(LifetimeParam {
        attrs: vec![],
        lifetime: str_to_lifetime(generic),
        colon_token: None,
        bounds: Punctuated::new(),
    }))
}

pub struct TypeVisitor {
    unused_generics: HashSet<String>,
    used_generics: HashSet<String>,
}

impl Visit<'_> for TypeVisitor {
    fn visit_type(&mut self, t: &Type) {
        let mut to_remove = vec![];

        for g in self.unused_generics.iter() {
            if (g.starts_with("'") && type_contains_lifetime(t, g))
                || (!g.starts_with("'") && type_contains(t, g))
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

pub fn handle_generics<F: FnMut(&str)>(generics_str: &str, mut generics_fn: F) {
    let generics = str_to_generics(generics_str);
    for g in collect_generics_lifetimes::<Vec<_>>(&generics) {
        generics_fn(&g);
    }
    for g in collect_generics_types::<Vec<_>>(&generics) {
        generics_fn(&g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        conversions::{str_to_generics, to_string},
        types::TypeReplacer,
    };
    use syn::{Generics, Type};

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
        let mut replacer = TypeReplacer {
            prev: "T".into(),
            new: str_to_type_name("u32"),
        };
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
        let mut target = TestTarget {
            type_: str_to_type_name("T"),
        };
        let mut generics = str_to_generics("<T>");
        let mut other_generics = str_to_generics("<T>");
        let impl_generic = "T";
        let type_ = "String";

        apply_type_condition(
            &mut target,
            &mut generics,
            &mut other_generics,
            impl_generic,
            type_,
        );

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
            WhenCondition::Type("T".into(), "String".into()),
        ];

        let res = get_assignable_conditions(&conditions, "<T>");

        assert_eq!(res.len(), 2);
    }

    #[test]
    fn get_assignable_conditions_conflicting_types() {
        let conditions = vec![
            WhenCondition::Trait("T".into(), vec!["Copy".into()]),
            WhenCondition::Type("T".into(), "A".into()),
            WhenCondition::Type("T".into(), "B".into()),
        ];

        let res = get_assignable_conditions(&conditions, "<T>");

        assert_eq!(res.len(), 1);
        assert_eq!(
            res[0],
            WhenCondition::Trait("T".into(), vec!["Copy".into()])
        );
    }

    #[test]
    fn get_assignable_conditions_nested_types() {
        let conditions = vec![
            WhenCondition::Type("T".into(), "A".into()),
            WhenCondition::Type("U".into(), "for <'a> fn(&'a T) -> T".into()),
        ];

        let res = get_assignable_conditions(&conditions, "<T, U>");

        assert_eq!(res.len(), 2);
    }

    #[test]
    fn get_generic_types_from_conditions_ordering_and_filtering() {
        let conditions = vec![
            WhenCondition::Type("T".into(), "A".into()),
            WhenCondition::Type("T".into(), "Vec<_>".into()),
            WhenCondition::Type("T".into(), "Vec<String>".into()),
            WhenCondition::Type("U".into(), "Foo".into()),
        ];

        let types_t = get_generic_types_from_conditions("T", &conditions);
        assert_eq!(
            types_t,
            vec![
                "A".to_string(),
                "Vec<_>".to_string(),
                "Vec<String>".to_string()
            ]
        );

        let types_u = get_generic_types_from_conditions("U", &conditions);
        assert_eq!(types_u, vec!["Foo".to_string()]);

        let types_v = get_generic_types_from_conditions("V", &conditions);
        assert!(types_v.is_empty());
    }

    #[test]
    fn split_ref_type_with_lifetime() {
        let (lt, inner) = split_ref_type("&'a i32").expect("expected reference type");
        assert_eq!(lt, Some("'a".to_string()));
        assert_eq!(inner.replace(" ", ""), "i32");
    }

    #[test]
    fn split_ref_type_non_reference() {
        assert!(split_ref_type("i32").is_none());
    }

    #[test]
    fn build_merged_ref_condition_basic() {
        let conditions = vec![
            WhenCondition::Type("T".into(), "&'a _".into()),
            WhenCondition::Type("T".into(), "&i32".into()),
        ];

        let merged =
            build_merged_ref_condition(&conditions, 0, 1, "T").expect("expected merged condition");

        assert_eq!(merged, WhenCondition::Type("T".into(), "& 'a i32".into()));
    }

    #[test]
    fn build_merged_ref_condition_invalid_indices() {
        let conditions = vec![WhenCondition::Trait("T".into(), vec!["Copy".into()])];

        let merged = build_merged_ref_condition(&conditions, 0, 0, "T");
        assert!(merged.is_none());
    }

    #[test]
    fn merge_ref_and_lifetime_single_pair() {
        let conditions = vec![
            WhenCondition::Type("T".into(), "&'a _".into()),
            WhenCondition::Type("T".into(), "&str".into()),
        ];

        let res = merge_ref_and_lifetime(conditions);

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], WhenCondition::Type("T".into(), "& 'a str".into()));
    }

    #[test]
    fn merge_ref_and_lifetime_multiple_pairs() {
        let conditions = vec![
            WhenCondition::Type("T".into(), "&'a _".into()),
            WhenCondition::Type("T".into(), "&'b _".into()),
            WhenCondition::Type("T".into(), "&Foo".into()),
            WhenCondition::Type("T".into(), "&Bar".into()),
        ];

        let res = merge_ref_and_lifetime(conditions);

        assert_eq!(res.len(), 4);

        let expected = vec![
            WhenCondition::Type("T".into(), "&'a _".into()),
            WhenCondition::Type("T".into(), "&'b _".into()),
            WhenCondition::Type("T".into(), "&Foo".into()),
            WhenCondition::Type("T".into(), "&Bar".into()),
        ];
        assert_eq!(res, expected);
    }
}
