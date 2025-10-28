use super::distance::compute_similarity;
use super::sti_visitor::{STIVisitor, VisitedFn};
use super::tree::BodyTree;
use super::Analyzer;
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

    /// Get all pairs of specializable functions along with their similarity score.
    pub fn get_specializable_fns(&self) -> Vec<SpecializablePair<'a>> {
        let mut specializable = vec![];
        let groups = self.group_fns();
        for (i, group) in groups.iter().enumerate() {
            for (j, (idx_a, tree_a)) in group.iter().enumerate() {
                log::debug!(
                    "processing function {}/{} in group {}/{}",
                    j + 1,
                    group.len(),
                    i + 1,
                    groups.len(),
                );
                for (idx_b, tree_b) in group.iter().skip(j + 1) {
                    if let Some(similarity) = self.is_specializable(&tree_a, &tree_b) {
                        let fn_a = &self.visitor.visited_fns[*idx_a];
                        let fn_b = &self.visitor.visited_fns[*idx_b];
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

    /// Group functions by the number of arguments and name similarity.
    fn group_fns(&self) -> Vec<Vec<(usize, BodyTree<'tcx, 'a>)>> {
        let mut buckets: HashMap<usize, Vec<Vec<(usize, BodyTree<'tcx, 'a>)>>> = HashMap::new();

        for (idx, visited_fn) in self.visitor.visited_fns.iter().enumerate() {
            let bt = BodyTree::new(self.analyzer, &visited_fn.body);
            if bt.root.children.is_empty() {
                continue;
            }
            let bucket = buckets.entry(visited_fn.args).or_default();

            let mut target_subgroup = None;
            for (sub_idx, subgroup) in bucket.iter().enumerate() {
                if subgroup.is_empty() {
                    continue;
                }
                let rep_idx = subgroup[0].0;
                let rep_name = &self.visitor.visited_fns[rep_idx].name;
                if self.names_similar(rep_name, &visited_fn.name) {
                    target_subgroup = Some(sub_idx);
                    break;
                }
            }

            if let Some(i) = target_subgroup {
                bucket[i].push((idx, bt));
            } else {
                bucket.push(vec![(idx, bt)]);
            }
        }

        buckets.into_iter().flat_map(|(_, v)| v).collect()
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
