use spec_trait_utils::conversions::{ str_to_type_name, to_string };
use syn::{ Type, TypeTuple, TypeReference, TypeArray };
use crate::vars::VarInfo;

pub fn get_concrete_type(type_or_alias: &str, vars: &[VarInfo]) -> String {
    let parsed_type = str_to_type_name(type_or_alias);
    let resolved_type = resolve_type(&parsed_type, vars);
    to_string(&resolved_type)
}

fn resolve_type(ty: &Type, vars: &[VarInfo]) -> Type {
    match unwrap_paren(ty) {
        #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

        // (T, U)
        Type::Tuple(tuple) => {
            let resolved_elems = tuple.elems
                .iter()
                .map(|elem| resolve_type(elem, vars))
                .collect();
            Type::Tuple(TypeTuple {
                elems: resolved_elems,
                ..tuple.clone()
            })
        }

        // &T
        Type::Reference(reference) => {
            let resolved_elem = resolve_type(&reference.elem, vars);
            Type::Reference(TypeReference {
                elem: Box::new(resolved_elem),
                ..reference.clone()
            })
        }

        // [T; N]
        Type::Array(array) => {
            let resolved_elem = resolve_type(&array.elem, vars);
            Type::Array(TypeArray {
                elem: Box::new(resolved_elem),
                ..array.clone()
            })
        }

        // [T]
        Type::Slice(slice) => {
            let resolved_elem = resolve_type(&slice.elem, vars);
            Type::Slice(syn::TypeSlice {
                elem: Box::new(resolved_elem),
                ..slice.clone()
            })
        }

        // T, T<U>
        Type::Path(type_path) if type_path.qself.is_none() => {
            let mut resolved_path = type_path.clone();

            let ident = type_path.path.segments.last().unwrap().ident.to_string();
            if let Some(alias) = vars.iter().find(|v| v.type_aliases.contains(&ident)) {
                return str_to_type_name(&alias.concrete_type);
            }

            for segment in &mut resolved_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in &mut args.args {
                        if let syn::GenericArgument::Type(inner_ty) = arg {
                            *inner_ty = resolve_type(inner_ty, vars);
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

/// types can be something like: "T", "&T", "U<T>", "(T, T)", "&[T]"
/// each of the "T" can be a type or a "_", which means any type
pub fn types_equal(type1: &str, type2: &str, vars: &[VarInfo]) -> bool {
    let t1 = str_to_type_name(&get_concrete_type(type1, vars));
    let t2 = str_to_type_name(&get_concrete_type(type2, vars));
    println!("{} == {} ? {}", to_string(&t1), to_string(&t2), compare_types(&t1, &t2));
    compare_types(&t1, &t2)
}

fn compare_types(t1: &Type, t2: &Type) -> bool {
    let t1 = unwrap_paren(t1);
    let t2 = unwrap_paren(t2);

    match (t1, t2) {
        #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

        // `_`
        (t1, t2) if matches!(t1, Type::Infer(_)) || matches!(t2, Type::Infer(_)) => true,

        // `(T, U)`, `(T, _)`
        (Type::Tuple(tuple1), Type::Tuple(tuple2)) => {
            tuple1.elems.len() == tuple2.elems.len() &&
                tuple1.elems
                    .iter()
                    .zip(&tuple2.elems)
                    .all(|(elem1, elem2)| compare_types(elem1, elem2))
        }

        // `&T`, `&_`
        (Type::Reference(ref1), Type::Reference(ref2)) => { compare_types(&ref1.elem, &ref2.elem) }

        // `[T]`, `[_]`
        (Type::Slice(slice1), Type::Slice(slice2)) => { compare_types(&slice1.elem, &slice2.elem) }

        // `[T; N]`, `[_; N]`, `[T; _]`, `[_; _]`
        (Type::Array(array1), Type::Array(array2)) => {
            compare_types(&array1.elem, &array2.elem) &&
                (matches!(array1.len, syn::Expr::Infer(_)) ||
                    matches!(array2.len, syn::Expr::Infer(_)) ||
                    to_string(&array1.len) == to_string(&array2.len))
        }

        // T, `T<U>`, `T<_>`
        (Type::Path(path1), Type::Path(path2)) if path1.qself.is_none() && path2.qself.is_none() => {
            path1.path.segments.len() == path2.path.segments.len() &&
                path1.path.segments
                    .iter()
                    .zip(&path2.path.segments)
                    .all(|(seg1, seg2)| {
                        seg1.ident == seg2.ident &&
                            (match (&seg1.arguments, &seg2.arguments) {
                                (
                                    syn::PathArguments::AngleBracketed(args1),
                                    syn::PathArguments::AngleBracketed(args2),
                                ) =>
                                    args1.args
                                        .iter()
                                        .zip(&args2.args)
                                        .all(|(arg1, arg2)| {
                                            match (arg1, arg2) {
                                                (
                                                    syn::GenericArgument::Type(ty1),
                                                    syn::GenericArgument::Type(ty2),
                                                ) => compare_types(ty1, ty2),
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

#[cfg(test)]
mod tests {
    use super::*;
    use spec_trait_utils::conversions::str_to_type_name;

    fn get_var_info() -> Vec<VarInfo> {
        vec![VarInfo {
            type_definition: "T".to_string(),
            concrete_type: "u8".to_string(),
            type_aliases: vec!["MyType".to_string()],
            traits: vec![],
        }]
    }

    #[test]
    fn resolve_type_simple() {
        let ty = str_to_type_name("MyType");
        let resolved = resolve_type(&ty, &get_var_info());
        assert_eq!(to_string(&resolved), "u8");
    }

    #[test]
    fn resolve_type_tuples() {
        let ty = str_to_type_name("(MyType, u8)");
        let resolved = resolve_type(&ty, &get_var_info());
        assert_eq!(to_string(&resolved).replace(" ", ""), "(u8,u8)");
    }

    #[test]
    fn resolve_type_references() {
        let ty = str_to_type_name("&MyType");
        let resolved = resolve_type(&ty, &get_var_info());
        assert_eq!(to_string(&resolved).replace(" ", ""), "&u8");
    }

    #[test]
    fn resolve_type_arrays() {
        let ty = str_to_type_name("[MyType; 3]");
        let resolved = resolve_type(&ty, &get_var_info());
        assert_eq!(to_string(&resolved).replace(" ", ""), "[u8;3]");
    }

    #[test]
    fn resolve_type_slices() {
        let ty = str_to_type_name("[MyType]");
        let resolved = resolve_type(&ty, &get_var_info());
        assert_eq!(to_string(&resolved).replace(" ", ""), "[u8]");
    }

    #[test]
    fn resolve_type_parens() {
        let ty = str_to_type_name("(MyType)");
        let resolved = resolve_type(&ty, &get_var_info());
        assert_eq!(to_string(&resolved), "u8");
    }

    #[test]
    fn resolve_type_paths() {
        let ty = str_to_type_name("Vec<MyType>");
        let resolved = resolve_type(&ty, &get_var_info());
        assert_eq!(to_string(&resolved).replace(" ", ""), "Vec<u8>");
    }

    #[test]
    fn resolve_type_nested() {
        let ty = str_to_type_name("Option<(MyType, Vec<MyType>)>");
        let resolved = resolve_type(&ty, &get_var_info());
        assert_eq!(to_string(&resolved).replace(" ", ""), "Option<(u8,Vec<u8>)>");
    }

    #[test]
    fn compare_types_simple() {
        let t1 = str_to_type_name("_");
        let t2 = str_to_type_name("u8");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("u8");
        let t2 = str_to_type_name("_");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("_");
        let t2 = str_to_type_name("_");
        assert!(compare_types(&t1, &t2));
    }

    #[test]
    fn compare_types_tuples() {
        let t1 = str_to_type_name("(u8, _)");
        let t2 = str_to_type_name("(u8, i32)");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(u8, i32)");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("(u8, i32)");
        let t2 = str_to_type_name("(u8, f32)");
        assert!(!compare_types(&t1, &t2));
    }

    #[test]
    fn compare_types_references() {
        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&u8");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&_");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("&u8");
        let t2 = str_to_type_name("&i8");
        assert!(!compare_types(&t1, &t2));
    }

    #[test]
    fn compare_types_slices() {
        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[u8]");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[_]");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("[u8]");
        let t2 = str_to_type_name("[i8]");
        assert!(!compare_types(&t1, &t2));
    }

    #[test]
    fn compare_types_arrays() {
        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[u8; 3]");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[u8; 4]");
        assert!(!compare_types(&t1, &t2));

        let t1 = str_to_type_name("[u8; 3]");
        let t2 = str_to_type_name("[_; 3]");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("[u8; _]");
        let t2 = str_to_type_name("[u8; 3]");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("[_; _]");
        let t2 = str_to_type_name("[u8; 3]");
        assert!(compare_types(&t1, &t2));
    }

    #[test]
    fn compare_types_parens() {
        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((u8))");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("(u8)");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((i32))");
        assert!(!compare_types(&t1, &t2));

        let t1 = str_to_type_name("((u8))");
        let t2 = str_to_type_name("((_))");
        assert!(compare_types(&t1, &t2));
    }

    #[test]
    fn compare_types_paths() {
        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("Vec<u8>");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("Vec<_>");
        let t2 = str_to_type_name("Vec<u8>");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("Vec<u8>");
        let t2 = str_to_type_name("Vec<i32>");
        assert!(!compare_types(&t1, &t2));
    }

    #[test]
    fn compare_types_nested() {
        let t1 = str_to_type_name("Option<(u8, _)>");
        let t2 = str_to_type_name("Option<(u8, i32)>");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("Result<Vec<_>, _>");
        let t2 = str_to_type_name("Result<Vec<u8>, String>");
        assert!(compare_types(&t1, &t2));

        let t1 = str_to_type_name("Result<Vec<u8>, String>");
        let t2 = str_to_type_name("Result<Vec<i32>, String>");
        assert!(!compare_types(&t1, &t2));
    }
}
