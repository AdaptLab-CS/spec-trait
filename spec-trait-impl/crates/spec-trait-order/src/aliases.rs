use std::collections::HashSet;
use syn::{Item, Path, UseTree};

const MACRO_PACKAGE: &str = "spec_trait_macro";
const MACRO_NAME: &str = "when";

pub fn collect_when_aliases(items: &[Item]) -> HashSet<String> {
    let mut set = HashSet::new();

    for item in items {
        if let Item::Use(item_use) = item {
            collect_aliases_from_tree(&item_use.tree, false, &mut set);
        }
    }

    set
}

fn collect_aliases_from_tree(tree: &UseTree, prefix_spec: bool, set: &mut HashSet<String>) {
    match tree {
        // `use spec_trait_macro::...`
        UseTree::Path(use_path) => {
            let new_prefix_spec = prefix_spec || use_path.ident == MACRO_PACKAGE;
            collect_aliases_from_tree(&use_path.tree, new_prefix_spec, set);
        }
        // `use spec_trait_macro::when;` or `use spec_trait_macro::{ when };`
        UseTree::Name(use_name) => {
            if prefix_spec && use_name.ident == MACRO_NAME {
                set.insert(use_name.ident.to_string());
            }
        }
        // `use spec_trait_macro::{ when as when_alias };`
        UseTree::Rename(use_rename) => {
            if prefix_spec && use_rename.ident == MACRO_NAME {
                set.insert(use_rename.rename.to_string());
            }
        }
        // `use spec_trait_macro::{ ... };`
        UseTree::Group(use_group) => {
            for t in &use_group.items {
                collect_aliases_from_tree(t, prefix_spec, set);
            }
        }
        // `use spec_trait_macro::*;`
        UseTree::Glob(_) => {
            if prefix_spec {
                set.insert(MACRO_NAME.to_string());
            }
        }
    }
}

pub fn is_when_macro(path: &Path, when_aliases: &HashSet<String>) -> bool {
    // `when` imported directly or via alias
    when_aliases.contains(&path.segments.last().unwrap().ident.to_string()) ||
        // `spec_trait_macro::when`
        (path.segments
            .last()
            .map(|s| s.ident == MACRO_NAME)
            .unwrap_or(false) &&
            path.segments
                .first()
                .map(|s| s.ident == MACRO_PACKAGE)
                .unwrap_or(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(s: &str) -> Item {
        syn::parse_str::<Item>(s).expect("failed to parse Item")
    }

    #[test]
    fn collect_when_simple() {
        let set = collect_when_aliases(&[item("use spec_trait_macro::when;")]);
        assert!(set.contains("when"));
    }

    #[test]
    fn collect_when_rename() {
        let set = collect_when_aliases(&[item("use spec_trait_macro::when as w;")]);
        assert!(set.contains("w"));
        assert!(!set.contains("when"));
    }

    #[test]
    fn collect_when_group() {
        let set = collect_when_aliases(&[item("use spec_trait_macro::{when as w, other};")]);
        assert!(set.contains("w"));
        assert!(!set.contains("when"));
    }

    #[test]
    fn collect_when_glob() {
        let set = collect_when_aliases(&[item("use spec_trait_macro::*;")]);
        assert!(set.contains("when"));
    }

    #[test]
    fn collect_when_other_package() {
        let set = collect_when_aliases(&[item("use other::when;")]);
        assert!(!set.contains("w"));
        assert!(!set.contains("when"));
    }
    #[test]
    fn is_when_macro_simple() {
        let mut aliases = HashSet::new();
        aliases.insert("when".to_string());
        let path: Path = syn::parse_str("when").unwrap();
        assert!(is_when_macro(&path, &aliases));
    }

    #[test]
    fn is_when_macro_alias() {
        let mut aliases = HashSet::new();
        aliases.insert("w".to_string());
        let path: Path = syn::parse_str("w").unwrap();
        assert!(is_when_macro(&path, &aliases));
    }

    #[test]
    fn is_when_macro_fully_qualified() {
        let aliases = HashSet::new();
        let path: Path = syn::parse_str("spec_trait_macro::when").unwrap();
        assert!(is_when_macro(&path, &aliases));
    }

    #[test]
    fn is_not_when_macro() {
        let aliases = HashSet::new();
        let path: Path = syn::parse_str("other::when").unwrap();
        assert!(!is_when_macro(&path, &aliases));
    }
}
