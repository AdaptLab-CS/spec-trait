mod sti_visitor;

use super::{
    Analyzer,
};
use sti_visitor::STIVisitor;
use rustc_hir::def_id::LOCAL_CRATE;
use std::{cell::Cell, time::Duration};

pub struct STIAnalysis<'tcx, 'a> {
    analyzer: &'a Analyzer<'tcx>,
    krate_name: String,
    elapsed: Cell<Option<Duration>>,
}

impl<'tcx, 'a> STIAnalysis<'tcx, 'a> {
    pub fn new(analyzer: &'a Analyzer<'tcx>) -> Self {
        let krate_name = analyzer.tcx.crate_name(LOCAL_CRATE).to_string();
        Self {
            analyzer,
            krate_name,
            elapsed: Cell::new(None),
        }
    }

    fn visitor(&self) {
        log::info!("Starting the STI visitor for crate {}", self.krate_name);

        let visitor: &mut STIVisitor<'tcx, 'a> = &mut STIVisitor::new(self.analyzer);

        /* 
            Useless:
            let krate = self.analyzer.tcx.resolver_for_lowering().borrow().1.clone();

            let body_owners = self.analyzer.tcx.hir_body_owners();

            let krate = self.analyzer.tcx.hir_root_module();
            let local_def_ids = krate.item_ids.iter().map(|item_id| item_id.owner_id.def_id).collect::<Vec<_>>();
        */

        let item_ids= self.analyzer.tcx.hir_root_module().item_ids;

        for item_id in item_ids {
            let hir_id = self.analyzer.tcx.local_def_id_to_hir_id(item_id.owner_id.def_id);
            let item = self.analyzer.tcx.hir_item(*item_id);
            visitor.visit_with_hir_id_and_item(hir_id, item);
        }
    }

    pub fn run(&self) {
        let start_time = std::time::Instant::now();
        self.visitor();
        let elapsed = start_time.elapsed();
        self.elapsed.set(Some(elapsed));
    }
}
