use super::tree::BodyTree;
use super::Analyzer;
use rustc_hir::intravisit::nested_filter::NestedFilter;
use rustc_hir::intravisit::{walk_impl_item, walk_item, walk_trait_item, Visitor};
use rustc_hir::{
    BodyId, FnSig, Generics, HirId, ImplItem, ImplItemKind, Item, ItemKind, ParamName, TraitFn,
    TraitItem, TraitItemKind, Ty,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::DefId;
use rustc_span::Ident;
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub struct VisitedTrait {
    pub name: String,
    pub id: DefId,
    pub items: usize,
    pub self_type: Option<String>,
}

#[derive(Debug)]
pub struct VisitedFn {
    /// Name of the function
    pub name: String,
    /// DefId of the function
    pub id: DefId,
    /// Argument types as Strings
    pub args: Vec<String>,
    /// BodyId of the function
    pub body: BodyId,
    /// all the generics that might be used in the fn (both fn generics and impl / trait generics)
    pub generics: Vec<String>,
    /// If the function is part of a trait impl or a trait, this contains the trait info
    pub parent_trait: Option<VisitedTrait>,
    /// If the function is part of a trait impl or an inherent impl, this contains the self type info
    pub self_type: Option<String>,
}

impl Display for VisitedFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

impl Display for VisitedTrait {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(self_type) = &self.self_type {
            write!(f, "{} for {}", self.name, self_type)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl Hash for VisitedTrait {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.self_type.hash(state);
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

    fn already_seen(&self, id: DefId) -> bool {
        self.visited_fns.iter().any(|f| f.id == id)
    }

    fn add_fn(
        &mut self,
        id: DefId,
        ident: Ident,
        sig: FnSig,
        body: BodyId,
        generics: &Generics,
        parent: Option<(Option<DefId>, Option<&Ty>)>,
    ) {
        if !self.already_seen(id) {
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
                parent_trait: parent
                    .and_then(|(trait_id, ty)| trait_id.map(|id| self.get_trait(id, ty))),
                self_type: parent.and_then(|(_, ty)| ty.and_then(|ty| self.get_ty(ty))),
            });
        }
    }

    fn get_trait(&self, def_id: DefId, self_type: Option<&Ty>) -> VisitedTrait {
        let name = self.analyzer.tcx.def_path_str(def_id).to_string();
        let items = self.analyzer.tcx.associated_items(def_id);
        VisitedTrait {
            name,
            id: def_id,
            items: items.len(),
            self_type: self_type.and_then(|ty| self.get_ty(ty)),
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
            self.add_fn(fn_id, ident, sig, body, generics, None);
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

                let fn_id = item.owner_id.def_id.to_def_id();
                let trait_id = self.analyzer.tcx.impl_opt_trait_id(impl_id);
                let (self_type, impl_generics) = {
                    if let ItemKind::Impl(impl_obj) = &self.get_item(&impl_id).kind {
                        (impl_obj.self_ty, &impl_obj.generics)
                    } else {
                        panic!("Expected impl item");
                    }
                };

                self.add_fn(
                    fn_id,
                    item.ident,
                    sig,
                    body,
                    impl_generics,
                    Some((trait_id, Some(self_type))),
                );
            }
        }

        walk_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'tcx TraitItem) {
        if let TraitItemKind::Fn(sig, TraitFn::Provided(body)) = item.kind {
            log::trace!("Visiting trait fn: {}", item.ident);

            let fn_id = item.owner_id.def_id.to_def_id();
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
                fn_id,
                item.ident,
                sig,
                body,
                trait_generics,
                Some((Some(trait_id), None)),
            );
        }

        walk_trait_item(self, item);
    }
}
