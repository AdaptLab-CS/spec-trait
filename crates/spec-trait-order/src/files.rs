use std::path::PathBuf;
use std::fs;

use spec_trait_utils::conditions::WhenCondition;
use spec_trait_utils::impls::{ self, ImplBody };
use spec_trait_utils::traits::{ self, TraitBody };
use spec_trait_utils::cache::CrateCache;
use syn::{ Attribute, Item, Meta };
use quote::quote;

pub fn parse_all(paths: &[PathBuf]) -> CrateCache {
    let mut traits = Vec::new();
    let mut impls = Vec::new();

    for path in paths {
        let crate_cache = parse(path);
        traits.extend(crate_cache.traits);
        impls.extend(crate_cache.impls);
    }

    CrateCache { traits, impls }
}

pub fn parse(path: &PathBuf) -> CrateCache {
    let content = fs::read_to_string(path).expect("failed to read file");
    let file = syn::parse_file(&content).expect("failed to parse content");

    CrateCache {
        traits: get_traits(&file.items),
        impls: get_impls(&file.items),
    }
}

fn get_traits(items: &[Item]) -> Vec<TraitBody> {
    items
        .iter()
        .filter_map(|item| {
            match item {
                Item::Trait(trait_item) => Some(trait_item),
                _ => None,
            }
        })
        .map(|trait_| {
            let (trait_no_attrs, _) = traits::break_attr(trait_);
            let tokens = quote! { #trait_no_attrs };
            TraitBody::try_from(tokens).expect("Failed to parse TokenStream into TraitBody")
        })
        .collect()
}

fn get_impls(items: &[Item]) -> Vec<ImplBody> {
    items
        .iter()
        .filter_map(|item| {
            match item {
                Item::Impl(impl_item) => Some(impl_item),
                _ => None,
            }
        })
        .map(|impl_| {
            let (impl_no_attrs, impl_attrs) = impls::break_attr(impl_);
            let tokens = quote! { #impl_no_attrs };
            let condition = get_condition(&impl_attrs);
            ImplBody::try_from((tokens, condition)).expect(
                "Failed to parse TokenStream into ImplBody"
            )
        })
        .collect()
}

fn get_condition(attrs: &[Attribute]) -> Option<WhenCondition> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("when")) // TODO: handle use spec_trait_macro::{ when as ... }
        .and_then(|attr| {
            match attr.clone().meta {
                Meta::List(meta_list) => {
                    let params = meta_list.tokens;
                    let tokens = quote! { #params };
                    WhenCondition::try_from(tokens).ok()
                }
                _ => None,
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::path::Path;
    use syn::{ Item, ItemImpl };

    fn make_file(file_path: &Path, content: &str) {
        fs::write(&file_path, content).expect("write file");
    }

    #[test]
    fn test_parse_single_file() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let file_path = root.join("test.rs");

        let content =
            "
            trait Foo { fn foo(&self); }
            impl Foo for MyStruct { fn foo(&self) {} }
        ";

        make_file(&file_path, content);

        let crate_cache = parse(&file_path);

        assert_eq!(crate_cache.traits.len(), 1);
        assert_eq!(crate_cache.impls.len(), 1);
        assert_eq!(crate_cache.traits[0].name, "Foo");
        assert_eq!(crate_cache.impls[0].trait_name, "Foo");
    }

    #[test]
    fn test_parse_all_files() {
        let dir = tempdir().unwrap();
        let file1_path = dir.path().join("file1.rs");
        let file2_path = dir.path().join("file2.rs");

        make_file(&file1_path, "trait Foo { fn foo(&self); }");
        make_file(&file2_path, "trait Bar { fn bar(&self); }");

        let crate_cache = parse_all(&[file1_path, file2_path]);

        assert_eq!(crate_cache.traits.len(), 2);
        assert!(crate_cache.traits.iter().any(|t| t.name == "Foo"));
        assert!(crate_cache.traits.iter().any(|t| t.name == "Bar"));
    }

    #[test]
    fn test_get_traits() {
        let items = vec![
            syn::parse_str::<Item>("struct MyStruct;").unwrap(),
            syn::parse_str::<Item>("trait Foo { fn foo(&self); }").unwrap(),
            syn::parse_str::<Item>("#[test] trait Bar { fn bar(&self); }").unwrap(),
            syn::parse_str::<Item>("impl Foo for MyStruct { fn foo(&self) {} }").unwrap()
        ];

        let traits = get_traits(&items);

        assert_eq!(traits.len(), 2);
        assert!(traits.iter().any(|t| t.name == "Foo"));
        assert!(traits.iter().any(|t| t.name == "Bar"));
    }

    #[test]
    fn test_get_impls() {
        let items = vec![
            syn::parse_str::<Item>("struct MyStruct;").unwrap(),
            syn::parse_str::<Item>("trait Foo { fn foo(&self); }").unwrap(),
            syn::parse_str::<Item>("impl Foo for MyStruct { fn foo(&self) {} }").unwrap(),
            syn::parse_str::<Item>("#[test] impl Bar for MyStruct { fn bar(&self) {} }").unwrap()
        ];

        let impls = get_impls(&items);

        assert_eq!(impls.len(), 2);
        assert!(impls.iter().any(|t| t.trait_name == "Foo"));
        assert!(impls.iter().any(|t| t.trait_name == "Bar"));
    }

    #[test]
    fn test_get_condition() {
        let impl_ = syn
            ::parse_str::<ItemImpl>(
                "#[test] #[when(T = i32)] impl Foo<T> for MyStruct { fn foo(&self, x: T) {} }"
            )
            .unwrap();

        let (_, attributes) = impls::break_attr(&impl_);

        let condition = get_condition(&attributes);

        assert!(condition.is_some());
        let condition = condition.unwrap();
        assert_eq!(condition, WhenCondition::Type("T".to_string(), "i32".to_string()));
    }
}
