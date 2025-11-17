mod compare;
mod distance;
mod sti_visitor;
mod tree;

use super::Analyzer;
use compare::{STIComparison, SpecializablePair, SpecializableTraitPair};
use itertools::Itertools;
use rustc_hir::def_id::LOCAL_CRATE;
use std::{cell::Cell, time::Duration};
use sti_visitor::{STIVisitor, VisitedFn, VisitedTrait};

pub struct STIAnalysis<'tcx, 'a> {
    analyzer: &'a Analyzer<'tcx>,
    krate_name: String,
    elapsed: Cell<Option<Duration>>,
}

impl<'tcx, 'a> STIAnalysis<'tcx, 'a> {
    pub fn new(analyzer: &'a Analyzer<'tcx>) -> Self {
        let krate_name = analyzer.tcx.crate_name(LOCAL_CRATE).to_string();
        log::info!("Initialized STIAnalysis for crate {}", krate_name);
        Self {
            analyzer,
            krate_name,
            elapsed: Cell::new(None),
        }
    }

    fn visit_item(&self, visitor: &mut STIVisitor<'tcx, 'a>, item_id: &rustc_hir::ItemId) {
        let hir_id = self
            .analyzer
            .tcx
            .local_def_id_to_hir_id(item_id.owner_id.def_id);
        let item = self.analyzer.tcx.hir_item(*item_id);
        visitor.visit_with_hir_id_and_item(hir_id, item);

        if let rustc_hir::ItemKind::Mod(_, module) = &item.kind {
            for sub in module.item_ids {
                self.visit_item(visitor, sub);
            }
        }
    }

    fn analyze_group(
        &self,
        comparison: &STIComparison<'tcx, 'a>,
        specializable_fn_pairs: &Vec<&SpecializablePair<'a>>,
    ) -> (
        Vec<SpecializableTraitPair<'a>>,
        Vec<&'a VisitedFn>,
        Vec<&'a VisitedTrait>,
    ) {
        let specializable_trait_pairs = comparison.get_specializable_traits(specializable_fn_pairs);

        let specializable_fns = specializable_fn_pairs
            .iter()
            .flat_map(|pair| vec![pair.fn_a, pair.fn_b])
            .unique_by(|f| f.id)
            .collect::<Vec<_>>();

        let specializable_traits = specializable_trait_pairs
            .iter()
            .flat_map(|pair| vec![pair.trait_a, pair.trait_b])
            .unique_by(|t| t.id)
            .collect::<Vec<_>>();

        (
            specializable_trait_pairs,
            specializable_fns,
            specializable_traits,
        )
    }

    fn get_groups(
        &self,
        comparison: &STIComparison<'tcx, 'a>,
        specializable_fn_pairs: &Vec<&SpecializablePair<'a>>,
    ) -> Vec<usize> {
        // specializable
        let (specializable_trait_pairs, specializable_fns, specializable_traits) =
            self.analyze_group(comparison, specializable_fn_pairs);

        // already specializable
        let already_specializable_fn_pairs = specializable_fn_pairs
            .iter()
            .filter(|pair| compare::already_specializable_heuristic(pair))
            .copied()
            .collect::<Vec<_>>();
        let (
            already_specializable_trait_pairs,
            already_specializable_fns,
            already_specializable_traits,
        ) = self.analyze_group(comparison, &already_specializable_fn_pairs);

        // already specialized
        let already_specialized_fn_pairs = already_specializable_fn_pairs
            .iter()
            .filter(|pair| compare::already_specialized_heuristic(pair))
            .copied()
            .collect::<Vec<_>>();
        let (already_specialized_trait_pairs, already_specialized_fns, already_specialized_traits) =
            self.analyze_group(comparison, &already_specialized_fn_pairs);

        // newly specializable
        let newly_specializable_fn_pairs =
            specializable_fn_pairs.len() - already_specializable_fn_pairs.len();
        let newly_specializable_trait_pairs =
            specializable_trait_pairs.len() - already_specializable_trait_pairs.len();
        let newly_specializable_fns = specializable_fns.len() - already_specializable_fns.len();
        let newly_specializable_traits =
            specializable_traits.len() - already_specializable_traits.len();

        // already specializable but not specialized
        let already_specializable_unspecialized_fn_pairs =
            already_specializable_fn_pairs.len() - already_specialized_fn_pairs.len();
        let already_specializable_unspecialized_trait_pairs =
            already_specializable_trait_pairs.len() - already_specialized_trait_pairs.len();
        let already_specializable_unspecialized_fns =
            already_specializable_fns.len() - already_specialized_fns.len();
        let already_specializable_unspecialized_traits =
            already_specializable_traits.len() - already_specialized_traits.len();

        vec![
            specializable_fn_pairs.len(),
            specializable_trait_pairs.len(),
            specializable_fns.len(),
            specializable_traits.len(),
            already_specializable_fn_pairs.len(),
            already_specializable_trait_pairs.len(),
            already_specializable_fns.len(),
            already_specializable_traits.len(),
            already_specialized_fn_pairs.len(),
            already_specialized_trait_pairs.len(),
            already_specialized_fns.len(),
            already_specialized_traits.len(),
            newly_specializable_fn_pairs,
            newly_specializable_trait_pairs,
            newly_specializable_fns,
            newly_specializable_traits,
            already_specializable_unspecialized_fn_pairs,
            already_specializable_unspecialized_trait_pairs,
            already_specializable_unspecialized_fns,
            already_specializable_unspecialized_traits,
        ]
    }

