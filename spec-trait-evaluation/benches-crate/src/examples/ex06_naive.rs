// Ex06 -- naive: dedup — HashSet (no specialization)
use std::collections::HashSet;
use std::hash::Hash;

#[inline(never)]
pub fn dedup_naive<T: Clone + Eq + Hash>(data: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in data {
        if seen.insert(item) {
            out.push(item.clone());
        }
    }
    out
}
