use syn::{Expr, Generics, ImplItem, Path, TraitItem, Type, parse_str};

pub fn str_to_generics(str: &str) -> Generics {
    parse_str::<Generics>(str).expect("Failed to parse generics")
}

pub fn str_to_trait(str: &str) -> Path {
    parse_str::<Path>(str).expect("Failed to parse path")
}

pub fn str_to_type(str: &str) -> Type {
    parse_str::<Type>(str).expect("Failed to parse type")
}

pub fn str_to_impl_item(str: &str) -> ImplItem {
    parse_str::<ImplItem>(str).expect("Failed to parse impl item")
}

pub fn str_to_trait_item(str: &str) -> TraitItem {
    parse_str::<TraitItem>(str).expect("Failed to parse trait item")
}

pub fn strs_to_impl_fns(strs: &Vec<String>) -> Vec<ImplItem> {
    strs.iter().map(|f| str_to_impl_item(f)).collect()
}

pub fn strs_to_trait_fns(strs: &Vec<String>) -> Vec<TraitItem> {
    strs.iter().map(|f| str_to_trait_item(f)).collect()
}

pub fn str_to_expr(str: &str) -> Expr {
    parse_str::<Expr>(str).expect("Failed to parse expr")
}
