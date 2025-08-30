use crate::annotations::{ Annotation, AnnotationBody, get_type_traits };
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

fn var_info_to_annotations(var: &[VarInfo]) -> Vec<Annotation> {
    var.iter()
        .flat_map(|v| {
            v.type_aliases
                .iter()
                .map(move |alias| Annotation::Alias(v.concrete_type.clone(), alias.clone()))
        })
        .chain(
            std::iter::once(Annotation::Trait(var[0].concrete_type.clone(), var[0].traits.clone()))
        )
        .collect()
}

fn generic_not_present_or_type_not_match(type_: &str, generic: &str, var: &[VarInfo]) -> bool {
    let var_ = var.iter().find(|v| v.type_definition == generic.to_string());

    var_.is_none_or(|v| {
        get_concrete_type(type_, &var) != get_concrete_type(&v.concrete_type, &var)
    })
}

fn generic_not_present_or_trait_not_match(
    traits: &Vec<String>,
    generic: &str,
    var: &[VarInfo]
) -> bool {
    let var_ = var.iter().find(|v| v.type_definition == generic.to_string());

    var_.is_none_or(|v| traits.iter().any(|trait_| !v.traits.contains(trait_)))
}

fn type_forbidden(constraint: &Constraint, type_: &str, var: &[VarInfo]) -> bool {
    constraint.not_types.contains(&get_concrete_type(type_, &var))
}

fn type_not_implementing_trait(
    constraint: &Constraint,
    type_: &Option<&str>,
    var: &[VarInfo]
) -> bool {
    type_.is_some_and(|t|
        constraint.traits
            .iter()
            .any(
                |trait_|
                    !get_type_traits(
                        &get_concrete_type(t, &var),
                        &var_info_to_annotations(&var)
                    ).contains(trait_)
            )
    )
}

fn trait_forbidden(constraint: &Constraint, traits: &[String], var: &[VarInfo]) -> bool {
    constraint.not_traits.iter().any(|t| traits.contains(t))
}

fn satisfies_condition(
    cond: &WhenCondition,
    vars: &Vec<VarInfo>,
    constraints: &Constraints
) -> (bool, Constraints) {
    match cond {
        WhenCondition::Type(generic, type_) => {
            let mut new_constraints = constraints.clone();
            let constraint = new_constraints
                .entry(generic.clone())
                .or_insert_with(Constraint::default);
            constraint.type_ = Some(get_concrete_type(type_, &vars));

            if
                generic_not_present_or_type_not_match(&type_, &generic, &vars) ||
                type_forbidden(&constraint, &type_, &vars) ||
                type_not_implementing_trait(&constraint, &Some(type_), &vars)
            {
                (false, new_constraints)
            } else {
                (true, new_constraints)
            }
        }
        WhenCondition::Trait(generic, traits) => {
            let mut new_constraints = constraints.clone();
            let constraint = new_constraints
                .entry(generic.clone())
                .or_insert_with(Constraint::default);
            constraint.traits.extend(traits.clone());

            if
                generic_not_present_or_trait_not_match(&traits, &generic, &vars) ||
                trait_forbidden(&constraint, traits, &vars) ||
                type_not_implementing_trait(&constraint, &constraint.type_.as_deref(), &vars)
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
                let (s, nc) = satisfies_condition(c, vars, &new_constraints);
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
                let (s, nc) = satisfies_condition(c, vars, constraints);
                if s {
                    satisfied = true;
                    let cmp = cmp_constraints(&nc, &new_constraints);
                    new_constraints = if cmp == Ordering::Greater { nc } else { new_constraints };
                }
            }
            (satisfied, new_constraints)
        }
        WhenCondition::Not(inner) => {
            let (satisfied, nc) = satisfies_condition(inner, vars, constraints);
            let mut new_constraints = Constraints::default();
            for (generic, constraint) in nc {
                new_constraints.insert(generic, constraint.reverse());
            }
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
