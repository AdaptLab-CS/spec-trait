use std::collections::{ HashMap, HashSet };
use crate::conversions::{ str_to_type_name, to_string };
use syn::{
    Type,
    TypeTuple,
    TypeReference,
    TypeArray,
    PathArguments,
    GenericArgument,
    TypeSlice,
    Expr,
};

pub type Aliases = HashMap<String, Vec<String>>;

pub fn get_concrete_type(type_or_alias: &str, aliases: &Aliases) -> String {
    let parsed_type = str_to_type_name(type_or_alias);
    let resolved_type = resolve_type(&parsed_type, aliases);
    to_string(&resolved_type)
}

fn resolve_type(ty: &Type, aliases: &Aliases) -> Type {
    match unwrap_paren(ty) {
        #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

        // (T, U)
        Type::Tuple(tuple) => {
            let resolved_elems = tuple.elems
                .iter()
                .map(|elem| resolve_type(elem, aliases))
                .collect();
            Type::Tuple(TypeTuple {
                elems: resolved_elems,
                ..tuple.clone()
            })
        }

        // &T
        Type::Reference(reference) => {
            let resolved_elem = resolve_type(&reference.elem, aliases);
            Type::Reference(TypeReference {
                elem: Box::new(resolved_elem),
                ..reference.clone()
            })
        }

        // [T; N]
        Type::Array(array) => {
            let resolved_elem = resolve_type(&array.elem, aliases);
            Type::Array(TypeArray {
                elem: Box::new(resolved_elem),
                ..array.clone()
            })
        }

        // [T]
        Type::Slice(slice) => {
            let resolved_elem = resolve_type(&slice.elem, aliases);
            Type::Slice(TypeSlice {
                elem: Box::new(resolved_elem),
                ..slice.clone()
            })
        }

        // T, T<U>
        Type::Path(type_path) if type_path.qself.is_none() => {
            let mut resolved_path = type_path.clone();

            let ident = type_path.path.segments.last().unwrap().ident.to_string();
            if let Some((k, _)) = aliases.iter().find(|(_, v)| v.contains(&ident)) {
                return str_to_type_name(k);
            }

            for segment in &mut resolved_path.path.segments {
                if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in &mut args.args {
                        if let GenericArgument::Type(inner_ty) = arg {
                            *inner_ty = resolve_type(inner_ty, aliases);
                        }
                    }
                }
            }

            Type::Path(resolved_path)
        }

        // Default case: return the type as-is
        _ => ty.clone(),
    }
}

type Generics = HashSet<String>;
type GenericsMap = HashMap<String, Option<String>>;

/// types can be something like: "T", "&T", "U<T>", "(T, T)", "&[T]"
/// each of the "T" can be a type, a generic or a "_", which means any type
pub fn types_equal_generic_constraints(
    type1: &str,
    type2: &str,
    generics: &Generics,
    aliases: &Aliases
) -> Option<GenericsMap> {
    let t1 = str_to_type_name(&get_concrete_type(type1, aliases));
    let t2 = str_to_type_name(&get_concrete_type(type2, aliases));

    let mut generics_map = generics
        .iter()
        .map(|g| (g.clone(), None))
        .collect();

    if same_type(&t1, &t2, &mut generics_map) {
        Some(generics_map)
    } else {
        None
    }
}

/// types can be something like: "T", "&T", "U<T>", "(T, T)", "&[T]"
/// each of the "T" can be a type, a generic or a "_", which means any type
pub fn types_equal(type1: &str, type2: &str, generics: &Generics, aliases: &Aliases) -> bool {
    types_equal_generic_constraints(type1, type2, generics, aliases).is_some()
}

