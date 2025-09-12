use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::visit_mut::{ self, VisitMut };
use syn::{ GenericParam, Generics, Ident, Type, TypeParam };
use crate::conversions::{ str_to_type_name, to_string };
use crate::parsing::get_generics;
use crate::types::{ replace_infers, replace_type, types_equal, Aliases };
use crate::conditions::WhenCondition;

pub trait Specializable {
    fn resolve_item_generic(&self, other_generics: &Generics, impl_generic: &str) -> Option<String>;

    fn handle_items_replace<V: VisitMut>(&mut self, replacer: &mut V);
}

pub fn get_assignable_conditions(
    conditions: &[WhenCondition],
    generics: &str
) -> Vec<WhenCondition> {
    let generics = get_generics(generics);
    conditions
        .iter()
        .filter_map(|c| {
            match c {
                WhenCondition::Trait(_, _) => Some(c.clone()),
                WhenCondition::Type(g, t) => {
                    let types = get_generic_types_from_conditions(g, conditions);
                    let most_specific = types.last() == Some(t);
                    let diff_types = types
                        .iter()
                        .any(|other_t| !types_equal(t, other_t, &generics, &Aliases::default()));

                    if diff_types || !most_specific {
                        None
                    } else {
                        Some(c.clone())
                    }
                }
                _ => None,
            }
        })
        .collect()
}

fn get_generic_types_from_conditions(generic: &str, conditions: &[WhenCondition]) -> Vec<String> {
    let mut types = conditions
        .iter()
        .filter_map(|c| {
            match c {
                WhenCondition::Type(g, t) if g == generic => Some(t.clone()),
                _ => None,
            }
        })
        .collect::<Vec<_>>();
    types.sort_by_key(|t| t.replace("_", "").len());
    types
}

pub struct TypeReplacer {
    pub generic: String,
    pub type_: Type,
}

impl VisitMut for TypeReplacer {
    fn visit_type_mut(&mut self, node: &mut Type) {
        println!("replacing {} with {}", self.generic, to_string(&self.type_));
        replace_type(node, &self.generic, &self.type_);
        visit_mut::visit_type_mut(self, node);
    }
}

pub fn apply_type_condition<T: Specializable>(
    target: &mut T,
    generics: &mut Generics,
    other_generics: &mut Generics,
    impl_generic: &str,
    type_: &str
) {
    let item_generic = target
        .resolve_item_generic(other_generics, impl_generic)
        .unwrap_or_else(|| impl_generic.to_string());

    // replace infers in the type
    let mut new_type = str_to_type_name(type_);
    let mut existing_generics = collect_generics(generics);
    let mut counter = 0;
    let mut new_generics = vec![];

    replace_infers(&mut new_type, &mut existing_generics, &mut counter, &mut new_generics);

    // add new generics
    for generic in new_generics {
        add_generic(generics, &generic);
        add_generic(other_generics, &generic);
    }

    // remove generic
    remove_generic(generics, &item_generic);
    remove_generic(other_generics, impl_generic);

    // replace generic with type in the items
    let mut replacer = TypeReplacer {
        generic: item_generic.clone(),
        type_: new_type.clone(),
    };

    target.handle_items_replace(&mut replacer);
}

fn remove_generic(generics: &mut Generics, generic: &str) {
    generics.params = generics.params
        .clone()
        .into_iter()
        .filter(|param| !matches!(param, GenericParam::Type(tp) if tp.ident == generic))
        .collect();
}

pub fn collect_generics<T: FromIterator<String>>(generics: &Generics) -> T {
    generics.params
        .iter()
        .filter_map(|p| {
            match p {
                GenericParam::Type(tp) => Some(tp.ident.to_string()),
                _ => None,
            }
        })
        .collect()
}

pub fn add_generic(generics: &mut Generics, generic: &str) {
    generics.params.push(
        GenericParam::Type(TypeParam {
            attrs: vec![],
            ident: Ident::new(generic, Span::call_site()),
            colon_token: None,
            bounds: Punctuated::new(),
            eq_token: None,
            default: None,
        })
    )
}
