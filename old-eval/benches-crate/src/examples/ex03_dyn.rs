// Example 03 — DYN: Hash computation
use std::hash::{Hash, Hasher};


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

