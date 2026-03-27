// Example 11 — RUNTIME TYPEID: Contains / search
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn contains_runtime_u8(haystack: &[u8], needle: &u8) -> bool {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        memchr::memchr(*needle, haystack).is_some()
    } else {
        haystack.contains(needle)
    }
}

