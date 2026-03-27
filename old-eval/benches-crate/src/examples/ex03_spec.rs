// Example 03 — WITH macros: Hash computation specialization
use spec_trait_macro::{spec, when};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashComputer;

pub trait HashTrait<T> {
    fn hash_all(&self, data: &[T]) -> u64;
}

impl<T: Hash> HashTrait<T> for HashComputer {
    #[inline(never)]
    fn hash_all(&self, data: &[T]) -> u64 {
        let mut h = DefaultHasher::new();
        for item in data {
            item.hash(&mut h);
        }
        h.finish()
    }
}

#[when(T = u8)]
impl<T> HashTrait<T> for HashComputer {
    #[inline(never)]
    fn hash_all(&self, data: &[T]) -> u64 {
        // FNV-1a for u8 slices — fast path
        let mut hash: u64 = 0xcbf29ce484222325;
        for &b in data {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
}

#[when(T = i32)]
impl<T> HashTrait<T> for HashComputer {
    #[inline(never)]
    fn hash_all(&self, data: &[T]) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &v in data {
            let bytes = v.to_le_bytes();
            for b in bytes {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
        hash
    }
}

pub fn hash_u8(c: &HashComputer, data: &[u8]) -> u64 {
    spec! { c.hash_all(data); HashComputer; [&[u8]] }
}

pub fn hash_i32(c: &HashComputer, data: &[i32]) -> u64 {
    spec! { c.hash_all(data); HashComputer; [&[i32]] }
}

pub fn hash_generic<T: Hash>(c: &HashComputer, data: &[T]) -> u64 {
    HashTrait::hash_all(c, data)
}
