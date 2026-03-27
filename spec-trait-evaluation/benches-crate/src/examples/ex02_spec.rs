// Ex02 -- spec: Set intersection count, specialized for u8
use spec_trait_macro::{spec, when};
use std::collections::HashSet;
use std::hash::Hash;

pub struct IntersectOp;

pub trait IntersectTrait<T> {
    fn intersection_count(&self, a: &[T], b: &[T]) -> usize;
}

impl<T: Eq + Hash> IntersectTrait<T> for IntersectOp {
    #[inline(never)]
    fn intersection_count(&self, a: &[T], b: &[T]) -> usize {
        let set: HashSet<&T> = a.iter().collect();
        b.iter().filter(|x| set.contains(x)).count()
    }
}

#[when(T = u8)]
impl<T> IntersectTrait<T> for IntersectOp {
    #[inline(never)]
    fn intersection_count(&self, a: &[T], b: &[T]) -> usize {
        let mut present = [false; 256];
        for &v in a {
            present[v as usize] = true;
        }
        b.iter().filter(|&&v| present[v as usize]).count()
    }
}

pub fn intersect_u8(op: &IntersectOp, a: &[u8], b: &[u8]) -> usize {
    spec! { op.intersection_count(a, b); IntersectOp; [&[u8], &[u8]] }
}

pub fn _intersect_generic<T: Eq + Hash>(_op: &IntersectOp, a: &[T], b: &[T]) -> usize {
    let set: HashSet<&T> = a.iter().collect();
    b.iter().filter(|x| set.contains(x)).count()
}
