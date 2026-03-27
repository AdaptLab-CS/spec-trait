// Example 17 — RUNTIME TYPEID: Lifetime specialization (static vs borrowed)
use std::any::TypeId;


// --- runtime check ---
#[inline(never)]
pub fn parse_runtime_static_str(data: &'static str) -> usize {
    if TypeId::of::<&'static str>() == TypeId::of::<&'static str>() {
        42
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn parse_runtime_borrowed_str(data: &str) -> usize {
    // Can't distinguish lifetimes at runtime; always generic path
    data.len()
}

