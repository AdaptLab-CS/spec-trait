// Ex06 -- runtime TypeId dispatch: transform
use std::any::TypeId;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct Deduplicator;

pub trait RtTransformTrait<T> {
    fn transform(&self, item: &T) -> u8;
}

impl<T: Hash + 'static> RtTransformTrait<T> for Deduplicator {
    #[inline(never)]
    fn transform(&self, item: &T) -> u8 {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let b = unsafe { *(item as *const T as *const u8) };
            b ^ 0x55
        } else {
            let mut h = DefaultHasher::new();
            item.hash(&mut h);
            (h.finish() & 0xFF) as u8
        }
    }
}

pub fn dedup_runtime(d: &Deduplicator, data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());
    for item in data {
        result.push(d.transform(item));
    }
    result
}
