// Ex07 -- naive: hash_key — SipHash for all keys (no specialization)
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashOp;

pub trait NaiveHashOneTrait<K> {
    fn hash_one(&self, key: &K) -> u64;
}

impl<K: Hash> NaiveHashOneTrait<K> for HashOp {
    #[inline(never)]
    fn hash_one(&self, key: &K) -> u64 {
        let mut h = DefaultHasher::new();
        key.hash(&mut h);
        h.finish()
    }
}

pub fn hash_all_naive(h: &HashOp, keys: &[u64]) -> u64 {
    let mut acc: u64 = 0;
    for key in keys {
        acc ^= h.hash_one(key);
    }
    acc
}
