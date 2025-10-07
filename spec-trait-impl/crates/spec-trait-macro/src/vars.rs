use std::collections::HashSet;

use crate::SpecBody;
use crate::annotations::{Annotation, AnnotationBody};
use spec_trait_utils::conversions::{
    str_to_generics, str_to_lifetime, str_to_type_name, to_string,
};
use spec_trait_utils::impls::ImplBody;
use spec_trait_utils::parsing::get_generics_types;
use spec_trait_utils::traits::TraitBody;
use spec_trait_utils::types::{
    Aliases, get_concrete_type, type_assignable, type_assignable_generic_constraints, type_contains,
};
use syn::{FnArg, TraitItemFn, Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarInfo {
    /// if the trait parameter is generic, this is the corresponding generic in the impl
    pub impl_generic: String,
    /// if the trait parameter is generic, this is the name of the generic in the trait
    pub trait_generic: Option<String>,
    /// concrete type with which the fn was called
    pub concrete_type: String,
    /// traits implemented by the concrete_type, got from annotations
    pub traits: Vec<String>,
}

#[derive(Debug)]
pub struct VarBody {
    /// map from concrete type to type aliases
    pub aliases: Aliases,
    /// impl generics
    pub generics: String,
    /// map from type definition (e.g. generic) to VarInfo
    pub vars: Vec<VarInfo>,
}

impl From<&SpecBody> for VarBody {
    fn from(spec: &SpecBody) -> Self {
        let aliases = get_type_aliases(&spec.annotations.annotations);
        let generics = spec.impl_.impl_generics.clone();
        let vars = get_vars(&spec.annotations, &spec.impl_, &spec.trait_, &aliases);
        VarBody {
            aliases,
            generics,
            vars,
        }
    }
}

pub fn get_type_aliases(ann: &[Annotation]) -> Aliases {
    let mut aliases = Aliases::new();

    for a in ann {
        if let Annotation::Alias(type_, alias) = a {
            aliases
                .entry(type_.clone())
                .or_default()
                .push(alias.clone());
        }
    }

    aliases
}

fn get_vars(
    ann: &AnnotationBody,
    impl_: &ImplBody,
    trait_: &TraitBody,
    aliases: &Aliases,
) -> Vec<VarInfo> {
    get_generics_types::<Vec<_>>(&impl_.impl_generics)
        .iter()
        .flat_map(|g| {
            let from_type = get_generic_constraints_from_type(g, impl_, ann, aliases);
            let from_type_specialized = get_generic_constraints_from_type(
                g,
                impl_.specialized.as_ref().unwrap(),
                ann,
                aliases,
            );

            match trait_.get_corresponding_generic(&str_to_generics(&impl_.trait_generics), g) {
                // get type
                Some(trait_generic) => {
                    let from_trait = get_generic_constraints_from_trait(
                        &trait_generic,
                        trait_,
                        impl_,
                        ann,
                        aliases,
                    );

                    from_trait.into_iter().chain(from_type).collect::<Vec<_>>()
                }

                // get from specialized instead
                None => {
                    let trait_generic = trait_
                        .specialized
                        .as_ref()
                        .unwrap()
                        .get_corresponding_generic(
                            &str_to_generics(&impl_.specialized.as_ref().unwrap().trait_generics),
                            g,
                        );

                    if let Some(trait_generic) = trait_generic {
                        let from_trait = get_generic_constraints_from_trait(
                            &trait_generic,
                            trait_.specialized.as_ref().unwrap(),
                            impl_.specialized.as_ref().unwrap(),
                            ann,
                            aliases,
                        );

                        from_trait
                            .into_iter()
                            .chain(from_type_specialized)
                            .collect::<Vec<_>>()
                    } else {
                        // get from type only
                        from_type
                            .into_iter()
                            .chain(from_type_specialized)
                            .collect::<Vec<_>>()
                    }
                }
            }
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

/**
   Get the parameter types from a trait function.
   # Example
   `fn foo(&self, x: T, y: u32);` returns `vec!["T", "u32"]`
*/
fn get_param_types(trait_fn: &TraitItemFn) -> Vec<String> {
    trait_fn
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat_type) => Some(to_string(&pat_type.ty)),
            _ => None,
        })
        .collect()
}

fn get_generic_constraints_from_trait(
    trait_generic: &str,
    trait_: &TraitBody,
    impl_: &ImplBody,
    ann: &AnnotationBody,
    aliases: &Aliases,
) -> Vec<VarInfo> {
    let trait_fn = trait_.find_fn(&ann.fn_, ann.args.len()).unwrap();
    let param_types = get_param_types(&trait_fn);

    // find all params that use the generic
    let params_with_trait_generic = param_types
        .iter()
        .enumerate()
        .filter(|(_, p)| type_contains(&str_to_type_name(p), trait_generic))
        .collect::<Vec<_>>();

    // generic passed but not used
    if params_with_trait_generic.is_empty() {
        return vec![];
    }

    let (pos, trait_type_definition) = params_with_trait_generic.first().unwrap();
    let concrete_type = &ann.args_types[*pos];

    let mut res = HashSet::new();

    let constrained_generics = type_assignable_generic_constraints(
        concrete_type,
        trait_type_definition,
        &trait_.generics,
        aliases,
    );

    if let Some(generics_map) = constrained_generics {
        for (generic, constraint) in generics_map.types {
            if let Some(constraint) = constraint {
                let impl_generic = impl_
                    .get_corresponding_generic(&str_to_generics(&trait_.generics), &generic)
                    .unwrap();
                res.insert((constraint, impl_generic, generic));
            }
        }
    }

    res.into_iter()
        .map(|(constraint, impl_generic, trait_generic)| VarInfo {
            impl_generic,
            trait_generic: Some(trait_generic),
            concrete_type: get_concrete_type_with_lifetime(&constraint, &ann.annotations, aliases),
            traits: get_type_traits(&constraint, &ann.annotations, aliases),
        })
        .collect::<Vec<_>>()
}

fn get_generic_constraints_from_type(
    impl_generic: &str,
    impl_: &ImplBody,
    ann: &AnnotationBody,
    aliases: &Aliases,
) -> Vec<VarInfo> {
    if !type_contains(&str_to_type_name(&impl_.type_name), impl_generic) {
        return vec![];
    }

    let constrained_generics = type_assignable_generic_constraints(
        &ann.var_type,
        &impl_.type_name,
        &impl_.impl_generics,
        aliases,
    );

    constrained_generics
        .into_iter()
        .flat_map(|generics_map| generics_map.types.into_iter())
        .filter_map(|(generic, constraint)| constraint.map(|c| (c, generic)))
        .map(|(constraint, impl_generic)| VarInfo {
            impl_generic,
            trait_generic: None,
            concrete_type: get_concrete_type_with_lifetime(&constraint, &ann.annotations, aliases),
            traits: get_type_traits(&constraint, &ann.annotations, aliases),
        })
        .collect::<Vec<_>>()
}

/// Get the traits associated with a type from annotations.
fn get_type_traits(type_: &str, ann: &[Annotation], aliases: &Aliases) -> Vec<String> {
    ann.iter()
        .flat_map(|a| match a {
            Annotation::Trait(t, traits) if type_assignable(type_, t, "", aliases) => {
                traits.clone()
            }
            _ => vec![],
        })
        .collect()
}

/// Get the lifetime associated with a type from annotations.
fn get_concrete_type_with_lifetime(type_: &str, ann: &[Annotation], aliases: &Aliases) -> String {
    let concrete_type = get_concrete_type(type_, aliases);
    let ty = str_to_type_name(&concrete_type);

    let lt_from_ann = ann
        .iter()
        .filter_map(|a| match a {
            Annotation::Lifetime(t, lt) if type_assignable(&concrete_type, t, "", aliases) => {
                Some(lt.clone())
            }
            _ => None,
        })
        .collect::<HashSet<_>>();

    match ty {
        Type::Reference(tr) => {
            let lt_from_ty = tr.clone().lifetime.map(|lt| lt.to_string());

            let lifetimes = lt_from_ann
                .into_iter()
                .chain(lt_from_ty)
                .collect::<HashSet<_>>();

            match lifetimes.len() {
                0 => concrete_type,
                1 => {
                    let mut tr_with_lifetime = tr.clone();
                    tr_with_lifetime.lifetime =
                        lifetimes.iter().next().map(|lt| str_to_lifetime(lt));
                    to_string(&Type::Reference(tr_with_lifetime))
                }
                _ => panic!("Multiple lifetimes found for type {}", type_),
            }
        }
        _ => concrete_type,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use spec_trait_utils::conditions::WhenCondition;

    #[test]
    fn test_get_type_aliases() {
        let ann = vec![
            Annotation::Alias("A".into(), "a1".into()),
            Annotation::Alias("A".into(), "a2".into()),
            Annotation::Alias("B".into(), "b1".into()),
        ];

        let result = get_type_aliases(&ann);

        let a = result.get("A").unwrap();
        assert!(a.contains(&"a1".to_string()));
        assert!(a.contains(&"a2".to_string()));

        let b = result.get("B").unwrap();
        assert_eq!(b.as_slice(), &["b1".to_string()]);
    }

    #[test]
    fn test_get_param_types() {
        let trait_fn: TraitItemFn = syn::parse_str("fn foo(&self, x: T, y: u32);").unwrap();
        let result = get_param_types(&trait_fn);
        assert_eq!(result, vec!["T".to_string(), "u32".to_string()]);
    }

    #[test]
    fn test_get_type_traits() {
        let ann = vec![
            Annotation::Trait("u32".into(), vec!["Copy".into(), "Clone".into()]),
            Annotation::Trait("MyType".into(), vec!["Debug".into()]),
            Annotation::Trait("Vec<_>".into(), vec!["Debug".into()]),
        ];
        let mut aliases = Aliases::new();
        aliases.insert("u32".into(), vec!["MyType".into()]);

        let result = get_type_traits("u32", &ann, &aliases);
        assert_eq!(
            result,
            vec!["Copy".to_string(), "Clone".to_string(), "Debug".to_string()]
        );

        let result = get_type_traits("Vec<_>", &ann, &aliases);
        assert_eq!(result, vec!["Debug".to_string()]);
    }

    #[test]
    fn test_get_vars() {
        let impl_body = ImplBody::try_from((
            syn
                ::parse_str::<TokenStream>(
                    "impl<W, T, U: Debug, V> MyTrait<T, U> for V { fn foo(&self, x: T, y: u32, z: Vec<U>) {} }"
                )
                .unwrap(),
            None,
        )).unwrap();

        let trait_body = TraitBody::try_from(
            syn::parse_str::<TokenStream>(
                "trait MyTrait<A, B> { fn foo(&self, x: A, y: u32, z: Vec<B>); }",
            )
            .unwrap(),
        )
        .unwrap()
        .specialize(&impl_body);

        let ann = AnnotationBody {
            fn_: "foo".to_string(),
            args_types: vec![
                "i32".to_string(),
                "u32".to_string(),
                "Vec<&'static i32>".to_string(),
            ],
            args: vec!["1i32".to_string(), "2u32".to_string(), "vec![]".to_string()],
            var: "x".to_string(),
            var_type: "MyType".to_string(),
            annotations: vec![Annotation::Trait("i32".into(), vec!["Debug".into()])],
        };

        let aliases = Aliases::new();

        let result = get_vars(&ann, &impl_body, &trait_body, &aliases);

        assert_eq!(result.len(), 3);
        let t = result.iter().find(|v| v.impl_generic == "T").unwrap();
        let u = result.iter().find(|v| v.impl_generic == "U").unwrap();
        let v = result.iter().find(|v| v.impl_generic == "V").unwrap();
        assert_eq!(
            t,
            &(VarInfo {
                impl_generic: "T".to_string(),
                trait_generic: Some("A".to_string()),
                concrete_type: "i32".to_string(),
                traits: vec!["Debug".to_string()],
            })
        );
        assert_eq!(
            u,
            &(VarInfo {
                impl_generic: "U".to_string(),
                trait_generic: Some("B".to_string()),
                concrete_type: "& 'static i32".to_string(),
                traits: vec![],
            })
        );
        assert_eq!(
            v,
            &(VarInfo {
                impl_generic: "V".to_string(),
                trait_generic: None,
                concrete_type: "MyType".to_string(),
                traits: vec![],
            })
        );
    }

    #[test]
    fn test_get_vars_different_formats() {
        let impl_body = ImplBody::try_from((
            syn
                ::parse_str::<TokenStream>(
                    "impl<T, U, V, W, X, Y> MyTrait<T, U, W, X> for Vec<Y> { fn foo(&self, x: &T, y: (String, X, i32), z: &[U], w: W) {} }"
                )
                .unwrap(),
            Some(
                WhenCondition::All(
                    vec![
                        WhenCondition::Type("W".into(), "Vec<V>".into()),
                        WhenCondition::Trait("V".into(), vec!["Debug".into()])
                    ]
                )
            ),
        )).unwrap();

        let trait_body = TraitBody::try_from(
            syn
                ::parse_str::<TokenStream>(
                    "trait MyTrait<A, B, C, D> { fn foo(&self, x: &A, y: (String, D, i32), z: &[B], w: C); }"
                )
                .unwrap()
        )
            .unwrap()
            .specialize(&impl_body);

        let ann = AnnotationBody {
            fn_: "foo".to_string(),
            args_types: vec![
                "&&i32".to_string(),
                "(String, u32, i32)".to_string(),
                "&[u32]".to_string(),
                "&'static Vec<i32>".to_string(),
            ],
            args: vec![
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "w".to_string(),
            ],
            var: "x".to_string(),
            var_type: "Vec<MyType>".to_string(),
            annotations: vec![
                Annotation::Trait("&i32".into(), vec!["Debug".into()]),
                Annotation::Lifetime("&i32".into(), "'a".into()),
            ],
        };

        let aliases = Aliases::new();

        let result = get_vars(&ann, &impl_body, &trait_body, &aliases);
        println!("{:#?}", result);

        assert_eq!(result.len(), 5);
        let t = result.iter().find(|v| v.impl_generic == "T").unwrap();
        let u = result.iter().find(|v| v.impl_generic == "U").unwrap();
        let v = result.iter().find(|v| v.impl_generic == "V");
        let w = result.iter().find(|v| v.impl_generic == "W").unwrap();
        let x = result.iter().find(|v| v.impl_generic == "X").unwrap();
        let y = result.iter().find(|v| v.impl_generic == "Y").unwrap();
        assert_eq!(
            t,
            &(VarInfo {
                impl_generic: "T".to_string(),
                trait_generic: Some("A".to_string()),
                concrete_type: "& 'a i32".to_string(),
                traits: vec!["Debug".to_string()],
            })
        );
        assert_eq!(
            u,
            &(VarInfo {
                impl_generic: "U".to_string(),
                trait_generic: Some("B".to_string()),
                concrete_type: "u32".to_string(),
                traits: vec![],
            })
        );
        assert!(v.is_none());
        assert_eq!(
            w,
            &(VarInfo {
                impl_generic: "W".to_string(),
                trait_generic: Some("C".to_string()),
                concrete_type: "& 'static Vec < i32 >".to_string(),
                traits: vec![],
            })
        );
        assert_eq!(
            x,
            &(VarInfo {
                impl_generic: "X".to_string(),
                trait_generic: Some("D".to_string()),
                concrete_type: "u32".to_string(),
                traits: vec![],
            })
        );
        assert_eq!(
            y,
            &(VarInfo {
                impl_generic: "Y".to_string(),
                trait_generic: None,
                concrete_type: "MyType".to_string(),
                traits: vec![],
            })
        );
    }
}
