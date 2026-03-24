// Example 7 — WITHOUT macros: Hash computation
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// --- dyn dispatch ---
pub trait DynHash {
    fn hash_dyn(&self) -> u64;
}
impl DynHash for Vec<u8> {
    #[inline(never)]
    fn hash_dyn(&self) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &b in self { hash ^= b as u64; hash = hash.wrapping_mul(0x100000001b3); }
        hash
    }
}
impl DynHash for Vec<i32> {
    #[inline(never)]
    fn hash_dyn(&self) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &v in self {
            for b in v.to_le_bytes() { hash ^= b as u64; hash = hash.wrapping_mul(0x100000001b3); }
        }
        hash
    }
}

pub fn hash_dyn(data: &dyn DynHash) -> u64 { data.hash_dyn() }

// --- enum dispatch ---
pub enum HashEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}
#[inline(never)]
pub fn hash_enum_all(variant: &HashEnum<'_>) -> u64 {
    match variant {
        HashEnum::U8(d) => {
            let mut h: u64 = 0xcbf29ce484222325;
            for &b in *d { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); }
            h
        }
        HashEnum::I32(d) => {
            let mut h: u64 = 0xcbf29ce484222325;
            for &v in *d {
                for b in v.to_le_bytes() { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); }
            }
            h
        }
    }
}

// --- generic (DefaultHasher) ---
#[inline(never)]
pub fn hash_generic<T: Hash>(data: &[T]) -> u64 {
    let mut h = DefaultHasher::new();
    for item in data { item.hash(&mut h); }
    h.finish()
}
