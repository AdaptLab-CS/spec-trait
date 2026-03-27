// Example 03 — GENERIC: Hash computation
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


// --- generic (DefaultHasher) ---
#[inline(never)]
pub fn hash_generic<T: Hash>(data: &[T]) -> u64 {
    let mut h = DefaultHasher::new();
    for item in data { item.hash(&mut h); }
    h.finish()
}
