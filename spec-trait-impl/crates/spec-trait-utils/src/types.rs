use crate::{
    conversions::{str_to_generics, str_to_lifetime, str_to_type_name, to_string},
    specialize::collect_generics_lifetimes,
};
use proc_macro2::Span;
use std::collections::{HashMap, HashSet};
use syn::{
    Expr, GenericArgument, GenericParam, Generics, Ident, PathArguments, Type, TypeArray,
    TypeReference, TypeSlice, TypeTuple,
};

pub type Aliases = HashMap<String, Vec<String>>;

pub fn get_concrete_type(type_or_alias: &str, aliases: &Aliases) -> String {
    let parsed_type = str_to_type_name(type_or_alias);
    let resolved_type = resolve_type(&parsed_type, aliases);
    to_string(&resolved_type)
}

fn resolve_type(ty: &Type, aliases: &Aliases) -> Type {
    match unwrap_paren(ty) {
        // (T, U)
        Type::Tuple(tuple) => {
            let resolved_elems = tuple
                .elems
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

type GenericsMap = HashMap<String, Option<String>>;

#[derive(Debug, Default)]
pub struct ConstrainedGenerics {
    pub types: GenericsMap,
    pub lifetimes: GenericsMap,
}

impl From<Generics> for ConstrainedGenerics {
    fn from(generics: Generics) -> Self {
        let types = generics
            .params
            .iter()
            .filter_map(|p| match p {
                GenericParam::Type(tp) => Some((tp.ident.to_string(), None)),
                _ => None,
            })
            .collect();

        let lifetimes = generics
            .params
            .iter()
            .filter_map(|p| match p {
                GenericParam::Lifetime(lt) => Some((lt.lifetime.to_string(), None)),
                _ => None,
            })
            .collect();

        ConstrainedGenerics { types, lifetimes }
    }
}

pub fn type_assignable_generic_constraints(
    concrete_type: &str,
    declared_or_concrete_type: &str,
    generics: &str,
    aliases: &Aliases,
) -> Option<ConstrainedGenerics> {
    let concrete_type = str_to_type_name(&get_concrete_type(concrete_type, aliases));
    let declared_or_concrete_type =
        str_to_type_name(&get_concrete_type(declared_or_concrete_type, aliases));

    let generics = str_to_generics(generics);
    let mut generics = ConstrainedGenerics::from(generics);

    if can_assign(&concrete_type, &declared_or_concrete_type, &mut generics) {
        Some(generics)
    } else {
        None
    }
}

pub fn type_assignable(
    concrete_type: &str,
    declared_or_concrete_type: &str,
    generics: &str,
    aliases: &Aliases,
) -> bool {
    type_assignable_generic_constraints(concrete_type, declared_or_concrete_type, generics, aliases)
        .is_some()
}

/// check if concrete_type can be assigned to declared_type
fn can_assign(
    concrete_type: &Type,
    declared_or_concrete_type: &Type,
    generics: &mut ConstrainedGenerics,
) -> bool {
    let t1 = unwrap_paren(concrete_type);
    let t2 = unwrap_paren(declared_or_concrete_type);

    match (t1, t2) {
        // `_`
        (_, Type::Infer(_)) => true,
        (Type::Infer(_), _) => true,

        // `T` generic
        (_, Type::Path(p2))
            if p2.qself.is_none()
                && p2.path.segments.len() == 1
                && generics
                    .types
                    .contains_key(&p2.path.segments[0].ident.to_string()) =>
        {
            check_and_assign_type_generic(&to_string(t1), &to_string(t2), generics)
        }

        // `(T, U)`, `(T, _)`
        (Type::Tuple(tuple1), Type::Tuple(tuple2)) => {
            tuple1.elems.len() == tuple2.elems.len()
                && tuple1
                    .elems
                    .iter()
                    .zip(&tuple2.elems)
                    .all(|(elem1, elem2)| can_assign(elem1, elem2, generics))
        }

        // `&T`, `&_`
        (Type::Reference(ref1), Type::Reference(ref2)) => {
            let lt1 = ref1.lifetime.as_ref().map(to_string);
            let lt2 = ref2.lifetime.as_ref().map(to_string);

            check_and_assign_lifetime_generic(&lt1, &lt2, generics)
                && can_assign(&ref1.elem, &ref2.elem, generics)
        }

        // `[T]`, `[_]`
        (Type::Slice(slice1), Type::Slice(slice2)) => {
            can_assign(&slice1.elem, &slice2.elem, generics)
        }

        // `[T; N]`, `[_; N]`, `[T; _]`, `[_; _]`
        (Type::Array(array1), Type::Array(array2)) => {
            can_assign(&array1.elem, &array2.elem, generics)
                && (matches!(array1.len, Expr::Infer(_))
                    || matches!(array2.len, Expr::Infer(_))
                    || to_string(&array1.len) == to_string(&array2.len))
        }

        // `T`, `T<U>`, `T<_>`
        (Type::Path(path1), Type::Path(path2))
            if path1.qself.is_none() && path2.qself.is_none() =>
        {
            path1.path.segments.len() == path2.path.segments.len()
                && path1
                    .path
                    .segments
                    .iter()
                    .zip(&path2.path.segments)
                    .all(|(seg1, seg2)| {
                        check_and_assign_type_generic(
                            &seg1.ident.to_string(),
                            &seg2.ident.to_string(),
                            generics,
                        ) && (match (&seg1.arguments, &seg2.arguments) {
                            (
                                PathArguments::AngleBracketed(args1),
                                PathArguments::AngleBracketed(args2),
                            ) => args1.args.iter().zip(&args2.args).all(|(arg1, arg2)| {
                                match (arg1, arg2) {
                                    (GenericArgument::Type(t1), GenericArgument::Type(t2)) => {
                                        can_assign(t1, t2, generics)
                                    }
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
    if let Type::Paren(paren) = ty {
        unwrap_paren(&paren.elem)
    } else {
        ty
    }
}

fn check_and_assign_type_generic(
    concrete_type: &str,
    declared_type: &str,
    generics: &mut ConstrainedGenerics,
) -> bool {
    if generics
        .types
        .get(declared_type)
        .cloned()
        .is_some_and(|assigned| {
            assigned.clone().is_none_or(|assigned| {
                can_assign(
                    &str_to_type_name(concrete_type),
                    &str_to_type_name(&assigned),
                    generics,
                )
            })
        })
    {
        generics
            .types
            .insert(declared_type.to_string(), Some(concrete_type.to_string()));
        return true;
    }

    concrete_type == declared_type || declared_type == "_"
}

fn check_and_assign_lifetime_generic(
    concrete_lifetime: &Option<String>,
    declared_lifetime: &Option<String>,
    generics: &mut ConstrainedGenerics,
) -> bool {
    if declared_lifetime.as_ref().is_some_and(|d| {
        generics
            .lifetimes
            .get(d)
            .cloned()
            .is_some_and(|assigned| assigned.clone().is_none_or(|assigned| d == &assigned))
    }) {
        generics.lifetimes.insert(
            declared_lifetime.as_ref().unwrap().clone(),
            concrete_lifetime.clone(),
        );
        return true;
    }

    declared_lifetime.as_ref().is_none_or(|v| v == "_")
        || concrete_lifetime.as_ref().is_some_and(|c| c == "'static")
}

pub fn type_contains(ty: &Type, generic: &str) -> bool {
    let mut type_ = ty.clone();
    let replacement = str_to_type_name("__G__");

    replace_type(&mut type_, generic, &replacement);

    to_string(&type_) != to_string(ty)
}

pub fn type_contains_lifetime(ty: &Type, lifetime: &str) -> bool {
    let mut type_ = ty.clone();
    let replacement = "'__G__";

    replace_lifetime(&mut type_, lifetime, replacement);

    to_string(&type_) != to_string(ty)
}

/// Replaces all occurrences of `prev` in the given type with `new`.
pub fn replace_type(ty: &mut Type, prev: &str, new: &Type) {
    if to_string(ty) == to_string(&str_to_type_name(prev)) {
        *ty = new.clone();
        return;
    }

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

        // _
        Type::Infer(_) if prev == "_" => {
            *ty = new.clone();
        }

        // T, T<U>
        Type::Path(type_path) => {
            // T
            if type_path.qself.is_none()
                && type_path.path.segments.len() == 1
                && type_path.path.segments[0].ident == prev
                && type_path.path.segments[0].arguments.is_empty()
            {
                *ty = new.clone();
                return;
            }

            // T<U>
            for seg in &mut type_path.path.segments {
                // T
                if seg.ident == prev {
                    seg.ident = Ident::new(&to_string(&new.clone()), Span::call_site());
                }

                // <U>
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

/// Replaces all occurrences of `prev` lifetime in the given type with `new`.
pub fn replace_lifetime(ty: &mut Type, prev: &str, new: &str) {
    match ty {
        Type::Reference(r) => {
            if let Some(lifetime) = &r.lifetime
                && lifetime.to_string() == prev
            {
                r.lifetime = Some(str_to_lifetime(new));
            }
            replace_lifetime(&mut r.elem, prev, new);
        }
        Type::Tuple(t) => {
            for elem in &mut t.elems {
                replace_lifetime(elem, prev, new);
            }
        }
        Type::Array(a) => replace_lifetime(&mut a.elem, prev, new),
        Type::Slice(s) => replace_lifetime(&mut s.elem, prev, new),
        Type::Paren(p) => replace_lifetime(&mut p.elem, prev, new),
        Type::Path(type_path) => {
            for seg in &mut type_path.path.segments {
                if let PathArguments::AngleBracketed(ref mut ab) = seg.arguments {
                    for arg in ab.args.iter_mut() {
                        if let GenericArgument::Type(inner_ty) = arg {
                            replace_lifetime(inner_ty, prev, new);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// removes all lifetimes present in generics
pub fn strip_lifetimes(ty: &mut Type, generics: &Generics) {
    match ty {
        // (T, U)
        Type::Tuple(t) => {
            for elem in &mut t.elems {
                strip_lifetimes(elem, generics);
            }
        }

        // &T
        Type::Reference(r) => {
            let generics_lifetimes = collect_generics_lifetimes::<HashSet<_>>(generics);

            if r.lifetime
                .as_ref()
                .is_some_and(|l| generics_lifetimes.contains(&l.to_string()))
            {
                r.lifetime = None;
            }

            strip_lifetimes(&mut r.elem, generics);
        }

        // [T; N]
        Type::Array(a) => strip_lifetimes(&mut a.elem, generics),

        // [T]
        Type::Slice(s) => strip_lifetimes(&mut s.elem, generics),

        // (T)
        Type::Paren(s) => strip_lifetimes(&mut s.elem, generics),

        // T, T<U>
        Type::Path(type_path) => {
            for seg in &mut type_path.path.segments {
                if let PathArguments::AngleBracketed(ref mut ab) = seg.arguments {
                    for arg in ab.args.iter_mut() {
                        if let GenericArgument::Type(inner_ty) = arg {
                            strip_lifetimes(inner_ty, generics);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// replaces all lifetimes with the most specific one in two types
/// the two types must be assignable
pub fn assign_lifetimes(t1: &mut Type, t2: &Type, generics: &mut ConstrainedGenerics) {
    match (t1, t2) {
        // `(T, U)`, `(T, _)`
        (Type::Tuple(tuple1), Type::Tuple(tuple2)) => tuple1
            .elems
            .iter_mut()
            .zip(&tuple2.elems)
            .for_each(|(elem1, elem2)| assign_lifetimes(elem1, elem2, generics)),

        // `&T`, `&_`
        (Type::Reference(ref1), Type::Reference(ref2)) => {
            let lt1 = ref1.lifetime.as_ref().map(to_string);
            let lt2 = ref2.lifetime.as_ref().map(to_string);

            if let Some(lt2) = lt2 {
                if let Some(lt1) = lt1 {
                    if let Some(corresponding) = generics.lifetimes.get(&lt2).cloned().flatten() {
                        ref1.lifetime = Some(str_to_lifetime(&corresponding));
                    } else if lt1 != "'static" {
                        ref1.lifetime = Some(str_to_lifetime(&lt2));
                    }
                } else {
                    ref1.lifetime = Some(str_to_lifetime(&lt2));
                }
            }

            assign_lifetimes(&mut ref1.elem, &ref2.elem, generics);
        }

        // `[T]`, `[_]`
        (Type::Slice(slice1), Type::Slice(slice2)) => {
            assign_lifetimes(&mut slice1.elem, &slice2.elem, generics);
        }

        // (T)
        (Type::Paren(paren1), Type::Paren(paren2)) => {
            assign_lifetimes(&mut paren1.elem, &paren2.elem, generics);
        }

        // `[T; N]`, `[_; N]`, `[T; _]`, `[_; _]`
        (Type::Array(array1), Type::Array(array2)) => {
            assign_lifetimes(&mut array1.elem, &array2.elem, generics);
        }

        // `T`, `T<U>`, `T<_>`
        (Type::Path(path1), Type::Path(path2)) => path1
            .path
            .segments
            .iter_mut()
            .zip(&path2.path.segments)
            .for_each(|(seg1, seg2)| {
                if let (
                    PathArguments::AngleBracketed(args1),
                    PathArguments::AngleBracketed(args2),
                ) = (&mut seg1.arguments, &seg2.arguments)
                {
                    args1
                        .args
                        .iter_mut()
                        .zip(&args2.args)
                        .for_each(|(arg1, arg2)| {
                            if let (GenericArgument::Type(t1), GenericArgument::Type(t2)) =
                                (arg1, arg2)
                            {
                                assign_lifetimes(t1, t2, generics);
                            }
                        })
                };
            }),

        _ => {}
    }
}

// TODO: use replace_type to simplify this function
/// Replaces all occurrences of `_` (inferred types) in the given type with fresh generic type parameters.
pub fn replace_infers(
    ty: &mut Type,
    generics: &mut HashSet<String>,
    counter: &mut usize,
    new_generics: &mut Vec<String>,
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
            let name = get_unique_generic_name(generics, counter, None);
            *ty = str_to_type_name(&name);
            new_generics.push(name);
        }

        _ => {}
    }
}

pub fn get_unique_generic_name(
    generics: &mut HashSet<String>,
    counter: &mut usize,
    prefix: Option<&str>,
) -> String {
    let prefix = prefix.unwrap_or_default();
    loop {
        let candidate = format!("{}__G_{}__", prefix, *counter);
        *counter += 1;

        if generics.insert(candidate.clone()) {
            return candidate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse2;

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
        assert_eq!(
            to_string(&resolved).replace(" ", ""),
            "Option<(u8,Vec<u8>)>"
        );
    }

    #[test]
    fn compare_types_simple() {
        let mut g = ConstrainedGenerics::default();

        let t1 = str_to_type_name("u8");
        let t2 = str_to_type_name("_");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("u8");
        let t2 = str_to_type_name("T");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("T");
        let t2 = str_to_type_name("T");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        g.types.insert("U".to_string(), None);
        let t1 = str_to_type_name("T");
        let t2 = str_to_type_name("U");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("T");
        let t2 = str_to_type_name("_");
        assert!(can_assign(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_tuples() {
        let mut g = ConstrainedGenerics::default();

        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(u8, _)");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(u8, T)");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("T");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(u8, i32)");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(u8, f32)");
        assert!(!can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(T, T)");
        assert!(!can_assign(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_references() {
        let mut g = ConstrainedGenerics::default();

        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&u8");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&_");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&T");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("&i8");
        let t2 = str_to_type_name("T");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&i8");
        assert!(!can_assign(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_references_with_lifetimes() {
        let mut g = ConstrainedGenerics::default();

        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&u8");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("&'a u8");
        let t2 = str_to_type_name("&u8");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("&'static u8");
        let t2 = str_to_type_name("&u8");
        assert!(can_assign(&t1, &t2, &mut g));

        g.lifetimes.insert("'a".to_string(), None);
        let t1 = str_to_type_name("&'a u8");
        let t2 = str_to_type_name("&'a u8");
        assert!(can_assign(&t1, &t2, &mut g));

        g.lifetimes.insert("'a".to_string(), None);
        let t1 = str_to_type_name("&'a u8");
        let t2 = str_to_type_name("&'a _");
        assert!(can_assign(&t1, &t2, &mut g));

        g.lifetimes.insert("'a".to_string(), None);
        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("&'a u8");
        let t2 = str_to_type_name("&'a T");
        assert!(can_assign(&t1, &t2, &mut g));

        g.lifetimes.insert("'b".to_string(), None);
        let t1 = str_to_type_name("&'a u8");
        let t2 = str_to_type_name("&'b u8");
        assert!(can_assign(&t1, &t2, &mut g));

        g.lifetimes.insert("'a".to_string(), None);
        let t1 = str_to_type_name("&'a u8");
        let t2 = str_to_type_name("&'static u8");
        assert!(!can_assign(&t1, &t2, &mut g));

        g.lifetimes.insert("'a".to_string(), None);
        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&'static u8");
        assert!(!can_assign(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_slices() {
        let mut g = ConstrainedGenerics::default();

        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[u8]");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[_]");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[T]");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("T");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[i8]");
        assert!(!can_assign(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_arrays() {
        let mut g = ConstrainedGenerics::default();

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[u8; 3]");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[u8; 4]");
        assert!(!can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[_; 3]");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[u8; _]");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[_; _]");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[T; 3]");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("T");
        assert!(can_assign(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_parens() {
        let mut g = ConstrainedGenerics::default();

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((u8))");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("(u8)");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((i32))");
        assert!(!can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((_))");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((T))");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("T");
        assert!(can_assign(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_paths() {
        let mut g = ConstrainedGenerics::default();

        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("Vec<u8>");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("Vec<_>");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("_");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("Vec<T>");
        assert!(can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("T");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("Vec<i32>");
        assert!(!can_assign(&t1, &t2, &mut g));
    }

    #[test]
    fn compare_types_nested() {
        let mut g = ConstrainedGenerics::default();

        let t1 = str_to_type_name("Option<(u8, i32)>");
        let t2 = str_to_type_name("Option<(u8, _)>");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("Result<Vec<u8>, String>");
        let t2 = str_to_type_name("Result<Vec<_>, _>");
        assert!(can_assign(&t1, &t2, &mut g));

        let t1 = str_to_type_name("Result<Vec<u8>, String>");
        let t2 = str_to_type_name("Result<Vec<i32>, String>");
        assert!(!can_assign(&t1, &t2, &mut g));

        g.types.insert("T".to_string(), None);
        let t1 = str_to_type_name("Result<Vec<u8>, String>");
        let t2 = str_to_type_name("Result<T, T>");
        assert!(!can_assign(&t1, &t2, &mut g));
    }

    #[test]
    fn contains_type_true() {
        let types = vec![
            "T",
            "(T, Other)",
            "&T",
            "[T; 3]",
            "&[T]",
            "(T)",
            "Other<T>",
            "T<Other>",
        ];
        for ty in types {
            let type_ = str_to_type_name(ty);
            assert!(type_contains(&type_, "T"));
        }
    }

    #[test]
    fn contains_type_false() {
        let types = vec![
            "T",
            "(T, Other)",
            "&T",
            "[T; 3]",
            "&[T]",
            "(T)",
            "Other<T>",
            "T<VOther>",
        ];
        for ty in types {
            let type_ = str_to_type_name(ty);
            assert!(!type_contains(&type_, "U"));
        }
    }

    #[test]
    fn replace_type_simple() {
        let mut ty: Type = parse2(quote! { T }).unwrap();
        let new_ty: Type = parse2(quote! { String }).unwrap();

        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "String".to_string());
    }

    #[test]
    fn replace_type_tuple() {
        let new_ty: Type = parse2(quote! { String }).unwrap();

        let mut ty: Type = parse2(quote! { (T, Other, T) }).unwrap();
        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "(String, Other, String)".to_string().replace(" ", "")
        );

        let mut ty: Type = parse2(quote! { (T, Other, T) }).unwrap();
        replace_type(&mut ty, "(T, Other, T)", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "String".to_string());
    }

    #[test]
    fn replace_type_reference() {
        let new_ty: Type = parse2(quote! { String }).unwrap();

        let mut ty: Type = parse2(quote! { &T }).unwrap();
        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "&String".to_string().replace(" ", "")
        );

        let mut ty: Type = parse2(quote! { &T }).unwrap();
        replace_type(&mut ty, "&T", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "String".to_string());
    }

    #[test]
    fn replace_type_array() {
        let new_ty: Type = parse2(quote! { String }).unwrap();

        let mut ty: Type = parse2(quote! { [T; 3] }).unwrap();
        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "[String; 3]".to_string().replace(" ", "")
        );

        let mut ty: Type = parse2(quote! { [T; 3] }).unwrap();
        replace_type(&mut ty, "[T; 3]", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "String".to_string());
    }

    #[test]
    fn replace_type_slice() {
        let new_ty: Type = parse2(quote! { String }).unwrap();

        let mut ty: Type = parse2(quote! { &[T] }).unwrap();
        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "&[String]".to_string().replace(" ", "")
        );

        let mut ty: Type = parse2(quote! { &[T] }).unwrap();
        replace_type(&mut ty, "&[T]", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "String".to_string());
    }

    #[test]
    fn replace_type_paren() {
        let new_ty: Type = parse2(quote! { String }).unwrap();

        let mut ty: Type = parse2(quote! { (T) }).unwrap();
        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "(String)".to_string().replace(" ", "")
        );

        let mut ty: Type = parse2(quote! { (T) }).unwrap();
        replace_type(&mut ty, "(T)", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "String".to_string());
    }

    #[test]
    fn replace_type_path() {
        let new_ty: Type = parse2(quote! { String }).unwrap();

        let mut ty: Type = parse2(quote! { Option<T> }).unwrap();
        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "Option<String>".to_string().replace(" ", "")
        );

        let mut ty: Type = parse2(quote! { Option<T> }).unwrap();
        replace_type(&mut ty, "Option<T>", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "String".to_string());
    }

    #[test]
    fn replace_type_nested() {
        let new_ty: Type = parse2(quote! { String }).unwrap();

        let mut ty: Type = parse2(quote! { Option<(T, &[T], T<T<i32>>)> }).unwrap();
        replace_type(&mut ty, "T", &new_ty);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "Option<(String, &[String], String<String<i32>>)>"
                .to_string()
                .replace(" ", "")
        );

        let mut ty: Type = parse2(quote! { Option<(T, &[T], T<T<i32>>)> }).unwrap();
        replace_type(&mut ty, "Option<(T, &[T], T<T<i32>>)>", &new_ty);

        assert_eq!(to_string(&ty).replace(" ", ""), "String".to_string());
    }

    #[test]
    fn replace_infers_simple() {
        let mut ty: Type = parse2(quote! { _ }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "__G_0__".to_string().replace(" ", "")
        );
        assert_eq!(new_generics, vec!["__G_0__".to_string()]);
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
            "(__G_0__, Other, __G_1__)".to_string().replace(" ", "")
        );
        assert_eq!(
            new_generics,
            vec!["__G_0__".to_string(), "__G_1__".to_string()]
        );
    }

    #[test]
    fn replace_infers_reference() {
        let mut ty: Type = parse2(quote! { &_ }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "&__G_0__".to_string().replace(" ", "")
        );
        assert_eq!(new_generics, vec!["__G_0__".to_string()]);
    }

    #[test]
    fn replace_infers_array() {
        let mut ty: Type = parse2(quote! { [_; 3] }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "[__G_0__; 3]".to_string().replace(" ", "")
        );
        assert_eq!(new_generics, vec!["__G_0__".to_string()]);
    }

    #[test]
    fn replace_infers_slice() {
        let mut ty: Type = parse2(quote! { &[_] }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "&[__G_0__]".to_string().replace(" ", "")
        );
        assert_eq!(new_generics, vec!["__G_0__".to_string()]);
    }

    #[test]
    fn replace_infers_paren() {
        let mut ty: Type = parse2(quote! { (_) }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "(__G_0__)".to_string().replace(" ", "")
        );
        assert_eq!(new_generics, vec!["__G_0__".to_string()]);
    }

    #[test]
    fn replace_infers_path() {
        let mut ty: Type = parse2(quote! { Option<_> }).unwrap();
        let mut generics = HashSet::new();
        let mut counter = 0;
        let mut new_generics = vec![];

        replace_infers(&mut ty, &mut generics, &mut counter, &mut new_generics);

        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "Option<__G_0__>".to_string().replace(" ", "")
        );
        assert_eq!(new_generics, vec!["__G_0__".to_string()]);
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
            "Option<(__G_0__, &[__G_1__])>".to_string().replace(" ", "")
        );
        assert_eq!(
            new_generics,
            vec!["__G_0__".to_string(), "__G_1__".to_string()]
        );
    }

    #[test]
    fn strip_lifetimes_simple() {
        let mut ty: Type = parse2(quote! { &'a u8 }).unwrap();
        let generics = str_to_generics("<'a>");
        strip_lifetimes(&mut ty, &generics);
        assert_eq!(to_string(&ty).replace(" ", ""), "&u8");
    }

    #[test]
    fn strip_lifetimes_tuple() {
        let mut ty: Type = parse2(quote! { (&'a u8, &'b i32) }).unwrap();
        let generics = str_to_generics("<'b>");
        strip_lifetimes(&mut ty, &generics);
        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "(&'a u8, &i32)".replace(" ", "")
        );
    }

    #[test]
    fn strip_lifetimes_array() {
        let mut ty: Type = parse2(quote! { [&'a u8; 3] }).unwrap();
        let generics = str_to_generics("<'a>");
        strip_lifetimes(&mut ty, &generics);
        assert_eq!(to_string(&ty).replace(" ", ""), "[&u8; 3]".replace(" ", ""));
    }

    #[test]
    fn strip_lifetimes_slice() {
        let mut ty: Type = parse2(quote! { &'a [u8] }).unwrap();
        let generics = str_to_generics("<'a>");
        strip_lifetimes(&mut ty, &generics);
        assert_eq!(to_string(&ty).replace(" ", ""), "&[u8]");
    }

    #[test]
    fn strip_lifetimes_nested() {
        let mut ty: Type = parse2(quote! { Option<&'a (u8, &'b i32)> }).unwrap();
        let generics = str_to_generics("<'a, 'b>");
        strip_lifetimes(&mut ty, &generics);
        assert_eq!(
            to_string(&ty).replace(" ", ""),
            "Option<&(u8, &i32)>".replace(" ", "")
        );
    }

    #[test]
    fn assign_lifetimes_simple() {
        let mut t1: Type = parse2(quote! { &'a u8 }).unwrap();
        let t2: Type = parse2(quote! { &'static u8 }).unwrap();
        let mut generics = ConstrainedGenerics::from(str_to_generics(""));
        assign_lifetimes(&mut t1, &t2, &mut generics);
        assert_eq!(
            to_string(&t1).replace(" ", ""),
            "&'static u8".replace(" ", "")
        );

        let mut t1: Type = parse2(quote! { &'a u8 }).unwrap();
        let t2: Type = parse2(quote! { &'b u8 }).unwrap();
        let mut generics = ConstrainedGenerics::from(str_to_generics("<'b>"));
        assign_lifetimes(&mut t1, &t2, &mut generics);
        assert_eq!(to_string(&t1).replace(" ", ""), "&'b u8".replace(" ", ""));

        let mut t1: Type = parse2(quote! { &u8 }).unwrap();
        let t2: Type = parse2(quote! { &'a u8 }).unwrap();
        let mut generics = ConstrainedGenerics::from(str_to_generics("<'a>"));
        assign_lifetimes(&mut t1, &t2, &mut generics);
        assert_eq!(to_string(&t1).replace(" ", ""), "&'a u8".replace(" ", ""));

        let mut t1: Type = parse2(quote! { &u8 }).unwrap();
        let t2: Type = parse2(quote! { &'static u8 }).unwrap();
        let mut generics = ConstrainedGenerics::from(str_to_generics(""));
        assign_lifetimes(&mut t1, &t2, &mut generics);
        assert_eq!(
            to_string(&t1).replace(" ", ""),
            "&'static u8".replace(" ", "")
        );
    }

    #[test]
    fn assign_lifetimes_tuple() {
        let mut t1: Type = parse2(quote! { (&'a u8, &'b i32) }).unwrap();
        let t2: Type = parse2(quote! { (&'static u8, &'static i32) }).unwrap();
        let mut generics = ConstrainedGenerics::from(str_to_generics(""));
        assign_lifetimes(&mut t1, &t2, &mut generics);
        assert_eq!(
            to_string(&t1).replace(" ", ""),
            "(&'static u8, &'static i32)".replace(" ", "")
        );
    }

    #[test]
    fn assign_lifetimes_array() {
        let mut t1: Type = parse2(quote! { [&'a u8; 3] }).unwrap();
        let t2: Type = parse2(quote! { [&'static u8; 3] }).unwrap();
        let mut generics = ConstrainedGenerics::from(str_to_generics(""));
        assign_lifetimes(&mut t1, &t2, &mut generics);
        assert_eq!(
            to_string(&t1).replace(" ", ""),
            "[&'static u8; 3]".replace(" ", "")
        );
    }

    #[test]
    fn assign_lifetimes_slice() {
        let mut t1: Type = parse2(quote! { &'a [u8] }).unwrap();
        let t2: Type = parse2(quote! { &'static [u8] }).unwrap();
        let mut generics = ConstrainedGenerics::from(str_to_generics(""));
        assign_lifetimes(&mut t1, &t2, &mut generics);
        assert_eq!(
            to_string(&t1).replace(" ", ""),
            "&'static [u8]".replace(" ", "")
        );
    }

    #[test]
    fn assign_lifetimes_nested() {
        let mut t1: Type = parse2(quote! { Option<&'a (u8, &'b i32)> }).unwrap();
        let t2: Type = parse2(quote! { Option<&'static (u8, &'static i32)> }).unwrap();
        let mut generics = ConstrainedGenerics::from(str_to_generics(""));
        assign_lifetimes(&mut t1, &t2, &mut generics);
        assert_eq!(
            to_string(&t1).replace(" ", ""),
            "Option<&'static (u8, &'static i32)>".replace(" ", "")
        );

        let mut t1: Type = parse2(quote! { &'a Option<&'a u8> }).unwrap();
        let t2: Type = parse2(quote! { &'b Option<&'static u8> }).unwrap();
        let mut generics = ConstrainedGenerics::from(str_to_generics("<'b>"));
        generics
            .lifetimes
            .insert("'b".to_string(), Some("'static".to_string()));
        assign_lifetimes(&mut t1, &t2, &mut generics);
        assert_eq!(
            to_string(&t1).replace(" ", ""),
            "&'static Option<&'static u8>".replace(" ", "")
        );
    }
}
