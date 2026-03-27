// Ex07 -- naive: hash_key — SipHash for all keys (no specialization)
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[inline(never)]
pub fn hash_all_naive<K: Hash>(keys: &[K]) -> u64 {
    let mut acc: u64 = 0;
    for key in keys {
        let mut h = DefaultHasher::new();
        key.hash(&mut h);
        acc ^= h.finish();
    }
    acc
}
