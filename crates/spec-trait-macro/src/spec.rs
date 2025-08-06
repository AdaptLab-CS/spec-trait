use crate::annotations::{Annotation, AnnotationBody, get_type_aliases, get_type_traits};
use crate::cache::Impl;
use crate::conditions::WhenCondition;
use crate::traits::TraitBody;

struct VarInfo {
    concrete_type: String,
    aliases: Vec<String>,
    traits: Vec<String>,
}

struct FnInfo {
    var: VarInfo,
    args: Vec<VarInfo>,
}

pub fn get_most_specific_impl(
    impls: &Vec<Impl>,
    traits: &Vec<TraitBody>,
    ann: &AnnotationBody,
) -> Impl {
    let fn_info = get_fn_info(&ann);

    let filtered_impls = impls.iter().filter_map(|impl_| match &impl_.condition {
        Some(condition) => {
            let (satisfied, c) = satisfies_condition(condition, &fn_info.args, &vec![]);
            if satisfied { Some((impl_, c)) } else { None }
        }
        None => Some((impl_, vec![])),
    });

    panic!("Not implemented yet");
}

fn get_fn_info(ann: &AnnotationBody) -> FnInfo {
    // TODO: get dynamically from annotations
    let var_type = "ZST";
    let args_types = vec!["i32"];

    FnInfo {
        var: get_var_info(var_type, ann),
        args: args_types
            .into_iter()
            .map(|arg| get_var_info(arg, ann))
            .collect(),
    }
}

fn get_var_info(type_: &str, ann: &AnnotationBody) -> VarInfo {
    let types = get_type_aliases(type_, &ann.annotations);
    let traits = get_type_traits(type_, &ann.annotations);
    VarInfo {
        concrete_type: type_.to_string(),
        aliases: types,
        traits,
    }
}

fn get_concrete_type(type_or_alias: &str, var: &Vec<VarInfo>) -> String {
    if let Some(alias) = var
        .iter()
        .find(|v| v.aliases.contains(&type_or_alias.to_string()))
    {
        alias.concrete_type.clone()
    } else {
        type_or_alias.to_string()
    }
}

fn var_info_to_annotations(var: &Vec<VarInfo>) -> Vec<Annotation> {
    var.iter()
        .flat_map(|v| {
            v.aliases
                .iter()
                .map(move |alias| Annotation::Alias(v.concrete_type.clone(), alias.clone()))
        })
        .chain(std::iter::once(Annotation::Trait(
            var[0].concrete_type.clone(),
            var[0].traits.clone(),
        )))
        .collect()
}

fn satisfies_condition(
    cond: &WhenCondition,
    var: &Vec<VarInfo>,
    constraints: &Vec<WhenCondition>, // only type, trait, not
) -> (bool, Vec<WhenCondition>) {
    match cond {
        WhenCondition::Type(generic, type_) => {
            if constraints.iter().any(|c| match c {
                // generic parameter is forbidden to be assigned to this type
                WhenCondition::Not(inner) => match &**inner {
                    WhenCondition::Type(g, t) if *g == *generic => var.iter().any(|v| {
                        v.concrete_type == *t
                            && (v.aliases.iter().any(|alias| *alias == *type_)
                                || v.concrete_type == *type_)
                    }),
                    _ => false,
                },
                // generic parameter is already assigned to another type
                WhenCondition::Type(g, t) if *g == *generic => {
                    get_concrete_type(type_, var) != get_concrete_type(&t, var)
                }
                // generic parameter should implement a trait that the type does not implement
                WhenCondition::Trait(g, t) if *g == *generic => t.iter().any(|trait_| {
                    !get_type_traits(
                        &get_concrete_type(type_, var),
                        &var_info_to_annotations(var),
                    )
                    .contains(trait_)
                }),
                _ => false,
            }) {
                (false, constraints.clone())
            } else {
                let mut new_constraints = constraints.clone();
                new_constraints.push(WhenCondition::Type(
                    generic.clone(),
                    get_concrete_type(type_, var),
                ));
                (true, new_constraints)
            }
        }
        WhenCondition::Trait(generic, traits) => {
            if constraints.iter().any(|c| match c {
                // generic parameter is forbidden to be implement one of the traits
                WhenCondition::Not(inner) => match &**inner {
                    WhenCondition::Trait(g, t) if *g == *generic => {
                        traits.iter().any(|trait_| t.contains(trait_))
                    }
                    _ => false,
                },
                // generic parameter is already assigned to a type that does not implement one of the traits
                WhenCondition::Type(g, type_) if *g == *generic => traits.iter().any(|trait_| {
                    !get_type_traits(
                        &get_concrete_type(type_, var),
                        &var_info_to_annotations(var),
                    )
                    .contains(trait_)
                }),
                _ => false,
            }) {
                (false, constraints.clone())
            } else {
                let mut new_constraints = constraints.clone();
                new_constraints.push(cond.clone());
                (true, new_constraints)
            }
        }
        WhenCondition::All(inner) => {
            let mut satisfied = true;
            let mut new_constraints = constraints.clone();
            for c in inner {
                let (s, nc) = satisfies_condition(c, var, &new_constraints);
                if !s {
                    satisfied = false;
                    break;
                }
                new_constraints = nc;
            }
            (satisfied, new_constraints)
        }
        WhenCondition::Any(inner) => {
            let mut satisfied = false;
            let mut new_constraints = constraints.clone();
            for c in inner {
                let (s, nc) = satisfies_condition(c, var, constraints);
                if s {
                    satisfied = true;
                    new_constraints = nc;
                    break;
                }
            }
            (satisfied, new_constraints)
        }
        WhenCondition::Not(inner) => {
            let mut new_constraints;
            let (satisfied, nc) = satisfies_condition(inner, var, constraints);
            new_constraints = nc
                .iter()
                .map(|c| WhenCondition::Not(Box::new(c.clone())))
                .collect();
            (!satisfied, new_constraints)
        }
    }
}
