use spec_trait_utils::traits::{ find_fn, get_param_types, TraitBody };

use crate::annotations::{ Annotation, AnnotationBody };

#[derive(Debug, Clone)]
pub struct VarInfo {
    /// type defined in the trait's fn, usually a generic
    pub type_definition: String,
    /// concrete type with which the fn was called
    pub concrete_type: String,
    /// aliases for the concrete_type, got from annotations
    pub type_aliases: Vec<String>,
    /// traits implemented by the concrete_type, got from annotations
    pub traits: Vec<String>,
}

pub fn get_var_info_for_trait(ann: &AnnotationBody, trait_: &TraitBody) -> Vec<VarInfo> {
    let trait_fn = find_fn(trait_, &ann.fn_, ann.args.len()).unwrap_or_else(||
        panic!("Function {} not found in trait {}", ann.fn_, trait_.name)
    );

    let param_types = get_param_types(&trait_fn);

    ann.args_types
        .iter()
        .enumerate()
        .map(|(i, type_)| VarInfo {
            type_definition: param_types[i].clone(),
            concrete_type: type_.clone(),
            type_aliases: get_type_aliases(type_, &ann.annotations),
            traits: get_type_traits(type_, &ann.annotations),
        })
        .collect()
}

fn get_type_aliases(type_: &str, ann: &[Annotation]) -> Vec<String> {
    ann.iter()
        .filter_map(|a| {
            match a {
                Annotation::Alias(t, alias) if t == type_ => Some(alias.clone()),
                _ => None,
            }
        })
        .collect()
}

fn get_type_traits(type_: &str, ann: &[Annotation]) -> Vec<String> {
    ann.iter()
        .filter_map(|a| {
            match a {
                Annotation::Trait(t, traits) if t == type_ => Some(traits.clone()),
                _ => None,
            }
        })
        .flatten()
        .collect()
}

pub fn get_concrete_type(type_or_alias: &str, var: &[VarInfo]) -> String {
    if let Some(alias) = var.iter().find(|v| v.type_aliases.contains(&type_or_alias.to_string())) {
        alias.concrete_type.clone()
    } else {
        type_or_alias.to_string()
    }
}
