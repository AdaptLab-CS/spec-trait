// Ex07 -- spec: hash_key — identity hash for u64 vs SipHash (DefaultHasher)
use spec_trait_macro::{spec, when};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashOp;

pub trait HashKeyTrait<K> {
    fn hash_all(&self, keys: &[K]) -> u64;
}

impl<K: Hash> HashKeyTrait<K> for HashOp {
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

#[when(K = u64)]
impl<K> HashKeyTrait<K> for HashOp {
    #[inline(never)]
    fn hash_all(&self, keys: &[K]) -> u64 {
        let slice = unsafe {
            std::slice::from_raw_parts(keys.as_ptr() as *const u64, keys.len())
        };
        let mut acc: u64 = 0;
        for &k in slice {
            acc ^= k;
        }
        acc
    }
}

pub fn hash_all_u64(h: &HashOp, keys: &[u64]) -> u64 {
    spec! { h.hash_all(keys); HashOp; [&[u64]] }
}
