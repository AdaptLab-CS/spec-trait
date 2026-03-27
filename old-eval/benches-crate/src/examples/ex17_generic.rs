// Example 17 — GENERIC: Lifetime specialization (static vs borrowed)


// --- generic fallback ---
#[inline(never)]
pub fn parse_generic(data: &str) -> usize {
    data.len()
}

