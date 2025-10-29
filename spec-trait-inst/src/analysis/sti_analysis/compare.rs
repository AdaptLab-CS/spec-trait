use super::distance::compute_similarity;
use super::sti_visitor::{STIVisitor, VisitedFn};
use super::tree::BodyTree;
use super::Analyzer;
use std::collections::HashSet;
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

impl Display for SpecializablePair<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:^30.30} <---{:^6.2}---> {:^30.30}",
            self.fn_a.name, self.similarity, self.fn_b.name,
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
            log::debug!("processing function {}/{}", i + 1, fns.len());
            for (fn_b, tree_b) in fns.iter().skip(i + 1) {
                if self.specializable_heuristic(fn_a, fn_b) {
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

    /// A heuristic to quickly determine if two functions could be specializable.
    fn specializable_heuristic(&self, fn_a: &VisitedFn, fn_b: &VisitedFn) -> bool {
        fn_a.args == fn_b.args
            && fn_a.parent_trait == fn_b.parent_trait
            && self.names_similar(&fn_a.name, &fn_b.name)
    }

    /// Determine if two body trees are specializable based on their similarity.
    /// If they are, return the similarity score.
    fn is_specializable(&self, tree_a: &BodyTree, tree_b: &BodyTree) -> Option<f64> {
        let similarity = compute_similarity(&tree_a.root, &tree_b.root);
        if similarity > 0.9 {
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

    /// Check if two function names are similar based on shared underscore-separated parts.
    fn names_similar(&self, a: &str, b: &str) -> bool {
        let parts_a = a.split('_').filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let parts_b = b.split('_').filter(|s| !s.is_empty()).collect::<Vec<_>>();

        if parts_a.is_empty() || parts_b.is_empty() {
            return false;
        }

        let set_a = parts_a.iter().copied().collect::<HashSet<_>>();
        let set_b = parts_b.iter().copied().collect::<HashSet<_>>();

        let common = set_a.intersection(&set_b).count();
        let len = parts_a.len().max(parts_b.len());

        (common as f64) / (len as f64) >= 0.5
    }
}
