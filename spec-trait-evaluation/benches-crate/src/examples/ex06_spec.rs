// Ex06 -- spec: transform — XOR remap for u8 vs hash-based remap
use spec_trait_macro::{spec, when};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct Deduplicator;

pub trait TransformTrait<T> {
    fn transform(&self, item: &T) -> u8;
}

impl<T: Hash> TransformTrait<T> for Deduplicator {
    #[inline(never)]
    fn transform(&self, item: &T) -> u8 {
        let mut h = DefaultHasher::new();
        item.hash(&mut h);
        (h.finish() & 0xFF) as u8
    }
}

#[when(T = u8)]
impl<T> TransformTrait<T> for Deduplicator {
    #[inline(never)]
    fn transform(&self, item: &T) -> u8 {
        let b = unsafe { *(item as *const T as *const u8) };
        b ^ 0x55
    }
}

#[inline(always)]
fn transform_spec(d: &Deduplicator, item: &u8) -> u8 {
    spec! { d.transform(item); Deduplicator; [&u8] }
}

pub fn dedup_u8(d: &Deduplicator, data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());
    for item in data {
        result.push(transform_spec(d, item));
    }
    result
}