    fn analyze(&self, visitor: &STIVisitor<'tcx, 'a>) {
        let comparison = STIComparison::new(self.analyzer, visitor);
        let specializable_pairs = comparison.get_specializable_fns();

        if self.analyzer.cli_args.print_spec_targets {
            let specializable_trait_pairs =
                comparison.get_specializable_traits(&specializable_pairs.iter().collect());

            println!("Specializable fn pairs: {}", specializable_pairs.len());
            for pair in &specializable_pairs {
                println!("{}", pair);
            }

            println!(
                "Specializable trait pairs: {}",
                specializable_trait_pairs.len()
            );
            for pair in &specializable_trait_pairs {
                println!("{}", pair);
            }
        }

        if self.analyzer.cli_args.print_spec_data {
            // groups
            let is_bare_fn = |f: &VisitedFn| f.parent_trait.is_none() && f.self_type.is_none();
            let specializable_pairs_from_bare_fns = specializable_pairs
                .iter()
                .filter(|p| is_bare_fn(p.fn_a) && is_bare_fn(p.fn_b))
                .collect::<Vec<_>>();

            let is_trait_fn = |f: &VisitedFn| f.parent_trait.is_some() && f.self_type.is_none();
            let specializable_pairs_from_traits = specializable_pairs
                .iter()
                .filter(|p| is_trait_fn(p.fn_a) && is_trait_fn(p.fn_b))
                .collect::<Vec<_>>();

            let is_trait_impl_fn =
                |f: &VisitedFn| f.parent_trait.is_some() && f.self_type.is_some();
            let specializable_pairs_from_trait_impls = specializable_pairs
                .iter()
                .filter(|p| is_trait_impl_fn(p.fn_a) && is_trait_impl_fn(p.fn_b))
                .collect::<Vec<_>>();

            let is_inherent_impl_fn =
                |f: &VisitedFn| f.parent_trait.is_none() && f.self_type.is_some();
            let specializable_pairs_from_inherent_impls = specializable_pairs
                .iter()
                .filter(|p| is_inherent_impl_fn(p.fn_a) && is_inherent_impl_fn(p.fn_b))
                .collect::<Vec<_>>();

            let is_macro_fn = |f: &VisitedFn| {
                f.name.contains("$")
                    || f.generics.iter().any(|g| g.contains("$"))
                    || f.self_type.as_ref().is_some_and(|ty| ty.contains("$"))
                    || f.parent_trait
                        .as_ref()
                        .is_some_and(|t| t.name.contains("$"))
            };
            let specializable_pairs_from_macros = specializable_pairs
                .iter()
                .filter(|p| is_macro_fn(p.fn_a) && is_macro_fn(p.fn_b))
                .collect::<Vec<_>>();

            let total_fns = visitor.visited_fns.len();
            let total_traits = visitor
                .visited_fns
                .iter()
                .filter_map(|f| f.parent_trait.as_ref().map(|t| t.id))
                .unique()
                .count();
            let total_bare_fns = visitor.visited_fns.iter().filter(|f| is_bare_fn(f)).count();
            let total_trait_fns = visitor
                .visited_fns
                .iter()
                .filter(|f| is_trait_fn(f))
                .count();
            let total_trait_impl_fns = visitor
                .visited_fns
                .iter()
                .filter(|f| is_trait_impl_fn(f))
                .count();
            let total_inherent_impl_fns = visitor
                .visited_fns
                .iter()
                .filter(|f| is_inherent_impl_fn(f))
                .count();
            let total_macro_fns = visitor
                .visited_fns
                .iter()
                .filter(|f| is_macro_fn(f))
                .count();

            // groups values
            let base_group = self.get_groups(&comparison, &specializable_pairs.iter().collect());
            let bare_fn_group = self.get_groups(&comparison, &specializable_pairs_from_bare_fns);
            let trait_group = self.get_groups(&comparison, &specializable_pairs_from_traits);
            let trait_impl_group =
                self.get_groups(&comparison, &specializable_pairs_from_trait_impls);
            let inherent_impl_group =
                self.get_groups(&comparison, &specializable_pairs_from_inherent_impls);
            let macro_group = self.get_groups(&comparison, &specializable_pairs_from_macros);
            let totals = vec![
                total_fns,
                total_traits,
                total_bare_fns,
                total_trait_fns,
                total_trait_impl_fns,
                total_inherent_impl_fns,
                total_macro_fns,
            ];

            let all_data = [
                totals,
                base_group,
                bare_fn_group,
                trait_group,
                trait_impl_group,
                inherent_impl_group,
                macro_group,
            ]
            .concat();

            println!(
                "Specialization analysis data: {}",
                all_data.iter().join(",")
            );
        }
    }

    fn visitor(&self) {
        log::info!("Starting the STI visitor for crate {}", self.krate_name);

        let visitor: &mut STIVisitor<'tcx, 'a> = &mut STIVisitor::new(self.analyzer);

        // Visit all items in the crate
        let root_item_ids = self.analyzer.tcx.hir_root_module().item_ids;
        for item_id in root_item_ids {
            self.visit_item(visitor, item_id);
        }

        log::info!("Found {} functions", visitor.visited_fns.len());

        // Analyze visited functions for specialization targets
        self.analyze(visitor);
    }

    fn peak_rss_kb(&self) -> Option<u64> {
        unsafe {
            let mut usage: libc::rusage = std::mem::zeroed();
            if libc::getrusage(libc::RUSAGE_SELF, &mut usage) == 0 {
                Some(usage.ru_maxrss as u64)
            } else {
                None
            }
        }
    }

    pub fn run(&self) {
        let start_time = std::time::Instant::now();
        self.visitor();
        let elapsed = start_time.elapsed();
        self.elapsed.set(Some(elapsed));
        log::info!(
            "STIAnalysis for crate {} completed in {:.3?} s with peak rss {} MB",
            self.krate_name,
            elapsed.as_secs_f64(),
            self.peak_rss_kb().unwrap_or(0) / 1024
        );
    }
}