fn same_type(t1: &Type, t2: &Type, generics: &mut GenericsMap) -> bool {
    let t1 = unwrap_paren(t1);
    let t2 = unwrap_paren(t2);

    match (t1, t2) {
        #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

        // `_`
        (t1, t2) if matches!(t1, Type::Infer(_)) || matches!(t2, Type::Infer(_)) => true,

        // `T` generic
        (t1, t2) if
            matches!(t1, Type::Path(p1) if p1.qself.is_none() && p1.path.segments.len() == 1 && generics.contains_key(&p1.path.segments[0].ident.to_string())) ||
            matches!(t2, Type::Path(p2) if p2.qself.is_none() && p2.path.segments.len() == 1 && generics.contains_key(&p2.path.segments[0].ident.to_string()))
        => check_equal_and_assign_generic(&to_string(t1), &to_string(t2), generics),

        // `(T, U)`, `(T, _)`
        (Type::Tuple(tuple1), Type::Tuple(tuple2)) => {
            tuple1.elems.len() == tuple2.elems.len() &&
                tuple1.elems
                    .iter()
                    .zip(&tuple2.elems)
                    .all(|(elem1, elem2)| same_type(elem1, elem2, generics))
        }

        // `&T`, `&_`
        (Type::Reference(ref1), Type::Reference(ref2)) => {
            same_type(&ref1.elem, &ref2.elem, generics)
        }

        // `[T]`, `[_]`
        (Type::Slice(slice1), Type::Slice(slice2)) => {
            same_type(&slice1.elem, &slice2.elem, generics)
        }

        // `[T; N]`, `[_; N]`, `[T; _]`, `[_; _]`
        (Type::Array(array1), Type::Array(array2)) => {
            same_type(&array1.elem, &array2.elem, generics) &&
                (matches!(array1.len, Expr::Infer(_)) ||
                    matches!(array2.len, Expr::Infer(_)) ||
                    to_string(&array1.len) == to_string(&array2.len))
        }

        // `T`, `T<U>`, `T<_>`
        (Type::Path(path1), Type::Path(path2)) if path1.qself.is_none() && path2.qself.is_none() => {
            if path1.path.segments.len() == 1 {
                let key = path1.path.segments.last().unwrap().ident.to_string();
                if let Some(existing) = generics.get(&key).cloned() {
                    if let Some(existing_val) = existing {
                        let existing_ty = str_to_type_name(&existing_val);
                        return same_type(&existing_ty, t2, generics);
                    } else {
                        generics.insert(key.clone(), Some(to_string(t2)));
                        return true;
                    }
                }
            }

            if path2.path.segments.len() == 1 {
                let key = path2.path.segments.last().unwrap().ident.to_string();
                if let Some(existing) = generics.get(&key).cloned() {
                    if let Some(existing_val) = existing {
                        let existing_ty = str_to_type_name(&existing_val);
                        return same_type(t1, &existing_ty, generics);
                    } else {
                        generics.insert(key.clone(), Some(to_string(t1)));
                        return true;
                    }
                }
            }

            path1.path.segments.len() == path2.path.segments.len() &&
                path1.path.segments
                    .iter()
                    .zip(&path2.path.segments)
                    .all(|(seg1, seg2)| {
                        check_equal_and_assign_generic(
                            &seg1.ident.to_string(),
                            &seg2.ident.to_string(),
                            generics
                        ) &&
                            (match (&seg1.arguments, &seg2.arguments) {
                                (
                                    PathArguments::AngleBracketed(args1),
                                    PathArguments::AngleBracketed(args2),
                                ) =>
                                    args1.args
                                        .iter()
                                        .zip(&args2.args)
                                        .all(|(arg1, arg2)| {
                                            match (arg1, arg2) {
                                                (
                                                    GenericArgument::Type(t1),
                                                    GenericArgument::Type(t2),
                                                ) => same_type(t1, t2, generics),
                                                _ => false,
                                            }
                                        }),
                                _ => seg1.arguments.is_empty() && seg2.arguments.is_empty(),
                            })
                    })
        }

        _ => false,
    }
}

fn unwrap_paren(ty: &Type) -> &Type {
    if let Type::Paren(paren) = ty { unwrap_paren(&paren.elem) } else { ty }
}

fn check_equal_and_assign_generic(t1: &str, t2: &str, generics: &mut GenericsMap) -> bool {
    if t1 == t2 || t1 == "_" || t2 == "_" {
        return true;
    }

    let t1_generic = generics.get(t1).cloned();
    let t2_generic = generics.get(t2).cloned();

    if
        t1_generic.is_some_and(|v| {
            v.clone().is_none_or(|v|
                same_type(&str_to_type_name(&v), &str_to_type_name(t2), generics)
            )
        })
    {
        generics.insert(t1.to_string(), Some(t2.to_string()));
        return true;
    }

    if
        t2_generic.is_some_and(|v| {
            v.clone().is_none_or(|v|
                same_type(&str_to_type_name(&v), &str_to_type_name(t1), generics)
            )
        })
    {
        generics.insert(t2.to_string(), Some(t1.to_string()));
        return true;
    }

    false
}

