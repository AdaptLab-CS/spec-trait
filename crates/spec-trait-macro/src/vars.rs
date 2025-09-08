use spec_trait_utils::conversions::{ str_to_generics, to_string };
use spec_trait_utils::impls::ImplBody;
use spec_trait_utils::traits::TraitBody;
use syn::{ FnArg, TraitItemFn };
use crate::annotations::{ Annotation, AnnotationBody };
use spec_trait_utils::types::{ get_concrete_type, types_equal, Aliases };
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

pub struct VarBody {
    /// map from concrete type to type aliases
    pub aliases: Aliases,
    /// map from type definition (e.g. generic) to VarInfo
    pub vars: Vec<VarInfo>,
}

impl From<&SpecBody> for VarBody {
    fn from(spec: &SpecBody) -> Self {
        let aliases = get_type_aliases(&spec.annotations.annotations);
        let vars = get_vars(&spec.annotations, &spec.impl_, &spec.trait_, &aliases);
        VarBody { aliases, vars }
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
    let trait_fn = trait_.find_fn(&ann.fn_, ann.args.len()).unwrap();
    let param_types = get_param_types(&trait_fn);

    ann.args_types
        .iter()
        .zip(param_types)
        .map(|(concrete_type, trait_type_definition)| {
            VarInfo {
                impl_generic: impl_
                    .get_corresponding_generic(
                        &str_to_generics(&trait_.generics),
                        &trait_type_definition
                    )
                    .unwrap_or_default(),
                concrete_type: get_concrete_type(concrete_type, aliases),
                traits: get_type_traits(concrete_type, &ann.annotations, aliases),
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

/// Get the traits associated with a type from annotations.
fn get_type_traits(type_: &str, ann: &[Annotation], aliases: &Aliases) -> Vec<String> {
    ann.iter()
        .flat_map(|a| {
            match a {
                Annotation::Trait(t, traits) if types_equal(t, type_, aliases) => traits.clone(),
                _ => vec![],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;

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
        let trait_body = TraitBody::try_from(
            syn
                ::parse_str::<TokenStream>("trait MyTrait<A> { fn foo(&self, x: A, y: u32); }")
                .unwrap()
        ).unwrap();

        let impl_body = ImplBody::try_from((
            syn
                ::parse_str::<TokenStream>(
                    "impl<T> MyTrait<T> for MyType { fn foo(&self, x: T, y: u32) {} }"
                )
                .unwrap(),
            None,
        )).unwrap();

        let ann = AnnotationBody {
            fn_: "foo".to_string(),
            args_types: vec!["i32".to_string(), "u32".to_string()],
            args: vec!["1i32".to_string(), "2u32".to_string()],
            var: "x".to_string(),
            var_type: "MyType".to_string(),
            annotations: vec![Annotation::Trait("i32".into(), vec!["Debug".into()])],
        };

        let aliases = Aliases::new();

        let result = get_vars(&ann, &impl_body, &trait_body, &aliases);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], VarInfo {
            impl_generic: "T".to_string(),
            concrete_type: "i32".to_string(),
            traits: vec!["Debug".to_string()],
        });
        assert_eq!(result[1], VarInfo {
            impl_generic: "".to_string(),
            concrete_type: "u32".to_string(),
            traits: vec![],
        });
    }
}
