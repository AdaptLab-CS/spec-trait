use super::Analyzer;
use rustc_hir::{
    intravisit::{FnKind, Visitor, VisitorExt},
    HirId, Impl, Item, ItemKind, TraitImplHeader,
};

// TODO(bruzzone): remove when `analyzer` is used.
#[allow(dead_code)]
pub struct STIVisitor<'tcx, 'a> {
    // The analyzer contains the `TyCtxt`
    analyzer: &'a Analyzer<'tcx>,
}

// Guardare le tre diverse tipologie di linear: copy move e borrow
impl<'tcx, 'a> STIVisitor<'tcx, 'a> {
    pub fn new(analyzer: &'a Analyzer<'tcx>) -> Self {
        Self { analyzer }
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
    fn visit_item(&mut self, item: &'tcx Item) {
        log::debug!("Visiting item: {:?}", item);

        let Item {
            owner_id: _,
            kind,
            span: _,
            vis_span: _,
            has_delayed_lints: _,
        } = item;
        self.visit_id(item.hir_id());

        match *kind {
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
                if let Some(TraitImplHeader {
                    constness: _,
                    safety: _,
                    polarity: _,
                    defaultness: _,
                    defaultness_span: _,
                    trait_ref,
                }) = of_trait
                {
                    self.visit_trait_ref(trait_ref);
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
