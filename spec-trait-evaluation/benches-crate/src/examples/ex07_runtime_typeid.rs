// Ex07 -- runtime TypeId dispatch: hash_key
use std::any::TypeId;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashOp;

pub trait RtHashKeyTrait<K> {
    fn hash_all(&self, keys: &[K]) -> u64;
}

impl<K: Hash + 'static> RtHashKeyTrait<K> for HashOp {
    #[inline(never)]
    fn hash_all(&self, keys: &[K]) -> u64 {
        if TypeId::of::<K>() == TypeId::of::<u64>() {
            let slice = unsafe {
                std::slice::from_raw_parts(keys.as_ptr() as *const u64, keys.len())
            };
            let mut acc: u64 = 0;
            for &k in slice {
                acc ^= k;
            }
            acc
        } else {
            let mut acc: u64 = 0;
            for key in keys {
                let mut h = DefaultHasher::new();
                key.hash(&mut h);
                acc ^= h.finish();
            }
            acc
        }
    }
}

pub fn hash_all_runtime(h: &HashOp, keys: &[u64]) -> u64 {
    h.hash_all(keys)
}
