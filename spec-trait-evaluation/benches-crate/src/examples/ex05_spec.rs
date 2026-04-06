// Ex05 -- spec: histogram — direct byte index vs hash bucket
use spec_trait_macro::{spec, when};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HistogramOp;

pub trait BucketTrait<T> {
    fn bucket(&self, item: &T) -> usize;
}

impl<T: Hash> BucketTrait<T> for HistogramOp {
    #[inline(never)]
    fn bucket(&self, item: &T) -> usize {
        let mut h = DefaultHasher::new();
        item.hash(&mut h);
        (h.finish() % 256) as usize
    }
}

#[when(T = u8)]
impl<T> BucketTrait<T> for HistogramOp {
    #[inline(never)]
    fn bucket(&self, item: &T) -> usize {
        unsafe { *(item as *const T as *const u8) as usize }
    }
}

#[inline(always)]
fn bucket_spec(h: &HistogramOp, item: &u8) -> usize {
    spec! { h.bucket(item); HistogramOp; [&u8] }
}

pub fn max_freq_u8(h: &HistogramOp, data: &[u8]) -> u32 {
    let mut counts = [0u32; 256];
    for item in data {
        counts[bucket_spec(h, item)] += 1;
    }
    counts.iter().copied().max().unwrap_or(0)
}
