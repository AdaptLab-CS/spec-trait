// Ex07 -- runtime TypeId dispatch: hash_key
use std::any::TypeId;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashOp;

pub trait RtHashOneTrait<K> {
    fn hash_one(&self, key: &K) -> u64;
}

impl<K: Hash + 'static> RtHashOneTrait<K> for HashOp {
    #[inline(never)]
    fn hash_one(&self, key: &K) -> u64 {
        if TypeId::of::<K>() == TypeId::of::<u64>() {
            unsafe { *(key as *const K as *const u64) }
        } else {
            let mut h = DefaultHasher::new();
            key.hash(&mut h);
            h.finish()
        }
    }
}

pub fn hash_all_runtime(h: &HashOp, keys: &[u64]) -> u64 {
    let mut acc: u64 = 0;
    for key in keys {
        acc ^= h.hash_one(key);
    }
    acc
}
