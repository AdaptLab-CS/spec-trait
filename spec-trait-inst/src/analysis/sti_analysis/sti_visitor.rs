use super::tree::BodyTree;
use super::Analyzer;
use rustc_hir::intravisit::nested_filter::NestedFilter;
use rustc_hir::intravisit::{walk_impl_item, walk_item, walk_trait_item, Visitor};
use rustc_hir::{
    BodyId, FnSig, Generics, HirId, ImplItem, ImplItemKind, Item, ItemKind, TraitFn,
    TraitItem, TraitItemKind, Ty, ParamName
};
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::DefId;
use rustc_span::Ident;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct VisitedTrait {
    pub name: String,
    pub id: DefId,
    pub items: usize,
}

#[derive(Debug)]
pub struct VisitedFn {
    pub name: String,
    pub id: DefId,
    pub args: Vec<String>,
    pub body: BodyId,
    /// all the generics that might be used in the fn (both fn generics and impl / trait generics)
    pub generics: Vec<String>,
    pub parent_trait: Option<VisitedTrait>,
    pub self_type: Option<String>,
}

impl Display for VisitedFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(tr) = &self.parent_trait {
            if let Some(self_type) = &self.self_type {
                write!(f, "{}::{} for {}", tr.name, self.name, self_type)
            } else {
                write!(f, "{}::{}", tr.name, self.name)
            }
        } else if let Some(self_type) = &self.self_type {
            write!(f, "{} for {}", self.name, self_type)
        } else {
            write!(f, "{}", self.name)
        }
    }
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

    fn add_fn(
        &mut self,
        id: DefId,
        ident: Ident,
        sig: FnSig,
        body: BodyId,
        generics: &Generics,
        trait_id: Option<DefId>,
        self_type: Option<&Ty>,
    ) {
        if !self.already_seen(body) {
            self.visited_fns.push(VisitedFn {
                name: ident.to_string(),
                id,
                args: sig
                    .decl
                    .inputs
                    .iter()
                    .map(|ty| self.get_ty(ty).unwrap())
                    .collect(),
                body,
                generics: self.get_generics(generics),
                parent_trait: trait_id.map(|id| self.get_trait(id)),
                self_type: self_type.and_then(|ty| self.get_ty(ty)),
            });
        }
    }

    fn get_trait(&self, def_id: DefId) -> VisitedTrait {
        let name = self.analyzer.tcx.def_path_str(def_id).to_string();
        let items = self.analyzer.tcx.associated_items(def_id);
        VisitedTrait {
            name,
            id: def_id,
            items: items.len(),
        }
    }

    fn get_ty(&self, ty: &Ty) -> Option<String> {
        self.analyzer
            .tcx
            .sess
            .source_map()
            .span_to_snippet(ty.span)
            .ok()
    }

    fn get_generics(&self, generics: &Generics) -> Vec<String> {
        generics
            .params
            .iter()
            .filter_map(|param| match param.name {
                ParamName::Plain(ident) => Some(ident.to_string()),
                _ => None,
            })
            .collect()
    }

    fn get_item(&self, def_id: &DefId) -> &'tcx Item<'tcx> {
        self.analyzer
            .tcx
            .hir_expect_item(def_id.as_local().unwrap())
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
            ident,
            sig,
            body,
            generics,
            ..
        } = item.kind
        {
            log::trace!("Visiting fn: {}", ident);
            let fn_id = item.owner_id.def_id.to_def_id();
            self.add_fn(fn_id, ident, sig, body, generics, None, None);
        }

        walk_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'tcx ImplItem) {
        if let ImplItemKind::Fn(sig, body) = item.kind {
            let impl_id = self
                .analyzer
                .tcx
                .hir_get_parent_item(item.hir_id())
                .to_def_id();

            if self.analyzer.tcx.is_builtin_derived(impl_id) {
                log::trace!("Skipping impl fn generated by derive: {}", item.ident);
            } else {
                log::trace!("Visiting impl fn: {}", item.ident);

                let trait_id = self.analyzer.tcx.impl_opt_trait_id(impl_id);
                let (self_type, impl_generics) = {
                    if let ItemKind::Impl(impl_obj) = &self.get_item(&impl_id).kind {
                        (impl_obj.self_ty, &impl_obj.generics)
                    } else {
                        panic!("Expected impl item");
                    }
                };

                self.add_fn(
                    impl_id,
                    item.ident,
                    sig,
                    body,
                    impl_generics,
                    trait_id,
                    Some(self_type),
                );
            }
        }

        walk_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'tcx TraitItem) {
        if let TraitItemKind::Fn(sig, TraitFn::Provided(body)) = item.kind {
            log::trace!("Visiting trait fn: {}", item.ident);
            let trait_id = self
                .analyzer
                .tcx
                .hir_get_parent_item(item.hir_id())
                .to_def_id();
            let trait_generics = {
                if let ItemKind::Trait(_, _, _, _, generics, _, _) = &self.get_item(&trait_id).kind
                {
                    generics
                } else {
                    panic!("Expected trait item");
                }
            };
            self.add_fn(
                trait_id,
                item.ident,
                sig,
                body,
                trait_generics,
                Some(trait_id),
                None,
            );
        }

        walk_trait_item(self, item);
    }
}
