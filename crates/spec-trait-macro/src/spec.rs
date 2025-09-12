use crate::annotations::AnnotationBody;
use crate::vars::VarBody;
use spec_trait_utils::parsing::get_generics;
use spec_trait_utils::types::{ get_concrete_type, types_equal, Aliases };
use spec_trait_utils::conversions::{ str_to_expr, str_to_trait_name, str_to_type_name };
use spec_trait_utils::traits::TraitBody;
use spec_trait_utils::conditions::WhenCondition;
use spec_trait_utils::impls::ImplBody;
use proc_macro2::TokenStream;
use std::cmp::Ordering;
use crate::constraints::{ cmp_constraints, Constraints };
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
                let specialized_trait = trait_.specialize(impl_);
                let default = SpecBody {
                    impl_: impl_.clone(),
                    trait_: specialized_trait,
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
            let var = VarBody::from(&default);
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
            let generic_var = var.vars.iter().find(|v: &_| v.impl_generic == *generic);
            let concrete_type_var = var.vars
                .iter()
                .find(|v: &_|
                    types_equal(&concrete_type, &v.concrete_type, &var.generics, &var.aliases)
                );

            let mut new_constraints = constraints.clone();
            let constraint = new_constraints.entry(generic.clone()).or_default();

            // update the type only if it is more specific than the current one
            if
                constraint.type_
                    .as_ref()
                    .is_none_or(
                        |t|
                            types_equal(&concrete_type, t, &var.generics, &Aliases::default()) &&
                            concrete_type.replace("_", "").len() > t.replace("_", "").len()
                    )
            {
                constraint.type_ = Some(concrete_type.clone());
                constraint.generics = var.generics.clone();
            }

            let violates_constraints =
                // generic parameter is not present in the function parameters or the type does not match
                generic_var.is_none_or(
                    |v| !types_equal(&concrete_type, &v.concrete_type, &var.generics, &var.aliases)
                ) ||
                // generic parameter is forbidden to be assigned to this type
                constraint.not_types
                    .iter()
                    .any(|t| types_equal(&concrete_type, t, &var.generics, &var.aliases)) ||
                // generic parameter should implement a trait that the type does not implement
                concrete_type_var.is_none_or(|v|
                    constraint.traits.iter().any(|t| !v.traits.contains(t))
                );

            (!violates_constraints, new_constraints)
        }
        WhenCondition::Trait(generic, traits) => {
            let generic_var = var.vars.iter().find(|v: &_| v.impl_generic == *generic);

            let mut new_constraints = constraints.clone();
            let constraint = new_constraints.entry(generic.clone()).or_default();
            constraint.traits.extend(traits.clone());
            constraint.generics = var.generics.clone();

            let violates_constraints =
                // generic parameter is not present in the function parameters or the trait does not match
                generic_var.is_none_or(|v| traits.iter().any(|t| !v.traits.contains(t))) ||
                // generic parameter is forbidden to be implement one of the traits
                constraint.not_traits.iter().any(|t| traits.contains(t)) ||
                // generic parameter is already assigned to a type that does not implement one of the traits
                constraint.type_.as_ref().is_some_and(|ty| {
                    let concrete_type_var = var.vars
                        .iter()
                        .find(|v| types_equal(&v.concrete_type, ty, &var.generics, &var.aliases));
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
        let impl_body = spec_body.impl_.specialized.as_ref().expect("ImplBody not specialized");

        let type_ = str_to_type_name(&impl_body.type_name);
        let trait_ = str_to_trait_name(&impl_body.trait_name);
        let generics = get_generics_types(spec_body);
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
    let trait_body = spec.trait_.specialized.as_ref().expect("TraitBody not specialized");

    let types = get_generics::<Vec<_>>(&trait_body.generics)
        .iter()
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
    use std::vec;
    use std::collections::HashSet;
    use crate::annotations::Annotation;
    use crate::vars::VarInfo;
    use crate::constraints::Constraint;

    fn get_var_body() -> VarBody {
        let mut aliases = Aliases::new();
        aliases.insert("MyType".to_string(), vec!["MyOtherType".to_string()]);
        VarBody {
            aliases,
            generics: vec!["T".into()].into_iter().collect(),
            vars: vec![VarInfo {
                impl_generic: "T".into(),
                concrete_type: "MyType".into(),
                traits: vec!["MyTrait".into()],
            }],
        }
    }

    fn get_impl_body(condition: Option<WhenCondition>) -> ImplBody {
        let impl_ = quote! { impl <T, U> MyTrait<T> for MyType { fn foo(&self, my_arg: T) {} } };
        ImplBody::try_from((impl_, condition)).unwrap()
    }

    fn get_trait_body(impl_: &ImplBody) -> TraitBody {
        let trait_ = quote! { trait MyTrait<A> { fn foo(&self, my_arg: A); } };
        TraitBody::try_from(trait_).unwrap().specialize(impl_)
    }

    fn get_annotation_body() -> AnnotationBody {
        AnnotationBody {
            fn_: "foo".to_string(),
            args: vec!["my_arg".to_string()],
            args_types: vec!["MyType".to_string()],
            annotations: vec![Annotation::Trait("MyType".to_string(), vec!["MyTrait".to_string()])],
            ..Default::default()
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
    fn most_specific_type() {
        let condition = WhenCondition::All(
            vec![
                WhenCondition::Type("T".into(), "_".into()),
                WhenCondition::Type("T".into(), "Vec<MyType>".into()),
                WhenCondition::Type("T".into(), "Vec<_>".into())
            ]
        );
        let var = VarBody {
            aliases: Aliases::default(),
            generics: vec!["T".into()].into_iter().collect(),
            vars: vec![VarInfo {
                impl_generic: "T".into(),
                concrete_type: "Vec<MyType>".into(),
                traits: vec![],
            }],
        };

        let (satisfies, constraints) = satisfies_condition(
            &condition,
            &var,
            &Constraints::default()
        );

        assert!(satisfies);

        let c = constraints.get("T".into()).unwrap();
        assert_eq!(c.type_.clone().unwrap().replace(" ", ""), "Vec<MyType>".to_string());
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

    #[test]
    fn default_impl() {
        let impls = vec![get_impl_body(None)];
        let traits = vec![get_trait_body(&impls[0])];
        let annotations = get_annotation_body();

        let result = SpecBody::try_from((&impls, &traits, &annotations));

        assert!(result.is_ok());
        let spec_body = result.unwrap();
        assert_eq!(spec_body.impl_.trait_name, "MyTrait");
        assert_eq!(spec_body.constraints, Constraints::default());
    }

    #[test]
    fn single_impl() {
        let impls = vec![get_impl_body(Some(WhenCondition::Type("T".into(), "MyType".into())))];
        let traits = vec![get_trait_body(&impls[0])];
        let annotations = get_annotation_body();

        let result = SpecBody::try_from((&impls, &traits, &annotations));

        assert!(result.is_ok());
        let spec_body = result.unwrap();
        assert_eq!(spec_body.impl_.trait_name, "MyTrait");
        assert_eq!(
            spec_body.constraints.get("T".into()),
            Some(
                &(Constraint {
                    generics: HashSet::new(),
                    type_: Some("MyType".into()),
                    traits: vec![],
                    not_types: vec![],
                    not_traits: vec![],
                })
            )
        );
    }

    #[test]
    fn multiple_impls() {
        let impls = vec![
            get_impl_body(Some(WhenCondition::Type("T".into(), "MyType".into()))),
            get_impl_body(Some(WhenCondition::Trait("T".into(), vec!["MyTrait".into()])))
        ];
        let traits = vec![get_trait_body(&impls[0])];
        let annotations = get_annotation_body();

        let result = SpecBody::try_from((&impls, &traits, &annotations));

        assert!(result.is_ok());
        let spec_body = result.unwrap();
        assert_eq!(spec_body.impl_.trait_name, "MyTrait");
        assert_eq!(
            spec_body.constraints.get("T".into()),
            Some(
                &(Constraint {
                    generics: HashSet::new(),
                    type_: Some("MyType".into()),
                    traits: vec![],
                    not_types: vec![],
                    not_traits: vec![],
                })
            )
        );
    }

    #[test]
    fn multiple_equally_specific_impls() {
        let impls = vec![
            get_impl_body(Some(WhenCondition::Type("T".into(), "MyType".into()))),
            get_impl_body(Some(WhenCondition::Type("T".into(), "MyType".into())))
        ];
        let traits = vec![get_trait_body(&impls[0]), get_trait_body(&impls[1])];
        let annotations = get_annotation_body();

        let result = SpecBody::try_from((&impls, &traits, &annotations));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Multiple implementations are equally specific");
    }

    #[test]
    fn no_valid_impl() {
        let impls = vec![
            get_impl_body(Some(WhenCondition::Type("T".into(), "MyOtherType".into()))),
            get_impl_body(Some(WhenCondition::Trait("T".into(), vec!["MyOtherTrait".into()])))
        ];
        let traits = vec![get_trait_body(&impls[0]), get_trait_body(&impls[1])];
        let annotations = get_annotation_body();

        let result = SpecBody::try_from((&impls, &traits, &annotations));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No valid implementation found");
    }

    #[test]
    fn impl_with_wildcard() {
        let impls = vec![get_impl_body(Some(WhenCondition::Type("T".into(), "Vec<_>".into())))];
        let traits = vec![get_trait_body(&impls[0])];
        let mut annotations = get_annotation_body();
        annotations.args_types = vec!["Vec<MyType>".to_string()];

        let result = SpecBody::try_from((&impls, &traits, &annotations));

        assert!(result.is_ok());
        let spec_body = result.unwrap();
        assert_eq!(spec_body.impl_.trait_name, "MyTrait");
        assert_eq!(
            spec_body.constraints.get("T".into()).unwrap().type_.clone().unwrap().replace(" ", ""),
            "Vec<_>".to_string()
        );
    }

    #[test]
    fn impl_with_generic() {
        let impls = vec![get_impl_body(Some(WhenCondition::Type("T".into(), "Vec<U>".into())))];
        let traits = vec![get_trait_body(&impls[0])];
        let mut annotations = get_annotation_body();
        annotations.args_types = vec!["Vec<MyType>".to_string()];

        let result = SpecBody::try_from((&impls, &traits, &annotations));

        assert!(result.is_ok());
        let spec_body = result.unwrap();
        assert_eq!(spec_body.impl_.trait_name, "MyTrait");
        assert_eq!(
            spec_body.constraints.get("T".into()).unwrap().type_.clone().unwrap().replace(" ", ""),
            "Vec<U>".to_string()
        );
    }

    #[test]
    fn impl_with_conditioned_generics() {
        let impls = vec![
            get_impl_body(
                Some(
                    WhenCondition::All(
                        vec![
                            WhenCondition::Type("T".into(), "Vec<U>".into()),
                            WhenCondition::Trait("U".into(), vec!["MyTrait".into()])
                        ]
                    )
                )
            )
        ];
        let traits = vec![get_trait_body(&impls[0])];
        let mut annotations = get_annotation_body();
        annotations.args_types = vec!["Vec<MyType>".to_string()];

        let result = SpecBody::try_from((&impls, &traits, &annotations));

        assert!(result.is_ok());
        let spec_body = result.unwrap();
        assert_eq!(spec_body.impl_.trait_name, "MyTrait");
        assert_eq!(
            spec_body.constraints.get("T".into()).unwrap().type_.clone().unwrap().replace(" ", ""),
            "Vec<U>".to_string()
        );
        assert!(
            spec_body.constraints.get("U".into()).unwrap().traits.contains(&"MyTrait".to_string())
        );
    }

    #[test]
    fn impl_with_conditioned_generics_not_valid() {
        let impls = vec![
            get_impl_body(
                Some(
                    WhenCondition::All(
                        vec![
                            WhenCondition::Type("T".into(), "Vec<U>".into()),
                            WhenCondition::Trait("U".into(), vec!["MyOtherTrait".into()])
                        ]
                    )
                )
            )
        ];
        let traits = vec![get_trait_body(&impls[0])];
        let mut annotations = get_annotation_body();
        annotations.args_types = vec!["Vec<MyType>".to_string()];

        let result = SpecBody::try_from((&impls, &traits, &annotations));

        assert!(!result.is_ok());
    }
}
