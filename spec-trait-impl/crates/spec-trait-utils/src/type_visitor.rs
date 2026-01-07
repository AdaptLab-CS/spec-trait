use syn::{
    GenericArgument, Ident, Lifetime, PathArguments, PathSegment, ReturnType, Type, TypeArray,
    TypeBareFn, TypeImplTrait, TypeParamBound, TypeParen, TypePath, TypeReference, TypeSlice,
    TypeTuple,
};

pub trait VisitTypeInDepth {
    /// `T`
    fn visit_ident(&mut self, _ident: &mut Ident) {}

    /// `'a`
    fn visit_lifetime(&mut self, _lifetime: &mut Lifetime) {}

    /// `T<U>`
    fn visit_segment(&mut self, segment: &mut PathSegment) {
        // `T`
        self.visit_ident(&mut segment.ident);

        // `<U>`
        if let PathArguments::AngleBracketed(angle_bracketed) = &mut segment.arguments {
            for arg in &mut angle_bracketed.args {
                if let GenericArgument::Type(ty) = arg {
                    self.visit_type(ty);
                }
            }
        }
    }

    /// General Type visitor
    fn visit_type(&mut self, ty: &mut Type) {
        self.visit_type_common(ty);
        match ty {
            Type::Tuple(type_tuple) => self.visit_type_tuple(type_tuple),
            Type::Reference(type_reference) => self.visit_type_reference(type_reference),
            Type::Array(type_array) => self.visit_type_array(type_array),
            Type::Slice(type_slice) => self.visit_type_slice(type_slice),
            Type::Paren(type_paren) => self.visit_type_parentheses(type_paren),
            Type::BareFn(type_bare_fn) => self.visit_type_function(type_bare_fn),
            Type::ImplTrait(type_impl_trait) => self.visit_type_impl_trait(type_impl_trait),
            Type::Path(type_path) => self.visit_type_path(type_path),
            _ => self.visit_type_default(ty),
        }
    }

    /// Common visitor called before specific type visitors
    fn visit_type_common(&mut self, _ty: &mut Type) {}

    /// Default visitor for types not explicitly handled
    fn visit_type_default(&mut self, _ty: &mut Type) {}

    /// `(T, U)`
    fn visit_type_tuple(&mut self, ty: &mut TypeTuple) {
        for element in &mut ty.elems {
            self.visit_type(element);
        }
    }

    /// `&T`
    fn visit_type_reference(&mut self, ty: &mut TypeReference) {
        if let Some(lifetime) = &mut ty.lifetime {
            self.visit_lifetime(lifetime);
        }
        self.visit_type(&mut ty.elem);
    }

    /// `[T; N]`
    fn visit_type_array(&mut self, ty: &mut TypeArray) {
        self.visit_type(&mut ty.elem);
    }

    /// `[T]`
    fn visit_type_slice(&mut self, ty: &mut TypeSlice) {
        self.visit_type(&mut ty.elem);
    }

    /// `(T)`
    fn visit_type_parentheses(&mut self, ty: &mut TypeParen) {
        self.visit_type(&mut ty.elem);
    }

    /// `fn(T) -> U`
    fn visit_type_function(&mut self, ty: &mut TypeBareFn) {
        for input in &mut ty.inputs {
            self.visit_type(&mut input.ty);
        }
        if let ReturnType::Type(_, ty) = &mut ty.output {
            self.visit_type(ty);
        }
    }

    /// `impl T<U>`
    fn visit_type_impl_trait(&mut self, ty: &mut TypeImplTrait) {
        for bound in &mut ty.bounds {
            if let TypeParamBound::Trait(trait_bound) = bound {
                for segment in &mut trait_bound.path.segments {
                    self.visit_segment(segment);
                }
            }
        }
    }

    /// `T, T<U>`
    fn visit_type_path(&mut self, ty: &mut TypePath) {
        for segment in &mut ty.path.segments {
            self.visit_segment(segment);
        }
    }
}
