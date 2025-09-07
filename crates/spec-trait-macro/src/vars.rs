use spec_trait_utils::conversions::str_to_generics;
use spec_trait_utils::impls::ImplBody;
use spec_trait_utils::traits::{ get_param_types, TraitBody };
use crate::annotations::{ Annotation, AnnotationBody };
use spec_trait_utils::types::{ get_concrete_type, types_equal, Aliases };
use crate::SpecBody;

#[derive(Debug, Clone)]
pub struct VarInfo {
    /// if trait_type_definition is generic, this is the concrete type used in the fn call
    pub impl_type_definition: String,
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
            aliases.entry(type_.clone()).or_insert_with(Vec::new).push(alias.clone());
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
                impl_type_definition: impl_
                    .get_corresponding_generic(
                        &str_to_generics(&trait_.generics),
                        &trait_type_definition
                    )
                    .expect("generic in trait not found in impl"),
                concrete_type: get_concrete_type(concrete_type, aliases),
                traits: get_type_traits(concrete_type, &ann.annotations, aliases),
            }
        })
        .collect()
}

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
