use std::collections::HashSet;

use spec_trait_utils::conversions::{ str_to_generics, str_to_type_name, to_string };
use spec_trait_utils::impls::ImplBody;
use spec_trait_utils::parsing::get_generics;
use spec_trait_utils::traits::TraitBody;
use syn::{ FnArg, TraitItemFn };
use crate::annotations::{ Annotation, AnnotationBody };
use spec_trait_utils::types::{
    get_concrete_type,
    replace_type,
    types_equal,
    types_equal_generic_constraints,
    Aliases,
};
use crate::SpecBody;

#[derive(Debug, Clone, PartialEq)]
pub struct VarInfo {
    /// if the trait parameter is generic, this is the corresponding generic in the impl
    pub impl_generic: String,
    /// concrete type with which the fn was called
    pub concrete_type: String,
    /// traits implemented by the concrete_type, got from annotations
    pub traits: Vec<String>,
}

#[derive(Debug)]
pub struct VarBody {
    /// map from concrete type to type aliases
    pub aliases: Aliases,
    /// list of impl generics
    pub generics: HashSet<String>,
    /// map from type definition (e.g. generic) to VarInfo
    pub vars: Vec<VarInfo>,
}

impl From<&SpecBody> for VarBody {
    fn from(spec: &SpecBody) -> Self {
        let aliases = get_type_aliases(&spec.annotations.annotations);
        let generics = get_generics(&spec.impl_.impl_generics);
        let vars = get_vars(&spec.annotations, &spec.impl_, &spec.trait_, &aliases);
        VarBody { aliases, generics, vars }
    }
}

fn get_type_aliases(ann: &[Annotation]) -> Aliases {
    let mut aliases = Aliases::new();

    for a in ann {
        if let Annotation::Alias(type_, alias) = a {
            aliases.entry(type_.clone()).or_default().push(alias.clone());
        }
    }

    aliases
}

fn get_vars(
    ann: &AnnotationBody,
    impl_: &ImplBody,
    trait_: &TraitBody,
    aliases: &Aliases
) -> Vec<VarInfo> {
    get_generics(&impl_.impl_generics)
        .iter()
        .flat_map(|g| {
            match trait_.get_corresponding_generic(&str_to_generics(&impl_.impl_generics), g) {
                // get type
                Some(trait_generic) =>
                    get_generic_constraints(&trait_generic, trait_, impl_, ann, aliases),

                // get from specialized instead
                None => {
                    let trait_generic = trait_.specialized
                        .as_ref()
                        .unwrap()
                        .get_corresponding_generic(
                            &str_to_generics(&impl_.specialized.as_ref().unwrap().impl_generics),
                            g
                        );

                    if let Some(trait_generic) = trait_generic {
                        get_generic_constraints(
                            &trait_generic,
                            trait_.specialized.as_ref().unwrap(),
                            impl_.specialized.as_ref().unwrap(),
                            ann,
                            aliases
                        )
                    } else {
                        vec![]
                    }
                }
            }
        })
        .collect()
}

/**
    Get the parameter types from a trait function.
    # Example
    `fn foo(&self, x: T, y: u32);` returns `vec!["T", "u32"]`
 */
fn get_param_types(trait_fn: &TraitItemFn) -> Vec<String> {
    trait_fn.sig.inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                FnArg::Typed(pat_type) => Some(to_string(&pat_type.ty)),
                _ => None,
            }
        })
        .collect()
}

