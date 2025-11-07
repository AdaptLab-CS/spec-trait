mod compare;
mod distance;
mod sti_visitor;
mod tree;

use super::Analyzer;
use compare::STIComparison;
use itertools::Itertools;
use rustc_hir::def_id::LOCAL_CRATE;
use std::{cell::Cell, time::Duration};
use sti_visitor::STIVisitor;

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

    fn analyze(&self, visitor: &STIVisitor<'tcx, 'a>) {
        let comparison = STIComparison::new(self.analyzer, visitor);

        // fns, traits
        let total_fns = visitor.visited_fns.len();
        let total_traits = visitor
            .visited_fns
            .iter()
            .filter_map(|f| f.parent_trait.as_ref().map(|t| t.id))
            .count();

        // specializable fn pairs, specializable trait pairs
        let specializable_fn_pairs = comparison.get_specializable_fns();
        let specializable_trait_pairs =
            comparison.get_specializable_traits(&specializable_fn_pairs.iter().collect());

        // specializable fns, specializable traits
        let specializable_fns = specializable_fn_pairs
            .iter()
            .flat_map(|pair| vec![pair.fn_a, pair.fn_b])
            .unique_by(|f| f.id)
            .collect::<Vec<_>>();
        let specializable_traits = specializable_fns
            .iter()
            .unique_by(|f| f.id)
            .collect::<Vec<_>>();

        // already specializable fn pairs, already specializable trait pairs
        let already_specializable_fn_pairs = specializable_fn_pairs
            .iter()
            .filter(|pair| compare::already_specializable_heuristic(pair))
            .collect::<Vec<_>>();
        let already_specializable_trait_pairs =
            comparison.get_specializable_traits(&already_specializable_fn_pairs);

        // already specializable fns, already specializable traits
        let already_specializable_fns = already_specializable_fn_pairs
            .iter()
            .flat_map(|pair| vec![pair.fn_a, pair.fn_b])
            .unique_by(|f| f.id)
            .count();
        let already_specializable_traits = already_specializable_trait_pairs
            .iter()
            .flat_map(|pair| vec![pair.trait_a, pair.trait_b])
            .unique_by(|t| t.id)
            .count();

        // newly specializable fn pairs, newly specializable trait pairs
        let newly_specializable_fn_pairs =
            specializable_fn_pairs.len() - already_specializable_fn_pairs.len();
        let newly_specializable_trait_pairs =
            specializable_trait_pairs.len() - already_specializable_trait_pairs.len();

        // newly specializable fns, newly specializable traits
        let newly_specializable_fns = specializable_fns.len() - already_specializable_fns;
        let newly_specializable_traits = specializable_traits.len() - already_specializable_traits;

        // specializable fns from macros, specializable traits from macros
        let specializable_fns_from_macros = specializable_fns
            .iter()
            .filter(|f| f.self_type.as_ref().is_some_and(|t| t.starts_with("$")))
            .count();
        let specializable_traits_from_macros = specializable_traits
            .iter()
            .filter(|t| t.name.starts_with("$"))
            .count();

        // specializable fns from traits, specializable fns without traits
        let specializable_fns_from_traits = specializable_fns
            .iter()
            .filter(|f| f.parent_trait.is_some())
            .count();
        let specializable_fns_without_traits =
            specializable_fns.len() - specializable_fns_from_traits;

        log::info!(
            "specializable {} already specializable {} newly specializable {}",
            specializable_fn_pairs.len(),
            already_specializable_fn_pairs.len(),
            newly_specializable_fn_pairs
        );

        log::info!(
            "Specializable pairs: {} fns and {} traits",
            specializable_fn_pairs.len(),
            specializable_trait_pairs.len()
        );

        if self.analyzer.cli_args.print_spec_targets {
            println!("Specializable fn pairs: {}", specializable_fn_pairs.len());
            for pair in &specializable_fn_pairs {
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
            "STIAnalysis for crate {} completed in {:.2?} with peak rss {} KB",
            self.krate_name,
            elapsed,
            self.peak_rss_kb().unwrap_or(0)
        );
    }
}
