// Ex07 -- naive: hash_key — SipHash for all keys (no specialization)
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashOp;

pub trait NaiveHashKeyTrait<K> {
    fn hash_all(&self, keys: &[K]) -> u64;
}

impl<K: Hash> NaiveHashKeyTrait<K> for HashOp {
    #[inline(never)]
    fn hash_all(&self, keys: &[K]) -> u64 {
        let mut acc: u64 = 0;
        for key in keys {
            let mut h = DefaultHasher::new();
            key.hash(&mut h);
            acc ^= h.finish();
        }
        acc
    }
}

pub fn hash_all_naive(h: &HashOp, keys: &[u64]) -> u64 {
    h.hash_all(keys)
}