fn get_generic_constraints(
    trait_generic: &str,
    trait_: &TraitBody,
    impl_: &ImplBody,
    ann: &AnnotationBody,
    aliases: &Aliases
) -> Vec<VarInfo> {
    let trait_fn = trait_.find_fn(&ann.fn_, ann.args.len()).unwrap();
    let param_types = get_param_types(&trait_fn);

    let params_with_trait_generic = param_types
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            let mut param_type = str_to_type_name(p);
            let original = param_type.clone();
            let replacement = str_to_type_name("__GENERIC__");
            replace_type(&mut param_type, trait_generic, &replacement);
            if to_string(&param_type) != to_string(&original) {
                Some((i, to_string(&original)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let (pos, trait_type_definition) = params_with_trait_generic.first().unwrap();
    let concrete_type = &ann.args_types[*pos];

    let mut res = HashSet::new();

    let constrained_generics = types_equal_generic_constraints(
        concrete_type,
        trait_type_definition,
        &get_generics(&trait_.generics),
        aliases
    );

    if let Some(generics_map) = constrained_generics {
        for (generic, constraint) in generics_map {
            if let Some(constraint) = constraint {
                let impl_generic = impl_
                    .get_corresponding_generic(&str_to_generics(&trait_.generics), &generic)
                    .unwrap();
                res.insert((constraint, impl_generic));
            }
        }
    }

    res.into_iter()
        .map(|(constraint, generic)| VarInfo {
            impl_generic: generic,
            concrete_type: get_concrete_type(&constraint, aliases),
            traits: get_type_traits(&constraint, &ann.annotations, aliases),
        })
        .collect::<Vec<_>>()
}

/// Get the traits associated with a type from annotations.
fn get_type_traits(type_: &str, ann: &[Annotation], aliases: &Aliases) -> Vec<String> {
    ann.iter()
        .flat_map(|a| {
            match a {
                Annotation::Trait(t, traits) if types_equal(t, type_, &HashSet::new(), aliases) =>
                    traits.clone(),
                _ => vec![],
            }
        })
        .collect()
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
            Annotation::Alias("B".into(), "b1".into())
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
            Annotation::Trait("Vec<_>".into(), vec!["Debug".into()])
        ];
        let mut aliases = Aliases::new();
        aliases.insert("u32".into(), vec!["MyType".into()]);

        let result = get_type_traits("u32", &ann, &aliases);
        assert_eq!(result, vec!["Copy".to_string(), "Clone".to_string(), "Debug".to_string()]);

        let result = get_type_traits("Vec<_>", &ann, &aliases);
        assert_eq!(result, vec!["Debug".to_string()]);
    }

    #[test]
    fn test_get_vars() {
        let impl_body = ImplBody::try_from((
            syn
                ::parse_str::<TokenStream>(
                    "impl<T, U: Debug> MyTrait<T, U> for MyType { fn foo(&self, x: T, y: u32, z: Vec<U>) {} }"
                )
                .unwrap(),
            None,
        )).unwrap();

        let trait_body = TraitBody::try_from(
            syn
                ::parse_str::<TokenStream>(
                    "trait MyTrait<A, B> { fn foo(&self, x: A, y: u32, z: Vec<B>); }"
                )
                .unwrap()
        )
            .unwrap()
            .specialize(&impl_body);

        let ann = AnnotationBody {
            fn_: "foo".to_string(),
            args_types: vec!["i32".to_string(), "u32".to_string(), "Vec<i32>".to_string()],
            args: vec!["1i32".to_string(), "2u32".to_string(), "vec![]".to_string()],
            var: "x".to_string(),
            var_type: "MyType".to_string(),
            annotations: vec![Annotation::Trait("i32".into(), vec!["Debug".into()])],
        };

        let aliases = Aliases::new();

        let result = get_vars(&ann, &impl_body, &trait_body, &aliases);

        assert_eq!(result.len(), 2);
        let t = result
            .iter()
            .find(|v| v.impl_generic == "T")
            .unwrap();
        let u = result
            .iter()
            .find(|v| v.impl_generic == "U")
            .unwrap();
        assert_eq!(
            t,
            &(VarInfo {
                impl_generic: "T".to_string(),
                concrete_type: "i32".to_string(),
                traits: vec!["Debug".to_string()],
            })
        );
        assert_eq!(
            u,
            &(VarInfo {
                impl_generic: "U".to_string(),
                concrete_type: "i32".to_string(),
                traits: vec!["Debug".to_string()],
            })
        );
    }

    #[test]
    fn test_get_vars_different_formats() {
        let impl_body = ImplBody::try_from((
            syn
                ::parse_str::<TokenStream>(
                    "impl<T, U, V, W, X> MyTrait<T, U, W, D> for MyType { fn foo(&self, x: &T, y: (String, X, i32), z: &[U], w: W) {} }"
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
                "&i32".to_string(),
                "(String, u32, i32)".to_string(),
                "&[u32]".to_string(),
                "Vec<i32>".to_string()
            ],
            args: vec!["x".to_string(), "y".to_string(), "z".to_string(), "w".to_string()],
            var: "x".to_string(),
            var_type: "MyType".to_string(),
            annotations: vec![Annotation::Trait("i32".into(), vec!["Debug".into()])],
        };

        let aliases = Aliases::new();

        println!("impl_body: {:#?}", impl_body);
        println!("trait_body: {:#?}", trait_body);

        let result = get_vars(&ann, &impl_body, &trait_body, &aliases);
        println!("result: {:#?}", result);

        assert_eq!(result.len(), 5);
        let t = result
            .iter()
            .find(|v| v.impl_generic == "T")
            .unwrap();
        let u = result
            .iter()
            .find(|v| v.impl_generic == "U")
            .unwrap();
        let v = result
            .iter()
            .find(|v| v.impl_generic == "V")
            .unwrap();
        let w = result
            .iter()
            .find(|v| v.impl_generic == "W")
            .unwrap();
        let d = result
            .iter()
            .find(|v| v.impl_generic == "D")
            .unwrap();
        assert_eq!(
            t,
            &(VarInfo {
                impl_generic: "T".to_string(),
                concrete_type: "i32".to_string(),
                traits: vec!["Debug".to_string()],
            })
        );
        assert_eq!(
            u,
            &(VarInfo {
                impl_generic: "U".to_string(),
                concrete_type: "u32".to_string(),
                traits: vec![],
            })
        );
        assert_eq!(
            v,
            &(VarInfo {
                impl_generic: "V".to_string(),
                concrete_type: "i32".to_string(),
                traits: vec!["Debug".to_string()],
            })
        );
        assert_eq!(
            w,
            &(VarInfo {
                impl_generic: "W".to_string(),
                concrete_type: "Vec < i32 >".to_string(),
                traits: vec![],
            })
        );
        assert_eq!(
            d,
            &(VarInfo {
                impl_generic: "D".to_string(),
                concrete_type: "u32".to_string(),
                traits: vec![],
            })
        );
    }
}
