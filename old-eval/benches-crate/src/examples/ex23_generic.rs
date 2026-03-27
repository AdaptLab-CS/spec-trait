// Example 23 — GENERIC: Multiple self types (same trait across different receivers)


// --- manual specialization ---
#[inline(never)]
pub fn transform_a_u8() -> i64 { 1 }

#[inline(never)]
pub fn transform_a_i32() -> i64 { 2 }

#[inline(never)]
pub fn transform_b_u8() -> i64 { 10 }

#[inline(never)]
pub fn transform_generic(x: u8) -> i64 { x as i64 }

