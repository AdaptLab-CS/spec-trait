// Ex07 -- spec: hash_key — identity hash for u64 vs SipHash (DefaultHasher)
use spec_trait_macro::{spec, when};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashOp;

pub trait HashOneTrait<K> {
    fn hash_one(&self, key: &K) -> u64;
}

impl<K: Hash> HashOneTrait<K> for HashOp {
    #[inline(never)]
    fn hash_one(&self, key: &K) -> u64 {
        let mut h = DefaultHasher::new();
        key.hash(&mut h);
        h.finish()
    }
}

#[when(K = u64)]
impl<K> HashOneTrait<K> for HashOp {
    #[inline(never)]
    fn hash_one(&self, key: &K) -> u64 {
        unsafe { *(key as *const K as *const u64) }
    }
}

#[inline(always)]
fn hash_one_spec(h: &HashOp, key: &u64) -> u64 {
    spec! { h.hash_one(key); HashOp; [&u64] }
}

pub fn hash_all_u64(h: &HashOp, keys: &[u64]) -> u64 {
    let mut acc: u64 = 0;
    for key in keys {
        acc ^= hash_one_spec(h, key);
    }
    acc
}
