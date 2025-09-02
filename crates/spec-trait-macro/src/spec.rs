use crate::annotations::AnnotationBody;
use crate::vars::VarBody;
use crate::types::{ get_concrete_type, types_equal };
use spec_trait_utils::conversions::{ str_to_expr, str_to_trait_name, str_to_type_name };
use spec_trait_utils::traits::TraitBody;
use spec_trait_utils::conditions::WhenCondition;
use spec_trait_utils::impls::ImplBody;
use proc_macro2::TokenStream;
use std::cmp::Ordering;
use crate::constraints::{ cmp_constraints, Constraint, Constraints };
use quote::quote;

#[derive(Debug, Clone)]
pub struct SpecBody {
    pub impl_: ImplBody,
    pub trait_: TraitBody,
    pub constraints: Constraints,
    pub annotations: AnnotationBody,
}

impl TryFrom<(&Vec<ImplBody>, &Vec<TraitBody>, &AnnotationBody)> for SpecBody {
    type Error = String;

    fn try_from((impls, traits, ann): (&Vec<ImplBody>, &Vec<TraitBody>, &AnnotationBody)) -> Result<
        Self,
        Self::Error
    > {
        let mut satisfied_specs = impls
            .iter()
            .filter_map(|impl_| {
                let trait_ = traits.iter().find(|tr| tr.name == impl_.trait_name)?;
                let default = SpecBody {
                    impl_: impl_.clone(),
                    trait_: trait_.clone(),
                    constraints: Constraints::default(),
                    annotations: ann.clone(),
                };
                get_constraints(default)
            })
            .collect::<Vec<_>>();

        satisfied_specs.sort_by(|a, b| cmp_constraints(&a.constraints, &b.constraints));

        match satisfied_specs.as_slice() {
            [] => Err("No valid implementation found".into()),
            [most_specific] => Ok(most_specific.clone()),
            [.., second, first] => {
                if cmp_constraints(&first.constraints, &second.constraints) == Ordering::Equal {
                    Err("Multiple implementations are equally specific".into())
                } else {
                    Ok(first.clone())
                }
            }
        }
    }
}

/// if the condition is satisfiable, it inserts the constraints and returns the spec body, otherwise return none
fn get_constraints(default: SpecBody) -> Option<SpecBody> {
    match &default.impl_.condition {
        // from spec default
        None => Some(default),
        // from when macro
        Some(cond) => {
            let var = VarBody::from((&default.annotations, &default.trait_));
            let (satisfied, constraints) = satisfies_condition(cond, &var, &default.constraints);

            if satisfied {
                let mut with_constraints = default.clone();
                with_constraints.constraints = constraints;
                Some(with_constraints)
            } else {
                None
            }
        }
    }
}

fn satisfies_condition(
    condition: &WhenCondition,
    var: &VarBody,
    constraints: &Constraints
) -> (bool, Constraints) {
    match condition {
        WhenCondition::Type(generic, type_) => {
            let concrete_type = get_concrete_type(type_, &var.aliases);
            let generic_var = var.vars
                .iter()
                .find(|v: &_| v.type_definition == generic.to_string());
            let concrete_type_var = var.vars
                .iter()
                .find(|v: &_| types_equal(&concrete_type, &v.concrete_type, &var.aliases));

            let mut new_constraints = constraints.clone();
            let constraint = new_constraints
                .entry(generic.clone())
                .or_insert_with(Constraint::default);
            constraint.type_ = Some(concrete_type.clone());

            let violates_constraints =
                // generic parameter is not present in the function parameters or the type does not match
                generic_var.is_none_or(
                    |v| !types_equal(&concrete_type, &v.concrete_type, &var.aliases)
                ) ||
                // generic parameter is forbidden to be assigned to this type
                constraint.not_types.iter().any(|t| types_equal(&concrete_type, t, &var.aliases)) ||
                // generic parameter should implement a trait that the type does not implement
                concrete_type_var.is_none_or(|v|
                    constraint.traits.iter().any(|t| !v.traits.contains(t))
                );

            (!violates_constraints, new_constraints)
        }
        WhenCondition::Trait(generic, traits) => {
            let generic_var = var.vars
                .iter()
                .find(|v: &_| v.type_definition == generic.to_string());

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
                    let concrete_type_var = var.vars
                        .iter()
                        .find(|v| types_equal(&v.concrete_type, ty, &var.aliases));
                    concrete_type_var.is_none_or(|v| traits.iter().any(|tr| !v.traits.contains(tr)))
                });

            (!violates_constraints, new_constraints)
        }
        // make sure all the inner conditions are satisfied
        WhenCondition::All(inner) => {
            let mut new_constraints = constraints.clone();

            let satisfied = inner.iter().all(|cond| {
                let (is_satisfied, nc) = satisfies_condition(cond, var, &new_constraints);
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
                let (is_satisfied, nc) = satisfies_condition(cond, var, constraints);
                satisfied = satisfied || is_satisfied;

                if is_satisfied && cmp_constraints(&nc, &new_constraints) == Ordering::Greater {
                    new_constraints = nc;
                }
            }

            (satisfied, new_constraints)
        }
        // negates the constraints on the inner condition
        WhenCondition::Not(inner) => {
            let (satisfied, nc) = satisfies_condition(inner, var, constraints);

            let new_constraints = nc
                .into_iter()
                .map(|(generic, constraint)| (generic, constraint.reverse()))
                .collect::<Constraints>();

            (!satisfied, new_constraints)
        }
    }
}

