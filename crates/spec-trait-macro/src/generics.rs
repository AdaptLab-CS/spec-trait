use crate::conditions::WhenCondition;
use crate::cache::Impl;
use crate::traits::TraitBody;

pub fn get_for_impl(impl_: &Impl, traits: &[TraitBody], constraints: &[WhenCondition]) -> String {
    let trait_ = traits
        .iter()
        .find(|tr| tr.name == impl_.trait_name)
        .expect("Trait not found");

    let generics_without_angle_brackets = &trait_.generics[1..trait_.generics.len() - 1];
    let types = generics_without_angle_brackets
        .split(',')
        .filter_map(|g| get_type(g.trim(), constraints))
        .collect::<Vec<_>>();

    if types.is_empty() {
        String::new()
    } else {
        format!("<{}>", types.join(", "))
    }
}

fn get_type(generic: &str, constraints: &[WhenCondition]) -> Option<String> {
    if generic.is_empty() {
        return None;
    }

    constraints
        .iter()
        .find_map(|c| {
            match c {
                WhenCondition::Type(g, type_) if generic == *g => Some(type_.clone()),
                _ => None,
            }
        })
        .or(Some("_".to_string()))
}
