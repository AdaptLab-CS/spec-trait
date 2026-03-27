// Example 29 — GENERIC: Nested generic type specialization


// --- Approach C: generic fallback ---
#[inline(never)]
pub fn flat_generic_vec_vec_u8(data: &[Vec<Vec<u8>>]) -> usize {
    let mut total = 0usize;
    for outer in data {
        for inner in outer {
            total += inner.len();
        }
    }
    total
}

#[inline(never)]
pub fn flat_generic_vec_i32(data: &[Vec<i32>]) -> usize {
    let mut total = 0usize;
    for outer in data {
        total += outer.len() * 4;
    }
    total
}

// --- Approach D: manual specialization ---
#[inline(never)]
pub fn flat_manual_vec_vec_u8(data: &[Vec<Vec<u8>>]) -> usize {
    let mut total = 0usize;
    for outer in data {
        for inner in outer {
            total += inner.len();
        }
    }
    total
}

#[inline(never)]
pub fn flat_manual_vec_i32(data: &[Vec<i32>]) -> usize {
    let mut total = 0usize;
    for outer in data {
        total += outer.len() * 4;
    }
    total
}

