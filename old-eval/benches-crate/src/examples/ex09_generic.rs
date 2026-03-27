// Example 09 — GENERIC: Display-based formatting


// --- generic fallback ---
#[inline(never)]
pub fn fmt_generic<T: std::fmt::Debug>(items: &[T]) -> String {
    items.iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(", ")
}
