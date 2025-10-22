use super::fn_tree::FnTree;
use super::Analyzer;
use rustc_hir::{
    intravisit::{nested_filter::NestedFilter, FnKind, Visitor, VisitorExt},
    HirId, Impl, Item, ItemKind,
};
use rustc_middle::ty;
use std::collections::HashSet;

#[allow(dead_code)]
pub struct STIVisitor<'tcx, 'a> {
    // The analyzer contains the `TyCtxt`
    analyzer: &'a Analyzer<'tcx>,

    visited_traits: HashSet<rustc_hir::OwnerId>,
}

// Guardare le tre diverse tipologie di linear: copy move e borrow
impl<'tcx, 'a> STIVisitor<'tcx, 'a> {
    pub fn new(analyzer: &'a Analyzer<'tcx>) -> Self {
        Self {
            analyzer,
            visited_traits: HashSet::new(),
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
    type MaybeTyCtxt = ty::TyCtxt<'tcx>;
    /// Whether the visitor visits nested "item-like" things.
    /// E.g., item, impl-item.
    const INTER: bool = true;
    /// Whether the visitor visits "intra item-like" things.
    /// E.g., function body, closure, `AnonConst`
    const INTRA: bool = true;
}

// NOTE(bruzzone): `visit_ty_unambig` and `visit_const_arg_unambig` are defined in VisitorExt, so we need to import it.
impl<'tcx> Visitor<'tcx> for STIVisitor<'tcx, '_> {
    type NestedFilter = NestedFilterAll;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.analyzer.tcx
    }

    fn visit_nested_body(&mut self, id: rustc_hir::BodyId) -> Self::Result {
        let body = self.maybe_tcx().hir_body(id);
        let mut tree_visitor = FnTree::new(self.analyzer);
        log::debug!("Building function tree for body id: {:?}", id);
        tree_visitor.build_from_body(body);
        let node = tree_visitor.root;
        log::debug!("Function tree built: {:#?}", node);
    }

    fn visit_item(&mut self, item: &'tcx Item) {
        log::debug!("Visiting item: {:?}", item);

        self.visit_id(item.hir_id());

        match item.kind {
            ItemKind::ExternCrate(orig_name, ident) => {
                if let Some(orig_name) = orig_name {
                    self.visit_name(orig_name);
                }
                self.visit_ident(ident);
            }
            ItemKind::Use(path, _) => {
                self.visit_use(path, item.hir_id());
            }
            ItemKind::Static(_, ident, typ, body) => {
                self.visit_ident(ident);
                self.visit_ty_unambig(typ);
                self.visit_nested_body(body);
            }
            ItemKind::Const(ident, generics, typ, body) => {
                self.visit_ident(ident);
                self.visit_generics(generics);
                self.visit_ty_unambig(typ);
                self.visit_nested_body(body);
            }
            ItemKind::Fn {
                ident,
                sig,
                generics,
                body: body_id,
                ..
            } => {
                self.visit_ident(ident);
                self.visit_fn(
                    FnKind::ItemFn(ident, generics, sig.header),
                    sig.decl,
                    body_id,
                    item.span,
                    item.owner_id.def_id,
                );
            }
            ItemKind::Macro(ident, _, _) => {
                self.visit_ident(ident);
            }
            ItemKind::Mod(ident, module) => {
                self.visit_ident(ident);
                self.visit_mod(module, item.span, item.hir_id());
            }
            ItemKind::ForeignMod { abi: _, items } => {
                for item in items {
                    self.visit_foreign_item_ref(item);
                }
                // walk_list!(visitor, visit_foreign_item_ref, items);
            }
            ItemKind::GlobalAsm { asm: _, fake_body } => {
                self.visit_nested_body(fake_body);
            }
            ItemKind::TyAlias(ident, generics, ty) => {
                self.visit_ident(ident);
                self.visit_generics(generics);
                self.visit_ty_unambig(ty);
            }
            ItemKind::Enum(ident, generics, ref enum_definition) => {
                self.visit_ident(ident);
                self.visit_generics(generics);
                self.visit_enum_def(enum_definition);
            }
            ItemKind::Impl(Impl {
                generics,
                of_trait,
                self_ty,
                items,
            }) => {
                self.visit_generics(generics);
                if let Some(trait_header) = of_trait {
                    self.visit_trait_ref(&trait_header.trait_ref);
                }
                self.visit_ty_unambig(self_ty);
                for item in items {
                    self.visit_impl_item_ref(item);
                }
            }
            ItemKind::Struct(ident, generics, ref struct_definition)
            | ItemKind::Union(ident, generics, ref struct_definition) => {
                self.visit_ident(ident);
                self.visit_generics(generics);
                self.visit_variant_data(struct_definition);
            }
            ItemKind::Trait(
                _constness,
                _is_auto,
                _safety,
                ident,
                generics,
                bounds,
                trait_item_refs,
            ) => {
                self.visit_ident(ident);
                self.visit_generics(generics);
                for bound in bounds {
                    self.visit_param_bound(bound);
                }
                for trait_item_ref in trait_item_refs {
                    self.visited_traits.insert(trait_item_ref.owner_id);
                    // let owner_id = self.visited_traits.iter().last().unwrap();
                    // let trait_item = self.analyzer.tcx.hir_owner_node(*owner_id).expect_trait_item();

                    self.visit_trait_item_ref(trait_item_ref);
                }
            }
            ItemKind::TraitAlias(ident, generics, bounds) => {
                self.visit_ident(ident);
                self.visit_generics(generics);
                for bound in bounds {
                    self.visit_param_bound(bound);
                }
            }
        }

        // Continue walking the item.
        rustc_hir::intravisit::walk_item(self, item);
    }
}
