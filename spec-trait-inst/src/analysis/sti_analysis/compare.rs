use super::distance::compute_similarity;
use super::sti_visitor::{STIVisitor, VisitedFn, VisitedTrait};
use super::tree::BodyTree;
use super::Analyzer;
use rustc_hir::def_id::DefId;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

pub struct STIComparison<'tcx, 'a> {
    visitor: &'a STIVisitor<'tcx, 'a>,
    analyzer: &'a Analyzer<'tcx>,
}

pub struct SpecializablePair<'a> {
    pub fn_a: &'a VisitedFn,
    pub fn_b: &'a VisitedFn,
    pub similarity: f64,
}

pub struct SpecializableTraitPair<'a> {
    pub trait_a: &'a VisitedTrait,
    pub trait_b: &'a VisitedTrait,
    pub similarity: f64,
}

impl Display for SpecializablePair<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:^30.30} <---{:^6.2}---> {:^30.30}",
            self.fn_a, self.similarity, self.fn_b,
        )
    }
}

impl Display for SpecializableTraitPair<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:^30.30} <---{:^6.2}---> {:^30.30}",
            self.trait_a.name, self.similarity, self.trait_b.name,
        )
    }
}

impl<'tcx, 'a> STIComparison<'tcx, 'a> {
    pub fn new(analyzer: &'a Analyzer<'tcx>, visitor: &'a STIVisitor<'tcx, 'a>) -> Self {
        Self { analyzer, visitor }
    }

    /// Get pairs of specializable functions along with their similarity score.
    pub fn get_specializable_fns(&self) -> Vec<SpecializablePair<'a>> {
        let mut specializable = vec![];
        let fns = self.get_fns_with_tree();
        for (i, (fn_a, tree_a)) in fns.iter().enumerate() {
            log::trace!("processing function {}/{}", i + 1, fns.len());
            for (fn_b, tree_b) in fns.iter().skip(i + 1) {
                if specializable_fn_heuristic(fn_a, fn_b) {
                    if let Some(similarity) = self.is_specializable(tree_a, tree_b) {
                        specializable.push(SpecializablePair {
                            fn_a,
                            fn_b,
                            similarity,
                        });
                    }
                }
            }
        }
        specializable
    }

    /// Get pairs of specializable functions along with their similarity score.
    pub fn get_specializable_traits(
        &self,
        specializable_fns: &Vec<SpecializablePair<'a>>,
    ) -> Vec<SpecializableTraitPair<'a>> {
        let mut specializable = vec![];
        let mut traits = HashMap::new();

        let mut trait_info = HashMap::new();
        for f in &self.visitor.visited_fns {
            if let Some(tr) = f.parent_trait.as_ref() {
                let entry = trait_info.entry(tr.id).or_insert((tr, 0usize));
                entry.1 += 1;
            }
        }

        fn key(trait_a: &VisitedTrait, trait_b: &VisitedTrait) -> (DefId, DefId) {
            if trait_a.id.index <= trait_b.id.index {
                (trait_a.id, trait_b.id)
            } else {
                (trait_b.id, trait_a.id)
            }
        }

        for pair in specializable_fns {
            if let (Some(trait_a), Some(trait_b)) = (
                pair.fn_a.parent_trait.as_ref(),
                pair.fn_b.parent_trait.as_ref(),
            ) {
                let trait_a_items = trait_info.get(&trait_a.id).map(|(_, c)| *c).unwrap_or(0);
                let trait_b_items = trait_info.get(&trait_b.id).map(|(_, c)| *c).unwrap_or(0);

                if trait_a.items == trait_b.items && trait_a_items == trait_b_items {
                    if let Some(similarity) = traits.get_mut(&key(trait_a, trait_b)) {
                        *similarity *= pair.similarity;
                    } else {
                        traits.insert(key(trait_a, trait_b), pair.similarity);
                    }
                }
            }
        }

        for ((id_a, id_b), similarity) in traits {
            const THRESHOLD: f64 = 0.9;

            if similarity >= THRESHOLD {
                specializable.push(SpecializableTraitPair {
                    trait_a: trait_info.get(&id_a).unwrap().0,
                    trait_b: trait_info.get(&id_b).unwrap().0,
                    similarity,
                });
            }
        }

        specializable
    }

    /// Determine if two body trees are specializable based on their similarity.
    /// If they are, return the similarity score.
    fn is_specializable(&self, tree_a: &BodyTree, tree_b: &BodyTree) -> Option<f64> {
        const THRESHOLD: f64 = 0.9;

        let a_size = tree_a.root.size();
        let b_size = tree_b.root.size();

        let max_size = a_size.max(b_size) as f64;
        let min_size = a_size.min(b_size) as f64;

        // we would at least need to remove (max_size - min_size) nodes
        if min_size / max_size < THRESHOLD {
            return None;
        }

        let similarity = compute_similarity(&tree_a.root, &tree_b.root);
        if similarity >= THRESHOLD {
            Some(similarity)
        } else {
            None
        }
    }

    /// Retrieve all visited functions along with their body trees.
    fn get_fns_with_tree(&self) -> Vec<(&'a VisitedFn, BodyTree<'tcx, 'a>)> {
        self.visitor
            .visited_fns
            .iter()
            .filter_map(|visited_fn| {
                let tree = BodyTree::new(self.analyzer, &visited_fn.body);
                if tree.root.children.is_empty() {
                    None
                } else {
                    Some((visited_fn, tree))
                }
            })
            .collect()
    }
}

