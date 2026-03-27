// Example 06 — GENERIC: Dedup


// --- generic fallback ---
#[inline(never)]
pub fn dedup_generic<T: Clone + PartialEq>(data: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(data.len());
    for item in data {
        if out.last() != Some(item) { out.push(item.clone()); }
    }
    out
}
