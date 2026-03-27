// Example 24 — WITH macros: T: 'static lifetime bound
// Specialize on whether T is 'static to enable caching / interning.
use spec_trait_macro::{spec, when};

pub struct Hasher;

pub trait StaticHashTrait<T> {
    fn static_hash(&self, data: &T) -> u64;
}

impl<T> StaticHashTrait<T> for Hasher {
    #[inline(never)]
    fn static_hash(&self, _data: &T) -> u64 {
        // Generic: use size_of as fallback hash
        std::mem::size_of::<T>() as u64
    }
}

#[when(T: 'static)]
impl<T> StaticHashTrait<T> for Hasher {
    #[inline(never)]
    fn static_hash(&self, data: &T) -> u64 {
        // Static data: can use ptr-based hash (stable address)
        let ptr = data as *const T as usize;
        (ptr as u64).wrapping_mul(0x517cc1b727220a95)
    }
}

pub fn hash_static_i32(h: &Hasher, data: &i32) -> u64 {
    spec! { h.static_hash(data); Hasher; [&i32]; i32: 'static }
}

pub fn hash_static_str(h: &Hasher, data: &&'static str) -> u64 {
    spec! { h.static_hash(data); Hasher; [&'static str]; &'static str: 'static }
}

pub fn hash_generic<T>(h: &Hasher, data: &T) -> u64 {
    StaticHashTrait::static_hash(h, data)
}

#[inline(never)]
pub fn sum_hash_i32(h: &Hasher, items: &[i32]) -> u64 {
    let mut total = 0u64;
    for item in items {
        total = total.wrapping_add(spec! { h.static_hash(item); Hasher; [&i32]; i32: 'static });
    }
    total
}

#[inline(never)]
pub fn sum_hash_generic<T>(h: &Hasher, items: &[T]) -> u64 {
    let mut total = 0u64;
    for item in items {
        total = total.wrapping_add(StaticHashTrait::static_hash(h, item));
    }
    total
}
