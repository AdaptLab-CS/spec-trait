use super::distance::compute_similarity;
use super::sti_visitor::{STIVisitor, VisitedFn, VisitedTrait};
use super::tree::BodyTree;
use super::Analyzer;
use itertools::Itertools;
use spec_trait_utils::conversions::str_to_type_name;
use spec_trait_utils::types::{type_contains, type_contains_lifetime};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Result};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:^30.30} <---{:^6.2}---> {:^30.30}",
            self.fn_a, self.similarity, self.fn_b,
        )
    }
}

impl Display for SpecializableTraitPair<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:^30.30} <---{:^6.2}---> {:^30.30}",
            self.trait_a, self.similarity, self.trait_b,
        )
    }
}

#[derive(Debug)]
struct TraitInfo<'a> {
    tr: &'a VisitedTrait,
    fn_items: usize,
    specializable_fn_items: usize,
}

impl<'a> TraitInfo<'a> {
    fn new(tr: &'a VisitedTrait) -> Self {
        Self {
            tr,
            fn_items: 0,
            specializable_fn_items: 0,
        }
    }
}

const THRESHOLD: f64 = 0.9;

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
        specializable_fn_pairs: &Vec<&SpecializablePair<'a>>,
    ) -> Vec<SpecializableTraitPair<'a>> {
        let mut trait_info = HashMap::new();

        for f in &self.visitor.visited_fns {
            if let Some(tr) = f.parent_trait.as_ref() {
                let entry = trait_info.entry(tr).or_insert(TraitInfo::new(tr));
                entry.fn_items += 1;
            }
        }

        let specializable_fns = specializable_fn_pairs
            .iter()
            .flat_map(|pair| vec![pair.fn_a, pair.fn_b])
            .unique_by(|f| f.id)
            .collect::<Vec<_>>();

        for f in specializable_fns {
            if let Some(tr) = f.parent_trait.as_ref() {
                let entry = trait_info.entry(tr).or_insert(TraitInfo::new(tr));
                entry.specializable_fn_items += 1;
            }
        }

        let key = |trait_a: &TraitInfo<'a>, trait_b: &TraitInfo<'a>| {
            if trait_a.tr.id.index <= trait_b.tr.id.index {
                (trait_a.tr, trait_b.tr)
            } else {
                (trait_b.tr, trait_a.tr)
            }
        };

        let mut traits: HashMap<_, f64> = HashMap::new();
        for pair in specializable_fn_pairs {
            if let (Some(trait_a), Some(trait_b)) = (
                pair.fn_a
                    .parent_trait
                    .as_ref()
                    .and_then(|tr| trait_info.get(&tr)),
                pair.fn_b
                    .parent_trait
                    .as_ref()
                    .and_then(|tr| trait_info.get(&tr)),
            ) {
                if specializable_trait_pair_heuristic(trait_a, trait_b) {
                    if let Some(similarity) = traits.get_mut(&key(trait_a, trait_b)) {
                        *similarity = (*similarity).min(pair.similarity);
                    } else {
                        traits.insert(key(trait_a, trait_b), pair.similarity);
                    }
                }
            }
        }

        traits
            .iter()
            .map(|((tr_a, tr_b), sim)| SpecializableTraitPair {
                trait_a: trait_info.get(tr_a).unwrap().tr,
                trait_b: trait_info.get(tr_b).unwrap().tr,
                similarity: *sim,
            })
            .collect()
    }

    /// Determine if two body trees are specializable based on their similarity.
    /// If they are, return the similarity score.
    fn is_specializable(&self, tree_a: &BodyTree, tree_b: &BodyTree) -> Option<f64> {
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
    fn_a.args.len() == fn_b.args.len()
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

/// A heuristic to quickly determine if two traits could be specializable.
fn specializable_trait_pair_heuristic<'a>(tr_a: &TraitInfo<'a>, tr_b: &TraitInfo<'a>) -> bool {
    let max_diff = |a: usize, b: usize| a.abs_diff(b) <= 1;
    specializable_trait_heuristic(tr_a.tr, tr_b.tr)
        && tr_a.specializable_fn_items > 0
        && tr_b.specializable_fn_items > 0
        && max_diff(tr_a.fn_items, tr_b.fn_items)
        && max_diff(tr_a.specializable_fn_items, tr_b.specializable_fn_items)
        && max_diff(tr_a.fn_items, tr_a.specializable_fn_items)
        && max_diff(tr_b.fn_items, tr_b.specializable_fn_items)
}

/// A heuristic to quickly determine if two functions could be already specializable.
pub fn already_specializable_heuristic(pair: &SpecializablePair) -> bool {
    specializable_fn_heuristic(pair.fn_a, pair.fn_b)
        && pair.fn_a.self_type == pair.fn_b.self_type
        && pair.fn_a.args.iter().any(|arg_a| {
            pair.fn_b.args.iter().any(|arg_b| {
                arg_a != arg_b
                    && !arg_a.contains("$")
                    && !arg_b.contains("$")
                    && !contains_generics(arg_a, &pair.fn_a.generics)
                    && !contains_generics(arg_b, &pair.fn_b.generics)
            })
        })
}

/// A heuristic to quickly determine if two functions are already specialized.
pub fn already_specialized_heuristic(pair: &SpecializablePair) -> bool {
    already_specializable_heuristic(pair)
        && pair.fn_a.id == pair.fn_b.id
        && pair.fn_a.parent_trait.as_ref().is_some_and(|ta| {
            pair.fn_b
                .parent_trait
                .as_ref()
                .is_some_and(|tb| ta.id == tb.id)
        })
}

