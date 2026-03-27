// Example 11 — GENERIC: Contains / search


// --- generic fallback ---
#[inline(never)]
pub fn contains_generic<T: PartialEq>(haystack: &[T], needle: &T) -> bool {
    haystack.iter().any(|x| x == needle)
}

