use super::tree::BodyTree;
use super::Analyzer;
use rustc_hir::intravisit::nested_filter::NestedFilter;
use rustc_hir::intravisit::{walk_impl_item, walk_item, walk_trait_item, Visitor};
use rustc_hir::{
    BodyId, FnSig, HirId, ImplItem, ImplItemKind, Item, ItemKind, TraitFn, TraitItem, TraitItemKind,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::Ident;
use std::fmt::Debug;

#[derive(Debug)]
pub struct VisitedFn {
    pub name: String,
    pub args: usize,
    pub body: BodyId,
    pub parent_trait: bool,
}

pub struct STIVisitor<'tcx, 'a> {
    // The analyzer contains the `TyCtxt`
    analyzer: &'a Analyzer<'tcx>,

    pub visited_fns: Vec<VisitedFn>,
}

// Guardare le tre diverse tipologie di linear: copy move e borrow
impl<'tcx, 'a> STIVisitor<'tcx, 'a> {
    pub fn new(analyzer: &'a Analyzer<'tcx>) -> Self {
        Self {
            analyzer,
            visited_fns: vec![],
        }
    }

    fn already_seen(&self, body: BodyId) -> bool {
        self.visited_fns.iter().any(|f| f.body == body)
    }

    fn add_fn(&mut self, ident: Ident, sig: FnSig, body: BodyId, parent_trait: bool) {
        if !self.already_seen(body) {
            self.visited_fns.push(VisitedFn {
                name: ident.to_string(),
                args: sig.decl.inputs.len(),
                body,
                parent_trait,
            });
        }
    }

    /// The entry point of the visitor.
    pub fn visit_with_hir_id_and_item(&mut self, hir_id: HirId, item: &'tcx Item) {
        log::trace!(
            "Visiting the hir_id {:?}, with owner {:?}",
            hir_id,
            hir_id.owner
        );
        self.visit_item(item);
    }
}

pub struct NestedFilterAll;
impl<'tcx> NestedFilter<'tcx> for NestedFilterAll {
    type MaybeTyCtxt = TyCtxt<'tcx>;
    const INTER: bool = true;
    const INTRA: bool = true;
}

// NOTE(bruzzone): `visit_ty_unambig` and `visit_const_arg_unambig` are defined in VisitorExt, so we need to import it.
impl<'tcx> Visitor<'tcx> for STIVisitor<'tcx, '_> {
    type NestedFilter = NestedFilterAll;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.analyzer.tcx
    }

    fn visit_nested_body(&mut self, id: BodyId) -> Self::Result {
        if self.analyzer.cli_args.print_nested_bodies {
            log::debug!("Printing body tree for body id: {:?}", id);
            let body_tree = BodyTree::new(self.analyzer, &id);
            println!("{:#?}", body_tree.root);
        }
    }

    fn visit_item(&mut self, item: &'tcx Item) {
        if let ItemKind::Fn {
            ident, sig, body, ..
        } = item.kind
        {
            self.add_fn(ident, sig, body, false);
        }

        walk_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'tcx ImplItem) {
        if let ImplItemKind::Fn(sig, body) = item.kind {
            self.add_fn(item.ident, sig, body, true);
        }

        walk_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'tcx TraitItem) {
        if let TraitItemKind::Fn(sig, TraitFn::Provided(body)) = item.kind {
            self.add_fn(item.ident, sig, body, true);
        }

        walk_trait_item(self, item);
    }
}
