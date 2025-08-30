use crate::annotations::{ AnnotationBody };
use crate::generics::{ get_var_info_for_trait, VarInfo };
use spec_trait_utils::conversions::{
    str_to_expr,
    str_to_generics,
    str_to_trait_name,
    str_to_type_name,
};
use spec_trait_utils::traits::TraitBody;
use spec_trait_utils::conditions::WhenCondition;
use spec_trait_utils::impls::ImplBody;
use proc_macro2::TokenStream as TokenStream2;
use std::cmp::Ordering;
use crate::constraints::{ cmp_constraints, Constraint, Constraints };

pub fn get_most_specific_impl(
    impls: &[ImplBody],
    traits: &[TraitBody],
    ann: &AnnotationBody
) -> (ImplBody, Constraints) {
    let mut filtered_impls = impls
        .iter()
        .filter_map(|impl_| {
            let trait_ = traits
                .iter()
                .find(|tr| tr.name == impl_.trait_name)
                .unwrap_or_else(|| panic!("Trait {} not found", impl_.trait_name));

            let fn_info = get_var_info_for_trait(ann, trait_);

            match &impl_.condition {
                Some(condition) => {
                    let (satisfied, c) = satisfies_condition(
                        condition,
                        &fn_info,
                        &Constraints::default()
                    );

                    if satisfied {
                        Some((impl_.clone(), c))
                    } else {
                        None
                    }
                }
                None => Some((impl_.clone(), Constraints::default())),
            }
        })
        .collect::<Vec<_>>();

    filtered_impls.sort_by(|(_, a), (_, b)| cmp_constraints(a, b));

    let most_specific = filtered_impls.last();

    if let [.., second, last] = filtered_impls.as_slice() {
        if cmp_constraints(&last.1, &second.1) == Ordering::Equal {
            panic!("Ambiguous implementation: multiple implementations are equally specific");
        }
    }

    let (impl_ref, constraints) = most_specific.expect("No valid implementation found");
    (impl_ref.clone(), constraints.clone())
}

fn get_concrete_type(type_or_alias: &str, var: &[VarInfo]) -> String {
    if let Some(alias) = var.iter().find(|v| v.type_aliases.contains(&type_or_alias.to_string())) {
        alias.concrete_type.clone()
    } else {
        type_or_alias.to_string()
    }
}

fn satisfies_condition(
    cond: &WhenCondition,
    vars: &Vec<VarInfo>,
    constraints: &Constraints
) -> (bool, Constraints) {
    match cond {
        WhenCondition::Type(generic, type_) => {
            let concrete_type = get_concrete_type(type_, vars);
            let generic_var = vars.iter().find(|v: &_| v.type_definition == generic.to_string());
            let concrete_type_var = vars.iter().find(|v: &_| v.concrete_type == concrete_type);

            let mut new_constraints = constraints.clone();
            let constraint = new_constraints
                .entry(generic.clone())
                .or_insert_with(Constraint::default);
            constraint.type_ = Some(concrete_type.clone());

            let violates_constraints =
                // generic parameter is not present in the function parameters or the type does not match
                generic_var.is_none_or(|v| { concrete_type != v.concrete_type }) ||
                // generic parameter is forbidden to be assigned to this type
                constraint.not_types.contains(&concrete_type.clone().into()) ||
                // generic parameter should implement a trait that the type does not implement
                concrete_type_var.is_none_or(|v|
                    constraint.traits.iter().any(|t| !v.traits.contains(t))
                );

            (!violates_constraints, new_constraints)
        }
        WhenCondition::Trait(generic, traits) => {
            let generic_var = vars.iter().find(|v: &_| v.type_definition == generic.to_string());

            let mut new_constraints = constraints.clone();
            let constraint = new_constraints
                .entry(generic.clone())
                .or_insert_with(Constraint::default);
            constraint.traits.extend(traits.clone());

            let violates_constraints =
                // generic parameter is not present in the function parameters or the trait does not match
                generic_var.is_none_or(|v| traits.iter().any(|t| !v.traits.contains(t))) ||
                // generic parameter is forbidden to be implement one of the traits
                constraint.not_traits.iter().any(|t| traits.contains(t)) ||
                // generic parameter is already assigned to a type that does not implement one of the traits
                constraint.type_.as_ref().is_some_and(|ty| {
                    let concrete_type_var = vars.iter().find(|v: &_| v.concrete_type == *ty);
                    concrete_type_var.is_none_or(|v| traits.iter().any(|tr| !v.traits.contains(tr)))
                });

            (!violates_constraints, new_constraints)
        }
        // make sure all the inner conditions are satisfied
        WhenCondition::All(inner) => {
            let mut new_constraints = constraints.clone();

            let satisfied = inner.iter().all(|cond| {
                let (is_satisfied, nc) = satisfies_condition(cond, vars, &new_constraints);
                new_constraints = nc;
                is_satisfied
            });

            (satisfied, new_constraints)
        }
        // returns the most specific of all the consraints that satisfy the inner conditions
        WhenCondition::Any(inner) => {
            let mut satisfied = false;
            let mut new_constraints = constraints.clone();

            for cond in inner {
                let (is_satisfied, nc) = satisfies_condition(cond, vars, constraints);
                if is_satisfied {
                    satisfied = true;

                    if cmp_constraints(&nc, &new_constraints) == Ordering::Greater {
                        new_constraints = nc;
                    }
                }
            }

            (satisfied, new_constraints)
        }
        // negates the constraints on the inner condition
        WhenCondition::Not(inner) => {
            let (satisfied, nc) = satisfies_condition(inner, vars, constraints);

            let new_constraints = nc
                .into_iter()
                .map(|(generic, constraint)| (generic, constraint.reverse()))
                .collect::<Constraints>();

            (!satisfied, new_constraints)
        }
    }
}

