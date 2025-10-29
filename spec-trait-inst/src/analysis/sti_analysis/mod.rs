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
        let comparison = STIComparison::new(self.analyzer, visitor);
        let specializable = comparison.get_specializable_fns();
        let specializable_fns = specializable
            .iter()
            .filter(|pair| pair.fn_a.parent_trait.is_none() && pair.fn_b.parent_trait.is_none())
            .collect::<Vec<_>>();
        let specializable_traits = specializable
            .iter()
            .filter(|pair| pair.fn_a.parent_trait.is_some() && pair.fn_b.parent_trait.is_some())
            .flat_map(|pair| {
                [
                    pair.fn_a.parent_trait.as_ref().unwrap(),
                    pair.fn_b.parent_trait.as_ref().unwrap(),
                ]
            })
            .unique_by(|t| t.id)
            .collect::<Vec<_>>();
        log::info!(
            "Found {} specializable functions pairs:\n{:#?}",
            specializable_fns.len(),
            specializable_fns
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        );
        log::info!(
            "Found {} specializable traits:\n{:#?}",
            specializable_traits.len(),
            specializable_traits
                .iter()
                .map(|t| &t.name)
                .collect::<Vec<_>>()
        );
    }

    pub fn run(&self) {
        let start_time = std::time::Instant::now();
        self.visitor();
        let elapsed = start_time.elapsed();
        self.elapsed.set(Some(elapsed));
        log::info!(
            "STIAnalysis for crate {} completed in {:.2?}",
            self.krate_name,
            elapsed
        );
    }
}