/// Replaces all occurrences of `prev` in the given type with `new`.
pub fn replace_type(ty: &mut Type, prev: &str, new: &Type) {
    match ty {
        // (T, U)
        Type::Tuple(t) => {
            for elem in &mut t.elems {
                replace_type(elem, prev, new);
            }
        }

        // &T
        Type::Reference(r) => replace_type(&mut r.elem, prev, new),

        // [T; N]
        Type::Array(a) => replace_type(&mut a.elem, prev, new),

        // [T]
        Type::Slice(s) => replace_type(&mut s.elem, prev, new),

        // (T)
        Type::Paren(s) => replace_type(&mut s.elem, prev, new),

        // T, T<U>
        Type::Path(type_path) => {
            // T
            if
                type_path.qself.is_none() &&
                type_path.path.segments.len() == 1 &&
                type_path.path.segments[0].ident == prev
            {
                *ty = new.clone();
                return;
            }

            // T<U>
            for seg in &mut type_path.path.segments {
                if let PathArguments::AngleBracketed(ref mut ab) = seg.arguments {
                    for arg in ab.args.iter_mut() {
                        if let GenericArgument::Type(inner_ty) = arg {
                            replace_type(inner_ty, prev, new);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// Replaces all occurrences of `_` (inferred types) in the given type with fresh generic type parameters.
pub fn replace_infers(
    ty: &mut Type,
    generics: &mut HashSet<String>,
    counter: &mut usize,
    new_generics: &mut Vec<String>
) {
    match ty {
        // (T, U, _)
        Type::Tuple(t) => {
            for elem in &mut t.elems {
                replace_infers(elem, generics, counter, new_generics);
            }
        }

        // &_
        Type::Reference(r) => replace_infers(&mut r.elem, generics, counter, new_generics),

        // [_; N]
        Type::Array(a) => replace_infers(&mut a.elem, generics, counter, new_generics),

        // [_]
        Type::Slice(s) => replace_infers(&mut s.elem, generics, counter, new_generics),

        // (_)
        Type::Paren(p) => replace_infers(&mut p.elem, generics, counter, new_generics),

        // T<_>
        Type::Path(type_path) => {
            for seg in &mut type_path.path.segments {
                if let PathArguments::AngleBracketed(ref mut ab) = seg.arguments {
                    for arg in ab.args.iter_mut() {
                        if let GenericArgument::Type(inner_ty) = arg {
                            replace_infers(inner_ty, generics, counter, new_generics);
                        }
                    }
                }
            }
        }

        // _
        Type::Infer(_) => {
            let name = get_unique_generic_name(generics, counter);
            *ty = str_to_type_name(&name);
            new_generics.push(name);
        }

        _ => {}
    }
}

pub fn get_unique_generic_name(generics: &mut HashSet<String>, counter: &mut usize) -> String {
    loop {
        let candidate = format!("__G_{}__", *counter);
        *counter += 1;

        if generics.insert(candidate.clone()) {
            return candidate;
        }
    }
}

/// Collect all inner generic names (that appear in `generics`) from a parsed `Type`.
/// Traverses paths, angle-bracketed args, references, tuples, arrays, slices and parens.
/// Preserves discovery order and deduplicates.
pub fn extract_inners_from_type(ty: &Type, generics: &Generics, seen: &mut Generics) {
    match unwrap_paren(ty) {
        Type::Path(type_path) if type_path.qself.is_none() => {
            for seg in &type_path.path.segments {
                let ident = seg.ident.to_string();
                if generics.contains(&ident) {
                    seen.insert(ident.clone());
                }
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    for arg in &args.args {
                        if let GenericArgument::Type(inner_ty) = arg {
                            extract_inners_from_type(inner_ty, generics, seen);
                        }
                    }
                }
            }
        }

        Type::Reference(reference) => extract_inners_from_type(&reference.elem, generics, seen),

        Type::Tuple(tuple) => {
            for elem in &tuple.elems {
                extract_inners_from_type(elem, generics, seen);
            }
        }

        Type::Slice(slice) => extract_inners_from_type(&slice.elem, generics, seen),

        Type::Array(array) => extract_inners_from_type(&array.elem, generics, seen),

        _ => {}
    }
}

/// Parse a type string and return all inner generic names that belong to `generics`.
pub fn extract_inners(type_str: &str, generics: &Generics) -> Vec<String> {
    let parsed = str_to_type_name(type_str);
    let mut seen = HashSet::new();
    extract_inners_from_type(&parsed, generics, &mut seen);
    seen.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse2;
    use quote::quote;

    fn get_aliases() -> Aliases {
        let mut aliases = Aliases::new();
        aliases.insert("u8".to_string(), vec!["MyType".to_string()]);
        aliases
    }

    #[test]
    fn resolve_type_simple() {
        let ty = str_to_type_name("MyType");
        let resolved = resolve_type(&ty, &get_aliases());
        assert_eq!(to_string(&resolved), "u8");
    }

    #[test]
    fn resolve_type_tuples() {
        let ty = str_to_type_name("(MyType, u8)");
        let resolved = resolve_type(&ty, &get_aliases());
        assert_eq!(to_string(&resolved).replace(" ", ""), "(u8,u8)");
    }

    #[test]
    fn resolve_type_references() {
        let ty = str_to_type_name("&MyType");
        let resolved = resolve_type(&ty, &get_aliases());
        assert_eq!(to_string(&resolved).replace(" ", ""), "&u8");
    }

    #[test]
    fn resolve_type_arrays() {
        let ty = str_to_type_name("[MyType; 3]");
        let resolved = resolve_type(&ty, &get_aliases());
        assert_eq!(to_string(&resolved).replace(" ", ""), "[u8;3]");
    }

    #[test]
    fn resolve_type_slices() {
        let ty = str_to_type_name("[MyType]");
        let resolved = resolve_type(&ty, &get_aliases());
        assert_eq!(to_string(&resolved).replace(" ", ""), "[u8]");
    }

    #[test]
    fn resolve_type_parens() {
        let ty = str_to_type_name("(MyType)");
        let resolved = resolve_type(&ty, &get_aliases());
        assert_eq!(to_string(&resolved), "u8");
    }

    #[test]
    fn resolve_type_paths() {
        let ty = str_to_type_name("Vec<MyType>");
        let resolved = resolve_type(&ty, &get_aliases());
        assert_eq!(to_string(&resolved).replace(" ", ""), "Vec<u8>");
    }

    #[test]
    fn resolve_type_nested() {
        let ty = str_to_type_name("Option<(MyType, Vec<MyType>)>");
        let resolved = resolve_type(&ty, &get_aliases());
        assert_eq!(to_string(&resolved).replace(" ", ""), "Option<(u8,Vec<u8>)>");
    }

    #[test]
    fn compare_types_simple() {
        let mut g = GenericsMap::new();

        let t1 = str_to_type_name("_");
        let t2 = str_to_type_name("u8");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("u8");
        let t2 = str_to_type_name("_");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("_");
        let t2 = str_to_type_name("_");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("T");
        let t2 = str_to_type_name("u8");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("T");
        let t2 = str_to_type_name("T");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        g.insert("U".to_string(), None);
        let t1 = str_to_type_name("T");
        let t2 = str_to_type_name("U");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("T");
        let t2 = str_to_type_name("_");
        assert!(same_type(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_tuples() {
        let mut g = GenericsMap::new();

        let t1 = str_to_type_name("(u8, _)");
        let t2 = str_to_type_name("(u8, i32)");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("(u8, T)");
        let t2 = str_to_type_name("(u8, i32)");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("T");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(u8, i32)");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(u8, f32)");
        assert!(!same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(T, T)");
        assert!(!same_type(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_references() {
        let mut g = GenericsMap::new();

        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&u8");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&_");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&T");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("&i8");
        let t2 = str_to_type_name("T");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&i8");
        assert!(!same_type(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_slices() {
        let mut g = GenericsMap::new();

        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[u8]");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[_]");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[T]");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("T");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[i8]");
        assert!(!same_type(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_arrays() {
        let mut g = GenericsMap::new();

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[u8; 3]");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[u8; 4]");
        assert!(!same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[_; 3]");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8; _]");
        let t2 = str_to_type_name("[u8; 3]");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[_; _]");
        let t2 = str_to_type_name("[u8; 3]");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[T; 3]");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("T");
        assert!(same_type(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_parens() {
        let mut g = GenericsMap::new();

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((u8))");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("(u8)");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((i32))");
        assert!(!same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((_))");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((T))");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("T");
        assert!(same_type(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_paths() {
        let mut g = GenericsMap::new();

        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("Vec<u8>");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("Vec<_>");
        let t2 = str_to_type_name("Vec<u8>");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("_");
        let t2 = str_to_type_name("Vec<u8>");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("Vec<T>");
        let t2 = str_to_type_name("Vec<u8>");
        assert!(same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("T");
        let t2 = str_to_type_name("Vec<u8>");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("Vec<i32>");
        assert!(!same_type(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_nested() {
        let mut g = GenericsMap::new();

        let t1 = str_to_type_name("Option<(u8, _)>");
        let t2 = str_to_type_name("Option<(u8, i32)>");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("Result<Vec<_>, _>");
        let t2 = str_to_type_name("Result<Vec<u8>, String>");
        assert!(same_type(&t1, &t2, &mut g));

        let t1 = str_to_type_name("Result<Vec<u8>, String>");
        let t2 = str_to_type_name("Result<Vec<i32>, String>");
        assert!(!same_type(&t1, &t2, &mut g));

        g.insert("T".to_string(), None);
        let t1 = str_to_type_name("Result<Vec<u8>, String>");
        let t2 = str_to_type_name("Result<T, T>");
        assert!(!same_type(&t1, &t2, &mut g));
    }

    #[test]
    fn replace_type_simple() {
        let mut ty: Type = parse2(quote! { T }).unwrap();
        let new_ty: Type = parse2(quote! { String }).unwrap();

        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "String".to_string().replace(" ", ""));
    }

    #[test]
    fn replace_type_tuple() {
        let mut ty: Type = parse2(quote! { (T, Other, T) }).unwrap();
        let new_ty: Type = parse2(quote! { String }).unwrap();

        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "(String, Other, String)".to_string().replace(" ", "")
        );
    }

    #[test]
    fn replace_type_reference() {
        let mut ty: Type = parse2(quote! { &T }).unwrap();
        let new_ty: Type = parse2(quote! { String }).unwrap();

        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "&String".to_string().replace(" ", ""));
    }

    #[test]
    fn replace_type_array() {
        let mut ty: Type = parse2(quote! { [T; 3] }).unwrap();
        let new_ty: Type = parse2(quote! { String }).unwrap();

        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "[String; 3]".to_string().replace(" ", ""));
    }

    #[test]
    fn replace_type_slice() {
        let mut ty: Type = parse2(quote! { &[T] }).unwrap();
        let new_ty: Type = parse2(quote! { String }).unwrap();

        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "&[String]".to_string().replace(" ", ""));
    }

    #[test]
    fn replace_type_paren() {
        let mut ty: Type = parse2(quote! { (T) }).unwrap();
        let new_ty: Type = parse2(quote! { String }).unwrap();

        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "(String)".to_string().replace(" ", ""));
    }

    #[test]
    fn replace_type_path() {
        let mut ty: Type = parse2(quote! { Option<T> }).unwrap();
        let new_ty: Type = parse2(quote! { String }).unwrap();

        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "Option<String>".to_string().replace(" ", ""));
    }

    #[test]
    fn replace_type_nested() {
        let mut ty: Type = parse2(quote! { Option<(T, &[T])> }).unwrap();
        let new_ty: Type = parse2(quote! { String }).unwrap();

        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "Option<(String, &[String])>".to_string().replace(" ", "")
        );
    }

    #[test]
    fn replace_infers_simple() {
        let mut ty: Type = parse2(quote! { _ }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(to_string(&ty).replace(" ", ""), "__W0".to_string().replace(" ", ""));
        assert_eq!(new_generics, vec!["__W0".to_string()]);
    }

    #[test]
    fn replace_infers_tuple() {
        let mut ty: Type = parse2(quote! { (_, Other, _) }).unwrap();

        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "(__W0, Other, __W1)".to_string().replace(" ", "")
        );
        assert_eq!(new_generics, vec!["__W0".to_string(), "__W1".to_string()]);
    }

    #[test]
    fn replace_infers_reference() {
        let mut ty: Type = parse2(quote! { &_ }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(to_string(&ty).replace(" ", ""), "&__W0".to_string().replace(" ", ""));
        assert_eq!(new_generics, vec!["__W0".to_string()]);
    }

    #[test]
    fn replace_infers_array() {
        let mut ty: Type = parse2(quote! { [_; 3] }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(to_string(&ty).replace(" ", ""), "[__W0; 3]".to_string().replace(" ", ""));
        assert_eq!(new_generics, vec!["__W0".to_string()]);
    }

    #[test]
    fn replace_infers_slice() {
        let mut ty: Type = parse2(quote! { &[_] }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(to_string(&ty).replace(" ", ""), "&[__W0]".to_string().replace(" ", ""));
        assert_eq!(new_generics, vec!["__W0".to_string()]);
    }

    #[test]
    fn replace_infers_paren() {
        let mut ty: Type = parse2(quote! { (_) }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(to_string(&ty).replace(" ", ""), "(__W0)".to_string().replace(" ", ""));
        assert_eq!(new_generics, vec!["__W0".to_string()]);
    }

    #[test]
    fn replace_infers_path() {
        let mut ty: Type = parse2(quote! { Option<_> }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(to_string(&ty).replace(" ", ""), "Option<__W0>".to_string().replace(" ", ""));
        assert_eq!(new_generics, vec!["__W0".to_string()]);
    }

    #[test]
    fn replace_infers_nested() {
        let mut ty: Type = parse2(quote! { Option<(_, &[_])> }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "Option<(__W0, &[__W1])>".to_string().replace(" ", "")
        );
        assert_eq!(new_generics, vec!["__W0".to_string(), "__W1".to_string()]);
    }
}
