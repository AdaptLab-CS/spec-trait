use syn::{ GenericParam, Generics };

use crate::impls::ImplBody;
use crate::conversions::{ str_to_generics, to_string };
use crate::parsing::get_generics;

/// assert that all lifetimes constraints in impls follow the rules
pub fn assert_constraints(impls: &[ImplBody]) {
    assert_impl_lifetimes(impls);
}

/// Rule 2: in every spec we must have the same lifetimes costraints as in the default spec, so every generic parameter `T` can either have no lifetime constraint in every spec or have the same constraint (generic `'a` or `'static`) in each one of them.
fn assert_impl_lifetimes(impls: &[ImplBody]) {
    for impl_ in impls {
        let violating = impls.iter().find(|other| {
            let lifetimes_a = get_lifetimes(&impl_);
            let lifetimes_b = get_lifetimes(&other);

            let same_impl =
                impl_.type_name == other.type_name && impl_.trait_name == other.trait_name;
            same_impl && lifetimes_a != lifetimes_b
        });

        if let Some(other) = violating {
            panic!(
                "Impl for type '{}' and trait '{}' has conflicting lifetimes constraints: '{:?}' vs '{:?}'",
                impl_.type_name,
                impl_.trait_name,
                get_lifetimes(&impl_),
                get_lifetimes(&other)
            );
        }
    }
}

/// Extract lifetimes from impl generics that are present in the trait generics.
fn get_lifetimes(impl_: &ImplBody) -> Vec<Option<String>> {
    let impl_generics = str_to_generics(&impl_.impl_generics);
    let trait_generics = get_generics::<Vec<_>>(&impl_.trait_generics);

    let lifetimes_constraints = parse_generics_lifetimes(&impl_generics);

    trait_generics
        .iter()
        .map(|g| {
            lifetimes_constraints
                .iter()
                .find(|(name, _)| name == g)
                .map(|(_, lt)| lt.clone())
                .unwrap_or(None)
        })
        .collect()
}

/// Parse lifetimes from generic type parameters.
fn parse_generics_lifetimes(generics: &Generics) -> Vec<(String, Option<String>)> {
    generics.params
        .iter()
        .filter_map(|p| {
            match p {
                GenericParam::Type(tp) => {
                    tp.bounds
                        .iter()
                        .find_map(|b| {
                            if let syn::TypeParamBound::Lifetime(lt) = b {
                                Some((tp.ident.to_string(), Some(to_string(lt))))
                            } else {
                                Some((tp.ident.to_string(), None))
                            }
                        })
                        .or(Some((tp.ident.to_string(), None)))
                }
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversions::str_to_generics;

    #[test]
    fn test_parse_generics_lifetimes() {
        let generics = str_to_generics(
            "<'a, 'b, 'c, T: 'a, U: 'b + Trait, V, W: Trait, X: 'static>"
        );
        let res = parse_generics_lifetimes(&generics);

        let expected = vec![
            ("T".to_string(), Some("'a".to_string())),
            ("U".to_string(), Some("'b".to_string())),
            ("V".to_string(), None),
            ("W".to_string(), None),
            ("X".to_string(), Some("'static".to_string()))
        ];

        assert_eq!(res, expected);
    }

    #[test]
    fn test_get_lifetimes() {
        let impl_ = ImplBody {
            impl_generics: "<'a, 'b, 'c, T: 'a, U: 'b + Trait, V, W: Trait, X: 'static>".to_string(),
            trait_generics: "<T, V, X, U, W, X>".to_string(),
            ..Default::default()
        };
        let res = get_lifetimes(&impl_);

        let expected = vec![
            Some("'a".to_string()),
            None,
            Some("'static".to_string()),
            Some("'b".to_string()),
            None,
            Some("'static".to_string())
        ];

        assert_eq!(res, expected);
    }

    #[test]
    fn assert_impl_lifetimes_simple() {
        let a = ImplBody {
            impl_generics: "<'a, T: 'a>".to_string(),
            trait_generics: "<T>".to_string(),
            type_name: "MyType".to_string(),
            trait_name: "MyTrait".to_string(),
            ..Default::default()
        };
        let b = ImplBody {
            impl_generics: "<'a, T: 'a>".to_string(),
            trait_generics: "<T>".to_string(),
            type_name: "MyType".to_string(),
            trait_name: "MyTrait".to_string(),
            ..Default::default()
        };

        assert_impl_lifetimes(&[a, b]);
    }

    #[test]
    fn assert_impl_lifetimes_different_order() {
        let a = ImplBody {
            impl_generics: "<'a, T: 'a, U: 'static>".to_string(),
            trait_generics: "<T, U>".to_string(),
            type_name: "MyType".to_string(),
            trait_name: "MyTrait".to_string(),
            ..Default::default()
        };
        let b = ImplBody {
            impl_generics: "<'a, T: 'static, U: 'a>".to_string(),
            trait_generics: "<U, T>".to_string(),
            type_name: "MyType".to_string(),
            trait_name: "MyTrait".to_string(),
            ..Default::default()
        };

        assert_impl_lifetimes(&[a, b]);
    }

    #[test]
    fn assert_impl_lifetimes_different_type_or_trait() {
        let a = ImplBody {
            impl_generics: "<'a, T: 'a>".to_string(),
            trait_generics: "<T>".to_string(),
            type_name: "TypeA".to_string(),
            trait_name: "Trait".to_string(),
            ..Default::default()
        };
        let b = ImplBody {
            impl_generics: "<'static, T: 'static>".to_string(),
            trait_generics: "<T>".to_string(),
            type_name: "TypeB".to_string(),
            trait_name: "Trait".to_string(),
            ..Default::default()
        };
        let c = ImplBody {
            impl_generics: "<'static, T: 'static>".to_string(),
            trait_generics: "<T>".to_string(),
            type_name: "TypeA".to_string(),
            trait_name: "OtherTrait".to_string(),
            ..Default::default()
        };

        assert_impl_lifetimes(&[a, b, c]);
    }

    #[test]
    #[should_panic(expected = "conflicting lifetimes constraints")]
    fn assert_impl_lifetimes_conflict() {
        let a = ImplBody {
            impl_generics: "<'a, T: 'a>".to_string(),
            trait_generics: "<T>".to_string(),
            type_name: "X".to_string(),
            trait_name: "Y".to_string(),
            ..Default::default()
        };
        let b = ImplBody {
            impl_generics: "<'static, T: 'static>".to_string(),
            trait_generics: "<T>".to_string(),
            type_name: "X".to_string(),
            trait_name: "Y".to_string(),
            ..Default::default()
        };

        assert_impl_lifetimes(&[a, b]);
    }

    #[test]
    #[should_panic(expected = "conflicting lifetimes constraints")]
    fn assert_impl_lifetimes_different_order_conflict() {
        let a = ImplBody {
            impl_generics: "<'a, T: 'a, U: 'static>".to_string(),
            trait_generics: "<T, U>".to_string(),
            type_name: "MyType".to_string(),
            trait_name: "MyTrait".to_string(),
            ..Default::default()
        };
        let b = ImplBody {
            impl_generics: "<'a, T: 'a, U: 'static>".to_string(),
            trait_generics: "<U, T>".to_string(),
            type_name: "MyType".to_string(),
            trait_name: "MyTrait".to_string(),
            ..Default::default()
        };

        assert_impl_lifetimes(&[a, b]);
    }
}
