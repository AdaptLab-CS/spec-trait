// Ex05 -- spec: Distinct element count, specialized for u8
use spec_trait_macro::{spec, when};
use std::collections::HashSet;
use std::hash::Hash;

pub struct DistinctOp;

pub trait DistinctTrait<T> {
    fn distinct(&self, data: &[T]) -> usize;
}

impl<T: Eq + Hash> DistinctTrait<T> for DistinctOp {
    #[inline(never)]
    fn distinct(&self, data: &[T]) -> usize {
        let set: HashSet<&T> = data.iter().collect();
        set.len()
    }
}

#[when(T = u8)]
impl<T> DistinctTrait<T> for DistinctOp {
    #[inline(never)]
    fn distinct(&self, data: &[T]) -> usize {
        let mut seen = [false; 256];
        for &b in data {
            seen[b as usize] = true;
        }
        seen.iter().filter(|&&v| v).count()
    }
}

pub fn distinct_u8(op: &DistinctOp, data: &[u8]) -> usize {
    spec! { op.distinct(data); DistinctOp; [&[u8]] }
}

pub fn _distinct_generic<T: Eq + Hash>(_op: &DistinctOp, data: &[T]) -> usize {
    let set: HashSet<&T> = data.iter().collect();
    set.len()
}
