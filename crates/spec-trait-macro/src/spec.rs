use crate::annotations::{ Annotation, AnnotationBody, get_type_aliases, get_type_traits };
use crate::cache::Impl;
use crate::conditions::WhenCondition;
use crate::conversions::{ str_to_expr, str_to_generics, str_to_trait, str_to_type };
use crate::traits::{ find_fn, get_param_types, TraitBody };
use proc_macro2::TokenStream as TokenStream2;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct VarInfo {
    type_definition: String,
    concrete_type: String,
    aliases: Vec<String>,
    traits: Vec<String>,
}

#[derive(Debug, Clone)]
struct FnInfo {
    var: VarInfo,
    fn_name: String,
    args: Vec<VarInfo>,
}

pub fn get_most_specific_impl(
    impls: &Vec<Impl>,
    traits: &Vec<TraitBody>,
    ann: &AnnotationBody
) -> (Impl, Vec<WhenCondition>) {
    let mut filtered_impls: Vec<_> = impls
        .iter()
        .filter_map(|impl_| {
            let trait_ = traits
                .iter()
                .find(|tr| tr.name == impl_.trait_name)
                .unwrap_or_else(|| panic!("Trait {} not found", impl_.trait_name));

            let fn_info = get_fn_info(&ann, trait_);

            match &impl_.condition {
                Some(condition) => {
                    let (satisfied, c) = satisfies_condition(condition, &fn_info, &vec![]);
                    if satisfied {
                        Some((impl_.clone(), c))
                    } else {
                        None
                    }
                }
                None => Some((impl_.clone(), vec![])),
            }
        })
        .collect();

    filtered_impls.sort_by(|(_, a), (_, b)| compare_constraints(&a, &b));

    let most_specific = filtered_impls.last();

    if let [.., second, last] = filtered_impls.as_slice() {
        if compare_constraints(&last.1, &second.1) == Ordering::Equal {
            panic!("Ambiguous implementation: multiple implementations are equally specific");
        }
    }

    let (impl_ref, conditions) = most_specific.unwrap_or_else(||
        panic!("No valid implementation found")
    );
    return (impl_ref.clone(), conditions.clone());
}

fn get_fn_info(ann: &AnnotationBody, trait_: &TraitBody) -> FnInfo {
    let trait_fn = find_fn(trait_, &ann.fn_, ann.args.len()).unwrap_or_else(||
        panic!("Function {} not found in trait {}", ann.fn_, trait_.name)
    );

    let args_types_definition = get_param_types(&trait_fn);

    FnInfo {
        var: get_var_info(&ann.var_type, &ann.var_type, ann),
        fn_name: ann.fn_.clone(),
        args: ann.args_types
            .iter()
            .enumerate()
            .map(|(i, arg)| get_var_info(&arg, &args_types_definition[i], ann))
            .collect(),
    }
}

fn get_var_info(type_: &str, type_definition: &str, ann: &AnnotationBody) -> VarInfo {
    let types = get_type_aliases(type_, &ann.annotations);
    let traits = get_type_traits(type_, &ann.annotations);
    VarInfo {
        type_definition: type_definition.to_string(),
        concrete_type: type_.to_string(),
        aliases: types,
        traits,
    }
}