/// A heuristic to quickly determine if two functions could be specializable.
fn specializable_fn_heuristic(fn_a: &VisitedFn, fn_b: &VisitedFn) -> bool {
    fn_a.args == fn_b.args
        && names_similar(&fn_a.name, &fn_b.name)
        && match (fn_a.parent_trait.as_ref(), fn_b.parent_trait.as_ref()) {
            (None, None) => true,
            (Some(tr_a), Some(tr_b)) => specializable_trait_heuristic(tr_a, tr_b),
            _ => false,
        }
        && match (fn_a.self_type.as_ref(), fn_b.self_type.as_ref()) {
            (None, None) => true,
            (Some(ty_a), Some(ty_b)) => names_similar(ty_a, ty_b),
            _ => false,
        }
}

/// A heuristic to quickly determine if two traits could be specializable.
fn specializable_trait_heuristic(tr_a: &VisitedTrait, tr_b: &VisitedTrait) -> bool {
    tr_a.items == tr_b.items && names_similar(&tr_a.name, &tr_b.name)
}

/// Check if two function names are similar based on shared underscore-separated parts.
fn names_similar(a: &str, b: &str) -> bool {
    let parts_a = get_name_parts(a);
    let parts_b = get_name_parts(b);

    if parts_a.is_empty() || parts_b.is_empty() {
        return false;
    }

    let set_a = parts_a.iter().collect::<HashSet<_>>();
    let set_b = parts_b.iter().collect::<HashSet<_>>();

    let common = set_a.intersection(&set_b).count();
    let len = parts_a.len().max(parts_b.len());

    (common as f64) / (len as f64) >= 0.5
}

