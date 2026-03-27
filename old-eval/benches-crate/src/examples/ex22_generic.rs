// Example 22 — GENERIC: Fn trait bound specialization


// --- generic fallback ---
#[inline(never)]
pub fn dispatch_generic<T>(_x: T) -> i64 {
    0
}

// --- manual specialization ---
#[inline(never)]
pub fn dispatch_fn_i32(f: fn(i32) -> i32) -> i64 {
    f(42) as i64
}

#[inline(never)]
pub fn dispatch_fn_ref_i32(f: fn(&i32) -> i32) -> i64 {
    let v = 42i32;
    f(&v) as i64
}

#[inline(never)]
pub fn dispatch_fn_i32_i32(f: fn(i32, i32) -> i32) -> i64 {
    f(21, 21) as i64
}

