// Ex07 -- enum dispatch: hash_key
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub enum HashEnum<'a, K> {
    U64(&'a [u64]),
    Generic(&'a [K]),
}

#[inline(never)]
fn hash_one_u64_direct(key: &u64) -> u64 {
    *key
}

pub trait EnumHashKeyTrait<K> {
    fn hash_all(&self) -> u64;
}

impl<K: Hash> EnumHashKeyTrait<K> for HashEnum<'_, K> {
    #[inline(never)]
    fn hash_all(&self) -> u64 {
        match self {
            HashEnum::U64(keys) => {
                let mut acc: u64 = 0;
                for k in *keys {
                    acc ^= hash_one_u64_direct(k);
                }
                acc
            }
            HashEnum::Generic(keys) => {
                let mut acc: u64 = 0;
                for key in *keys {
                    let mut h = DefaultHasher::new();
                    key.hash(&mut h);
                    acc ^= h.finish();
                }
                acc
            }
        }
    }
}

pub fn hash_all_enum(e: &HashEnum<u64>) -> u64 {
    e.hash_all()
}
