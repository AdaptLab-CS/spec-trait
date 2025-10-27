use super::tree::BodyTree;
use super::Analyzer;
use rustc_hir::intravisit::{walk_item, FnKind, Visitor, VisitorExt};
use rustc_hir::{BodyId, HirId, Impl, Item, ItemKind, OwnerId};
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VisitedFn {
    pub name: String,
    pub args: usize,
    pub body: BodyId,
}

pub struct STIVisitor<'tcx, 'a> {
    // The analyzer contains the `TyCtxt`
    analyzer: &'a Analyzer<'tcx>,

    pub visited_fns: HashSet<VisitedFn>,
}

// Guardare le tre diverse tipologie di linear: copy move e borrow
impl<'tcx, 'a> STIVisitor<'tcx, 'a> {
    pub fn new(analyzer: &'a Analyzer<'tcx>) -> Self {
        Self {
            analyzer,
            visited_fns: HashSet::new(),
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

// NOTE(bruzzone): `visit_ty_unambig` and `visit_const_arg_unambig` are defined in VisitorExt, so we need to import it.
impl<'tcx> Visitor<'tcx> for STIVisitor<'tcx, '_> {
    fn visit_nested_body(&mut self, id: BodyId) -> Self::Result {
        if self.analyzer.cli_args.print_nested_bodies {
            log::debug!("Printing body tree for body id: {:?}", id);
            let mut body_tree = BodyTree::new(self.analyzer);
            body_tree.build_from_body_id(&id);
            println!("{:#?}", body_tree.root);
        }
    }

    fn visit_item(&mut self, item: &'tcx Item) {
        match item.kind {
            ItemKind::Fn {
                ident, sig, body, ..
            } => {
                self.visited_fns.insert(VisitedFn {
                    name: ident.to_string(),
                    args: sig.decl.inputs.len(),
                    body,
                });
            }
            // ItemKind::Impl(Impl {
            //     generics,
            //     of_trait,
            //     self_ty,
            //     items,
            // }) => {
            //     self.visit_generics(generics);
            //     if let Some(trait_header) = of_trait {
            //         self.visit_trait_ref(&trait_header.trait_ref);
            //     }
            //     self.visit_ty_unambig(self_ty);
            //     for item in items {
            //         self.visit_impl_item_ref(item);
            //     }
            // }
            // ItemKind::Trait(
            //     _constness,
            //     _is_auto,
            //     _safety,
            //     ident,
            //     generics,
            //     bounds,
            //     trait_item_refs,
            // ) => {
            //     self.visit_ident(ident);
            //     self.visit_generics(generics);
            //     for bound in bounds {
            //         self.visit_param_bound(bound);
            //     }
            //     for trait_item_ref in trait_item_refs {
            //         self.visited_traits.insert(trait_item_ref.owner_id);
            //         // let owner_id = self.visited_traits.iter().last().unwrap();
            //         // let trait_item = self.analyzer.tcx.hir_owner_node(*owner_id).expect_trait_item();

            //         self.visit_trait_item_ref(trait_item_ref);
            //     }
            // }
            _ => {}
        }

        walk_item(self, item);
    }
}