/// Split a name into its snake_case parts.
fn get_name_parts(name: &str) -> Vec<String> {
    to_snake_case(name)
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

/// Convert a given name to snake_case.
fn to_snake_case(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut snake_case = String::new();

    for i in 0..chars.len() {
        let ch = chars[i];

        if ch == '_' {
            snake_case.push('_');
        } else if ch.is_uppercase() {
            if i != 0 {
                let prev = chars[i - 1];
                let next_is_lower = chars.get(i + 1).map(|c| c.is_lowercase()).unwrap_or(false);

                if (prev.is_lowercase() || (prev.is_uppercase() && next_is_lower))
                    && !snake_case.ends_with('_')
                {
                    snake_case.push('_');
                }
            }

            for lower_ch in ch.to_lowercase() {
                snake_case.push(lower_ch);
            }
        } else {
            snake_case.push(ch);
        }
    }

    snake_case
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_hir::def_id::{DefIndex, LocalDefId};
    use rustc_hir::{BodyId, HirId, ItemLocalId, OwnerId};
    use rustc_index::Idx;

    fn get_visited_trait(name: &str, items: usize) -> VisitedTrait {
        VisitedTrait {
            name: name.to_string(),
            id: DefId::local(DefIndex::from_usize(0)),
            items,
        }
    }

    fn get_visited_fn(name: &str, args: usize, parent_trait: Option<VisitedTrait>) -> VisitedFn {
        VisitedFn {
            name: name.to_string(),
            args,
            body: BodyId {
                hir_id: HirId {
                    owner: OwnerId {
                        def_id: LocalDefId::new(0),
                    },
                    local_id: ItemLocalId::ZERO,
                },
            },
            parent_trait,
            self_type: None,
        }
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("camelCase"), "camel_case");
        assert_eq!(to_snake_case("PascalCase"), "pascal_case");
        assert_eq!(to_snake_case("snake_case"), "snake_case");
        assert_eq!(to_snake_case("CAPS_SNAKE_CASE"), "caps_snake_case");
        assert_eq!(to_snake_case("UPPERCASE"), "uppercase");
        assert_eq!(to_snake_case("lowercase"), "lowercase");
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
        assert_eq!(to_snake_case("TestXML"), "test_xml");
    }

    #[test]
    fn test_get_name_parts() {
        assert_eq!(
            get_name_parts("CamelCaseFunc"),
            vec!["camel".to_string(), "case".to_string(), "func".to_string()]
        );
        assert_eq!(
            get_name_parts("parseHTTPRes"),
            vec!["parse".to_string(), "http".to_string(), "res".to_string()]
        );
    }

    #[test]
    fn test_names_similar() {
        assert!(names_similar("loadConfigFile", "load_config"));
        assert!(names_similar("get_user_id", "fetch_user_id"));
        assert!(names_similar("parseHTTPRes", "parse_http_res"));
        assert!(!names_similar("fooBar", "baz"));
        assert!(!names_similar("foo1", "foo2"));
    }

    #[test]
    fn test_specializable_self_type_heuristic() {
        // no self type
        let mut f1 = get_visited_fn("foo", 1, None);
        let mut f2 = get_visited_fn("foo", 1, None);
        assert!(specializable_fn_heuristic(&f1, &f2));

        // same self type
        f1.self_type = Some("MyType".to_string());
        f2.self_type = Some("MyType".to_string());
        assert!(specializable_fn_heuristic(&f1, &f2));

        // similar self types
        let mut f3 = get_visited_fn("foo", 1, None);
        f3.self_type = Some("MyTypeStr".to_string());
        assert!(specializable_fn_heuristic(&f2, &f3));

        // different self types
        let mut f4 = get_visited_fn("foo", 1, None);
        f4.self_type = Some("Other".to_string());
        assert!(!specializable_fn_heuristic(&f2, &f4));

        // one has self type, other doesn't
        let f5 = get_visited_fn("foo", 1, None);
        assert!(!specializable_fn_heuristic(&f2, &f5));
    }

    #[test]
    fn test_specializable_trait_heuristic() {
        // same trait
        let t1 = get_visited_trait("MyTrait", 3);
        assert!(specializable_trait_heuristic(&t1, &t1));

        // similar names and same items
        let t3 = get_visited_trait("MyOtherTrait", 3);
        assert!(specializable_trait_heuristic(&t1, &t3));

        // names not similar
        let t4 = get_visited_trait("Other", 3);
        assert!(!specializable_trait_heuristic(&t1, &t4));

        // items different
        let t5 = get_visited_trait("MyOtherTrait", 4);
        assert!(!specializable_trait_heuristic(&t1, &t5));
    }

    #[test]
    fn test_specializable_fn_heuristic() {
        // bare fns, similar names
        let f1 = get_visited_fn("foo_i32", 2, None);
        let f2 = get_visited_fn("foo_str", 2, None);
        assert!(specializable_fn_heuristic(&f1, &f2));

        // bare fns, different names
        let f3 = get_visited_fn("other_fn", 2, None);
        assert!(!specializable_fn_heuristic(&f1, &f3));

        // bare fns, different args
        let f4 = get_visited_fn("foo_str", 1, None);
        assert!(!specializable_fn_heuristic(&f1, &f4));

        // one bare fn, one trait fn
        let t1 = get_visited_trait("TraitA", 1);
        let f5 = get_visited_fn("foo_str", 2, Some(t1));
        assert!(!specializable_fn_heuristic(&f1, &f5));

        // same trait, similar names
        let t1 = get_visited_trait("TraitA", 1);
        let f6 = get_visited_fn("foo_i32", 2, Some(t1));
        assert!(specializable_fn_heuristic(&f5, &f6));

        // same trait, different names
        let t1 = get_visited_trait("TraitA", 1);
        let f6 = get_visited_fn("bar_i32", 2, Some(t1));
        assert!(!specializable_fn_heuristic(&f5, &f6));

        // different trait, same name
        let t2 = get_visited_trait("TraitB", 1);
        let f7 = get_visited_fn("foo_i32", 2, Some(t2));
        assert!(specializable_fn_heuristic(&f6, &f7));

        // different trait, similar name
        assert!(specializable_fn_heuristic(&f5, &f7));

        // different trait, different name
        let t2 = get_visited_trait("TraitB", 1);
        let f7 = get_visited_fn("bar_i32", 2, Some(t2));
        assert!(!specializable_fn_heuristic(&f5, &f7));

        // traits not similar
        let t3 = get_visited_trait("Other", 3);
        let f8 = get_visited_fn("do_thing", 0, Some(t3));
        assert!(!specializable_fn_heuristic(&f5, &f8));
    }
}
