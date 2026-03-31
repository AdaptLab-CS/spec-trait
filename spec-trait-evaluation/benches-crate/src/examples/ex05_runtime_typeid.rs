// Ex05 -- runtime TypeId dispatch: histogram
use std::any::TypeId;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HistogramOp;

pub trait RtBucketTrait<T> {
    fn bucket(&self, item: &T) -> usize;
}

impl<T: Hash + 'static> RtBucketTrait<T> for HistogramOp {
    #[inline(never)]
    fn bucket(&self, item: &T) -> usize {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            unsafe { *(item as *const T as *const u8) as usize }
        } else {
            let mut h = DefaultHasher::new();
            item.hash(&mut h);
            (h.finish() % 256) as usize
        }
    }
}

pub fn max_freq_runtime(h: &HistogramOp, data: &[u8]) -> u32 {
    let mut counts = [0u32; 256];
    for item in data {
        counts[h.bucket(item)] += 1;
    }
    counts.iter().copied().max().unwrap_or(0)
}
