use std::collections::HashMap;
use spec_trait_utils::traits::{ find_fn, get_param_types, TraitBody };
use crate::annotations::{ Annotation, AnnotationBody };
use crate::types::{ get_concrete_type, types_equal };

#[derive(Debug, Clone)]
pub struct VarInfo {
    /// type defined in the trait's fn, usually a generic
    pub type_definition: String,
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

pub type Aliases = HashMap<String, Vec<String>>;

impl From<(&AnnotationBody, &TraitBody)> for VarBody {
    fn from((ann, trait_): (&AnnotationBody, &TraitBody)) -> Self {
        let aliases = get_type_aliases(&ann.annotations);
        let vars = get_vars(ann, trait_, &aliases);
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

fn get_vars(ann: &AnnotationBody, trait_: &TraitBody, aliases: &Aliases) -> Vec<VarInfo> {
    let trait_fn = find_fn(trait_, &ann.fn_, ann.args.len()).unwrap();
    let param_types = get_param_types(&trait_fn);

    ann.args_types
        .iter()
        .zip(param_types)
        .map(|(concrete_type, type_definition)| VarInfo {
            type_definition,
            concrete_type: get_concrete_type(concrete_type, aliases),
            traits: get_type_traits(concrete_type, &ann.annotations, aliases),
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