pub fn create_spec(impl_: &ImplBody, generics_types: &str, ann: &AnnotationBody) -> TokenStream2 {
    let type_ = str_to_type_name(&impl_.type_name);
    let trait_ = str_to_trait_name(&impl_.spec_trait_name);
    let generics = str_to_generics(generics_types);
    let fn_ = str_to_expr(&ann.fn_);
    let var = str_to_expr(("&".to_owned() + &ann.var).as_str());
    let args = ann.args
        .iter()
        .map(|arg| str_to_expr(arg))
        .collect::<Vec<_>>();

    let all_args = std::iter::once(var.clone()).chain(args.iter().cloned()).collect::<Vec<_>>();

    quote::quote! {
        <#type_ as #trait_ #generics>::#fn_(#(#all_args),*)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satisfies_condition() {
        let condition = WhenCondition::All(
            vec![
                WhenCondition::Type("T".into(), "MyType".into()),
                WhenCondition::Type("T".into(), "MyOtherType".into()),
                WhenCondition::Trait("T".into(), vec!["MyTrait".into()])
            ]
        );
        let fn_args = vec![VarInfo {
            type_aliases: vec!["MyOtherType".into()],
            type_definition: "T".into(),
            concrete_type: "MyType".into(),
            traits: vec!["MyTrait".into()],
        }];

        let (satisfies, constraints) = satisfies_condition(
            &condition,
            &fn_args,
            &Constraints::default()
        );

        assert!(satisfies);

        let c = constraints.get("T".into()).unwrap();
        assert_eq!(c.type_, Some("MyType".into()));
        assert!(c.traits.contains(&"MyTrait".into()));
    }

    #[test]
    fn type_not_respected() {
        let condition = WhenCondition::Type("T".into(), "MyType".into());
        let fn_args = vec![VarInfo {
            type_aliases: vec![],
            type_definition: "T".into(),
            concrete_type: "MyOtherType".into(),
            traits: vec![],
        }];

        let (satisfies, _) = satisfies_condition(&condition, &fn_args, &Constraints::default());

        assert!(!satisfies);
    }

    #[test]
    fn trait_not_respected() {
        let condition = WhenCondition::Trait("T".into(), vec!["MyTrait".into()]);
        let fn_args = vec![VarInfo {
            type_aliases: vec![],
            type_definition: "T".into(),
            concrete_type: "MyType".into(),
            traits: vec![],
        }];

        let (satisfies, _) = satisfies_condition(&condition, &fn_args, &Constraints::default());

        assert!(!satisfies);
    }

    #[test]
    fn type_forbidden() {
        let condition = WhenCondition::All(
            vec![
                WhenCondition::Type("T".into(), "MyType".into()),
                WhenCondition::Not(Box::new(WhenCondition::Type("T".into(), "MyType".into())))
            ]
        );
        let fn_args = vec![VarInfo {
            type_aliases: vec![],
            type_definition: "T".into(),
            concrete_type: "MyType".into(),
            traits: vec![],
        }];

        let (satisfies, _) = satisfies_condition(&condition, &fn_args, &Constraints::default());

        assert!(!satisfies);
    }

    #[test]
    fn trait_forbidden() {
        let condition = WhenCondition::All(
            vec![
                WhenCondition::Trait("T".into(), vec!["MyTrait".into()]),
                WhenCondition::Not(
                    Box::new(WhenCondition::Trait("T".into(), vec!["MyTrait".into()]))
                )
            ]
        );
        let fn_args = vec![VarInfo {
            type_aliases: vec![],
            type_definition: "T".into(),
            concrete_type: "MyType".into(),
            traits: vec!["MyTrait".into()],
        }];

        let (satisfies, _) = satisfies_condition(&condition, &fn_args, &Constraints::default());

        assert!(!satisfies);
    }
}
