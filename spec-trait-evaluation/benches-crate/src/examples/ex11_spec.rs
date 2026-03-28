// Ex11 -- spec: map_transform — higher-order function pointer specialization
//
// Pattern: #[when(F = for<'a> fn(&'a T) -> T)]
// This higher-ranked function pointer type pattern cannot be expressed with enum, TypeId, or naive.
// Only spec-trait can match on for<'a> fn(&'a T) -> T function pointer types.
use spec_trait_macro::{spec, when};

pub struct Mapper;

pub trait MapTransformTrait<F, T> {
    fn map_transform(&self, data: &[T], f: F) -> Vec<T>;
}

impl<F: Fn(&T) -> T, T> MapTransformTrait<F, T> for Mapper {
    #[inline(never)]
    fn map_transform(&self, data: &[T], f: F) -> Vec<T> {
        // Default: call through generic Fn trait (indirect call, may not inline)
        data.iter().map(|x| f(x)).collect()
    }
}

#[when(F = for<'a> fn(&'a T) -> T)]
impl<F, T: Clone> MapTransformTrait<F, T> for Mapper {
    #[inline(never)]
    fn map_transform(&self, data: &[T], f: F) -> Vec<T> {
        // Specialized: F is a concrete fn pointer, direct call (no vtable/closure indirection)
        let fp = unsafe { *(&f as *const F as *const fn(&T) -> T) };
        let mut result = Vec::with_capacity(data.len());
        for item in data {
            result.push(fp(item));
        }
        result
    }
}

fn double_i32(x: &i32) -> i32 {
    x.wrapping_mul(2)
}

pub fn map_transform_fn_ptr(m: &Mapper, data: &[i32]) -> Vec<i32> {
    let f: for<'a> fn(&'a i32) -> i32 = double_i32;
    spec! { m.map_transform(data, f); Mapper; [&[i32], for<'a> fn(&'a i32) -> i32] }
}