fn compare_constraints(a: &Vec<WhenCondition>, b: &Vec<WhenCondition>) -> Ordering {
    let a_type = a.iter().any(|c| matches!(c, WhenCondition::Type(_, _)));
    let b_type = b.iter().any(|c| matches!(c, WhenCondition::Type(_, _)));
    let a_trait = a
        .iter()
        .filter_map(|c| {
            if let WhenCondition::Trait(_, traits) = c { Some(traits.len()) } else { None }
        })
        .sum::<usize>();
    let b_trait = b
        .iter()
        .filter_map(|c| {
            if let WhenCondition::Trait(_, traits) = c { Some(traits.len()) } else { None }
        })
        .sum::<usize>();
    let a_not_type = a
        .iter()
        .filter(
            |c|
                matches!(c, WhenCondition::Not(inner) if matches!(&**inner, WhenCondition::Type(_, _)))
        )
        .count();
    let b_not_type = b
        .iter()
        .filter(
            |c|
                matches!(c, WhenCondition::Not(inner) if matches!(&**inner, WhenCondition::Type(_, _)))
        )
        .count();
    let a_not_trait = a
        .iter()
        .filter_map(|c| {
            if let WhenCondition::Not(inner) = c {
                if let WhenCondition::Trait(_, traits) = &**inner {
                    Some(traits.len())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum::<usize>();
    let b_not_trait = b
        .iter()
        .filter_map(|c| {
            if let WhenCondition::Not(inner) = c {
                if let WhenCondition::Trait(_, traits) = &**inner {
                    Some(traits.len())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum::<usize>();

    if a_type && !b_type {
        return Ordering::Greater;
    } else if !a_type && b_type {
        return Ordering::Less;
    }

    if a_trait > b_trait {
        return Ordering::Greater;
    } else if a_trait < b_trait {
        return Ordering::Less;
    }

    if a_not_type > b_not_type {
        return Ordering::Greater;
    } else if a_not_type < b_not_type {
        return Ordering::Less;
    }

    if a_not_trait > b_not_trait {
        return Ordering::Greater;
    } else if a_not_trait < b_not_trait {
        return Ordering::Less;
    }

    Ordering::Equal
}

fn get_concrete_type(type_or_alias: &str, var: &Vec<VarInfo>) -> String {
    if let Some(alias) = var.iter().find(|v| v.aliases.contains(&type_or_alias.to_string())) {
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
        .chain(
            std::iter::once(Annotation::Trait(var[0].concrete_type.clone(), var[0].traits.clone()))
        )
        .collect()
}

fn satisfies_condition(
    cond: &WhenCondition,
    fn_: &FnInfo,
    constraints: &Vec<WhenCondition> // only type, trait, not
) -> (bool, Vec<WhenCondition>) {
    match cond {
        WhenCondition::Type(generic, type_) => {
            let mut new_constraints = constraints.clone();
            new_constraints.push(
                WhenCondition::Type(generic.clone(), get_concrete_type(type_, &fn_.args))
            );

            let var_ = &fn_.args.iter().find(|v| v.type_definition == *generic);

            if
                // generic parameter is not present in the function parameters or the type does not match
                var_.is_none_or(|v| {
                    get_concrete_type(type_, &fn_.args) !=
                        get_concrete_type(&v.concrete_type, &fn_.args)
                }) ||
                constraints.iter().any(|c| {
                    match c {
                        // generic parameter is forbidden to be assigned to this type
                        WhenCondition::Not(inner) =>
                            match &**inner {
                                WhenCondition::Type(g, t) if *g == *generic =>
                                    fn_.args
                                        .iter()
                                        .any(|v| {
                                            v.concrete_type == *t &&
                                                (v.aliases.iter().any(|alias| *alias == *type_) ||
                                                    v.concrete_type == *type_)
                                        }),
                                _ => false,
                            }
                        // generic parameter is already assigned to another type
                        WhenCondition::Type(g, t) if *g == *generic => {
                            get_concrete_type(type_, &fn_.args) != get_concrete_type(&t, &fn_.args)
                        }
                        // generic parameter should implement a trait that the type does not implement
                        WhenCondition::Trait(g, t) if *g == *generic =>
                            t
                                .iter()
                                .any(|trait_| {
                                    !get_type_traits(
                                        &get_concrete_type(type_, &fn_.args),
                                        &var_info_to_annotations(&fn_.args)
                                    ).contains(trait_)
                                }),
                        _ => false,
                    }
                })
            {
                (false, new_constraints)
            } else {
                (true, new_constraints)
            }
        }
        WhenCondition::Trait(generic, traits) => {
            let mut new_constraints = constraints.clone();
            new_constraints.push(cond.clone());

            let var_ = &fn_.args.iter().find(|v| v.type_definition == *generic);

            if
                // generic parameter is not present in the function parameters or the trait does not match
                var_.is_none_or(|v| traits.iter().any(|trait_| !v.traits.contains(trait_))) ||
                constraints.iter().any(|c| {
                    match c {
                        // generic parameter is forbidden to be implement one of the traits
                        WhenCondition::Not(inner) =>
                            match &**inner {
                                WhenCondition::Trait(g, t) if *g == *generic => {
                                    traits.iter().any(|trait_| t.contains(trait_))
                                }
                                _ => false,
                            }
                        // generic parameter is already assigned to a type that does not implement one of the traits
                        WhenCondition::Type(g, type_) if *g == *generic => {
                            traits
                                .iter()
                                .any(|trait_| {
                                    !get_type_traits(
                                        &get_concrete_type(type_, &fn_.args),
                                        &var_info_to_annotations(&fn_.args)
                                    ).contains(trait_)
                                })
                        }
                        _ => false,
                    }
                })
            {
                (false, new_constraints)
            } else {
                (true, new_constraints)
            }
        }
        WhenCondition::All(inner) => {
            let mut satisfied = true;
            let mut new_constraints = constraints.clone();
            for c in inner {
                let (s, nc) = satisfies_condition(c, fn_, &new_constraints);
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
                let (s, nc) = satisfies_condition(c, fn_, constraints);
                if s {
                    satisfied = true;
                    let cmp = compare_constraints(&nc, &new_constraints);
                    new_constraints = if cmp == Ordering::Greater { nc } else { new_constraints };
                }
            }
            (satisfied, new_constraints)
        }
        WhenCondition::Not(inner) => {
            let mut new_constraints;
            let (satisfied, nc) = satisfies_condition(inner, fn_, constraints);
            new_constraints = nc
                .iter()
                .map(|c| WhenCondition::Not(Box::new(c.clone())))
                .collect();
            (!satisfied, new_constraints)
        }
    }
}

pub fn create_spec(impl_: &Impl, generics_types: &String, ann: &AnnotationBody) -> TokenStream2 {
    let type_ = str_to_type(&impl_.type_name);
    let trait_ = str_to_trait(&impl_.spec_trait_name);
    let generics = str_to_generics(generics_types);
    let fn_ = str_to_expr(&ann.fn_);
    let var = str_to_expr((&("&".to_owned() + &ann.var)).as_str());
    let args = ann.args
        .iter()
        .map(|arg| str_to_expr(arg))
        .collect::<Vec<_>>();

    let all_args = std::iter::once(var.clone()).chain(args.iter().cloned()).collect::<Vec<_>>();

    quote::quote! {
        <#type_ as #trait_ #generics>::#fn_(#(#all_args),*)
    }
}