impl From<&SpecBody> for TokenStream {
    fn from(spec_body: &SpecBody) -> Self {
        let type_ = str_to_type_name(&spec_body.impl_.type_name);
        let trait_ = str_to_trait_name(&spec_body.impl_.spec_trait_name);
        let generics = get_generics_types(&spec_body);
        let fn_ = str_to_expr(&spec_body.annotations.fn_);
        let var = str_to_expr(("&".to_owned() + &spec_body.annotations.var).as_str());
        let args = spec_body.annotations.args
            .iter()
            .map(|arg| str_to_expr(arg))
            .collect::<Vec<_>>();

        let all_args = std::iter::once(var.clone()).chain(args.iter().cloned()).collect::<Vec<_>>();

        quote! {
            <#type_ as #trait_ #generics>::#fn_(#(#all_args),*)
        }
    }
}

pub fn get_generics_types(spec: &SpecBody) -> TokenStream {
    let generics_without_angle_brackets = &spec.trait_.generics[1..spec.trait_.generics.len() - 1];
    let types = generics_without_angle_brackets
        .split(',')
        .map(|g| get_type(g.trim(), &spec.constraints))
        .map(|t| str_to_type_name(&t))
        .collect::<Vec<_>>();

    if types.is_empty() {
        TokenStream::new()
    } else {
        quote! { <#(#types),*> }
    }
}

fn get_type(generic: &str, constraints: &Constraints) -> String {
    constraints
        .get(generic)
        .and_then(|constraint| constraint.type_.clone())
        .unwrap_or_else(|| "_".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vars::{ Aliases, VarInfo };

    fn get_var_body() -> VarBody {
        let mut aliases = Aliases::new();
        aliases.insert("MyType".to_string(), vec!["MyOtherType".to_string()]);
        VarBody {
            aliases,
            vars: vec![VarInfo {
                type_definition: "T".into(),
                concrete_type: "MyType".into(),
                traits: vec!["MyTrait".into()],
            }],
        }
    }

    #[test]
    fn test_satisfies_condition() {
        let condition = WhenCondition::All(
            vec![
                WhenCondition::Type("T".into(), "MyType".into()),
                WhenCondition::Type("T".into(), "MyOtherType".into()),
                WhenCondition::Trait("T".into(), vec!["MyTrait".into()])
            ]
        );

        let (satisfies, constraints) = satisfies_condition(
            &condition,
            &get_var_body(),
            &Constraints::default()
        );

        assert!(satisfies);

        let c = constraints.get("T".into()).unwrap();
        assert_eq!(c.type_, Some("MyType".into()));
        assert!(c.traits.contains(&"MyTrait".into()));
    }

    #[test]
    fn type_not_respected() {
        let condition = WhenCondition::Type("T".into(), "AnotherType".into());
        let var = get_var_body();

        let (satisfies, _) = satisfies_condition(&condition, &var, &Constraints::default());

        assert!(!satisfies);
    }

    #[test]
    fn trait_not_respected() {
        let condition = WhenCondition::Trait("T".into(), vec!["AnotherTrait".into()]);
        let var = get_var_body();

        let (satisfies, _) = satisfies_condition(&condition, &var, &Constraints::default());

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
        let var = get_var_body();

        let (satisfies, _) = satisfies_condition(&condition, &var, &Constraints::default());

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
        let var = get_var_body();

        let (satisfies, _) = satisfies_condition(&condition, &var, &Constraints::default());

        assert!(!satisfies);
    }
}
