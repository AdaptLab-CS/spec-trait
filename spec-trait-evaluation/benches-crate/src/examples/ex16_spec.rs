// Ex16 -- spec: total_capacity — wildcard and type alias specialization
//
// Patterns: #[when(T = Vec<MyAlias>)] and #[when(T = Vec<_>)]
// Wildcard type matching (Vec<_> = any Vec) and type alias resolution (MyAlias = u8)
// cannot be expressed with enum, TypeId, or naive dispatch.
// Only spec-trait can match on Vec<_> (any Vec regardless of element type) and
// distinguish Vec<MyAlias> from Vec<_> using alias resolution.
use spec_trait_macro::{spec, when};

type MyAlias = u8;

pub struct CapacityOp;

pub trait TotalCapacityTrait<T> {
    fn total_capacity(&self, items: &[T]) -> usize;
}

impl<T> TotalCapacityTrait<T> for CapacityOp {
    #[inline(never)]
    fn total_capacity(&self, items: &[T]) -> usize {
        // Default: use size_of for each element
        items.len() * std::mem::size_of::<T>()
    }
}

#[when(T = Vec<MyAlias>)]
impl<T> TotalCapacityTrait<T> for CapacityOp {
    #[inline(never)]
    fn total_capacity(&self, items: &[T]) -> usize {
        // Specialized: T = Vec<u8> (via MyAlias) — direct capacity sum, 1 byte per element
        let vecs = unsafe {
            std::slice::from_raw_parts(items.as_ptr() as *const Vec<u8>, items.len())
        };
        let mut total: usize = 0;
        for v in vecs {
            total += v.capacity(); // 1 byte per u8
        }
        total
    }
}

#[when(T = Vec<_>)]
impl<T> TotalCapacityTrait<T> for CapacityOp {
    #[inline(never)]
    fn total_capacity(&self, items: &[T]) -> usize {
        // Specialized: T = Vec<_> (any Vec) — capacity * element size
        // We know T is some Vec<E> but don't know E, so use size_of::<T>
        // as a proxy for the Vec's metadata size
        let mut total: usize = 0;
        for item in items {
            total += std::mem::size_of_val(item);
        }
        total
    }
}

pub fn total_capacity_vec_u8(op: &CapacityOp, items: &[Vec<u8>]) -> usize {
    spec! { op.total_capacity(items); CapacityOp; [&[Vec<u8>]]; u8 = MyAlias }
}
