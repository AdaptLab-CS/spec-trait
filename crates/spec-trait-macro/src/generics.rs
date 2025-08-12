use crate::WhenCondition;
use crate::cache::Impl;
use crate::traits::TraitBody;

pub fn get_for_impl(
    impl_: &Impl,
    traits: &Vec<TraitBody>,
    constraints: &Vec<WhenCondition>,
) -> String {
    let trait_ = traits
        .iter()
        .find(|tr| tr.name == impl_.trait_name)
        .unwrap();

    let generics = trait_
        .generics
        .replace("<", "")
        .replace(">", "")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    let generics_types = generics
        .iter()
        .map(|g| {
            if let Some(cond) = constraints.iter().find(|c| match c {
                WhenCondition::Type(generic, _) if *generic == *g => true,
                _ => false,
            }) {
                match cond {
                    WhenCondition::Type(_, type_) => type_.clone(),
                    _ => panic!("Expected a type condition"),
                }
            } else {
                "_".to_string()
            }
        })
        .collect::<Vec<String>>();

    if generics_types.is_empty() {
        String::new()
    } else {
        format!("<{}>", generics_types.join(", "))
    }
}