fn contains_generics(ty: &str, generics: &[String]) -> bool {
    generics.iter().filter(|gen| !gen.contains("$")).any(|gen| {
        if gen.starts_with("'") {
            type_contains_lifetime(&str_to_type_name(ty), gen)
        } else {
            type_contains(&str_to_type_name(ty), gen)
        }
    })
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
    use rustc_hir::def_id::{DefId, DefIndex, LocalDefId};
    use rustc_hir::{BodyId, HirId, ItemLocalId, OwnerId};
    use rustc_index::Idx;

    fn get_visited_trait(name: &str, items: usize) -> VisitedTrait {
        VisitedTrait {
            name: name.to_string(),
            id: DefId::local(DefIndex::from_usize(0)),
            items,
            self_type: None,
        }
    }

    fn get_visited_fn(name: &str, args: usize, parent_trait: Option<VisitedTrait>) -> VisitedFn {
        VisitedFn {
            name: name.to_string(),
            id: DefId::local(DefIndex::from_usize(0)),
            args: (0..args).map(|i| format!("arg{}", i)).collect(),
            body: BodyId {
                hir_id: HirId {
                    owner: OwnerId {
                        def_id: LocalDefId::new(0),
                    },
                    local_id: ItemLocalId::ZERO,
                },
            },
            generics: vec!["'a".to_string(), "T".to_string()],
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
    fn test_specializable_trait_pair_heuristic_comprehensive() {
        fn get_trait_info<'a>(tr: &'a VisitedTrait, fns: usize, spec_fns: usize) -> TraitInfo<'a> {
            TraitInfo {
                tr,
                fn_items: fns,
                specializable_fn_items: spec_fns,
            }
        }

        let tr_a = get_visited_trait("MyTrait", 3);
        let tr_b = get_visited_trait("MyOtherTrait", 3);
        let tr_c = get_visited_trait("Other", 3);

        // similar trait names, same items, counts within allowed diffs
        let t1 = get_trait_info(&tr_a, 3, 2);
        let t2 = get_trait_info(&tr_b, 2, 1);
        assert!(specializable_trait_pair_heuristic(&t1, &t2));

        // fn_items differ by more than 1
        let t3 = get_trait_info(&tr_a, 1, 2);
        assert!(!specializable_trait_pair_heuristic(&t1, &t3));

        // specializable_fn_items differ by more than 1
        let t4 = get_trait_info(&tr_a, 4, 4);
        assert!(!specializable_trait_pair_heuristic(&t1, &t4));

        // one has zero specializable fn items
        let t5 = get_trait_info(&tr_b, 1, 0);
        assert!(!specializable_trait_pair_heuristic(&t2, &t5));

        // trait names not similar
        let t6 = get_trait_info(&tr_c, 3, 2);
        assert!(!specializable_trait_pair_heuristic(&t1, &t6));

        // difference between fn_items and specializable_fn_items > 1
        let t7 = get_trait_info(&tr_a, 3, 1);
        assert!(!specializable_trait_pair_heuristic(&t1, &t7));
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

    #[test]
    fn test_already_specializable_heuristic() {
        fn test_from_fns(f1: &VisitedFn, f2: &VisitedFn, expected: bool) {
            let pair = SpecializablePair {
                fn_a: f1,
                fn_b: f2,
                similarity: 0.95,
            };
            assert_eq!(already_specializable_heuristic(&pair), expected);
        }

        // same type, different args
        let f1 = get_visited_fn("foo", 2, None);
        let f2 = get_visited_fn("foo", 2, None);
        test_from_fns(&f1, &f2, true);

        // different type, different args
        let f3 = get_visited_fn("foo_bar", 2, None);
        test_from_fns(&f1, &f3, true);

        // same type, same args
        let f4 = get_visited_fn("foo", 1, None);
        test_from_fns(&f4, &f4, false);

        // with one generic
        let mut f5 = get_visited_fn("foo", 2, None);
        f5.args[0] = "T".to_string();
        test_from_fns(&f1, &f5, true);

        // with all generics
        let mut f6 = get_visited_fn("foo", 2, None);
        f6.args[0] = "T".to_string();
        f6.args[1] = "T".to_string();
        test_from_fns(&f1, &f6, false);
    }

    #[test]
    fn test_already_specialized_heuristic() {
        fn test_from_fns(f1: &VisitedFn, f2: &VisitedFn, expected: bool) {
            let pair = SpecializablePair {
                fn_a: f1,
                fn_b: f2,
                similarity: 0.95,
            };
            assert_eq!(already_specialized_heuristic(&pair), expected);
        }

        // same fn, same trait
        let f1 = get_visited_fn("foo", 2, Some(get_visited_trait("MyTrait", 2)));
        let f2 = get_visited_fn("foo", 2, Some(get_visited_trait("MyTrait", 2)));
        test_from_fns(&f1, &f2, true);

        // same fn, different trait
        let f1 = get_visited_fn("foo", 2, Some(get_visited_trait("MyTrait", 2)));
        let f2 = get_visited_fn("foo", 2, Some(get_visited_trait("MyOtherTrait", 2)));
        test_from_fns(&f1, &f2, true);

        // same fn, no trait
        let f4 = get_visited_fn("foo", 2, None);
        test_from_fns(&f1, &f4, false);

        // different fn
        let mut f5 = get_visited_fn("foo", 2, Some(get_visited_trait("MyTrait", 2)));
        f5.id = DefId::local(DefIndex::from_usize(1));
        test_from_fns(&f1, &f5, false);
    }
}
