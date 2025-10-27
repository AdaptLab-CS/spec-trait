mod distance;
mod sti_visitor;
mod tree;

use super::Analyzer;
use rustc_hir::def_id::LOCAL_CRATE;
use std::{cell::Cell, collections::HashMap, time::Duration};
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
                self.visit_item(visitor, &sub);
            }
        }
    }

    fn visitor(&self) {
        log::info!("Starting the STI visitor for crate {}", self.krate_name);

        let visitor: &mut STIVisitor<'tcx, 'a> = &mut STIVisitor::new(self.analyzer);

        // Visit all items in the crate
        let root_item_ids = self.analyzer.tcx.hir_root_module().item_ids;
        for item_id in root_item_ids {
            self.visit_item(visitor, &item_id);
        }

        log::info!(
            "Found {} functions:\n{:#?}",
            visitor.visited_fns.len(),
            visitor
                .visited_fns
                .iter()
                .map(|f| &f.name)
                .collect::<Vec<_>>()
        );

        // Analyze visited functions for specialization targets
        let mut groups: HashMap<usize, Vec<(&sti_visitor::VisitedFn, tree::BodyTree)>> =
            HashMap::new();
        for visited_fn in &visitor.visited_fns {
            let mut bt = tree::BodyTree::new(self.analyzer);
            bt.build_from_body_id(&visited_fn.body);

            if !bt.root.children.is_empty() {
                groups
                    .entry(visited_fn.args)
                    .or_default()
                    .push((visited_fn, bt));
            }
        }

        let mut specializable = vec![];
        for (args, group) in groups {
            log::debug!("processing functions with {} args", args);
            for (i, (fn_a, tree_a)) in group.iter().enumerate() {
                log::debug!("processing function {}/{}", i + 1, group.len());
                for (fn_b, tree_b) in group.iter().skip(i + 1) {
                    let similarity = distance::compute_similarity(&tree_a.root, &tree_b.root);
                    if similarity > 0.95 {
                        specializable.push((*fn_a, *fn_b, similarity));
                    }
                }
            }
        }

        log::info!(
            "Found {} specializable functions:\n{:#?}",
            specializable.len(),
            specializable
                .iter()
                .map(|(a, b, sim)| format!("{} <-> {} (sim: {:.2})", a.name, b.name, sim))
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
