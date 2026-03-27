// Example 28 — GENERIC: Tuple type specialization


// --- Approach C: generic fallback ---
#[inline(never)]
pub fn squared_sum_generic_i32(data: &[(i32, i32)]) -> f64 {
    let mut sum = 0.0f64;
    for &(x, y) in data {
        let fx = x as f64;
        let fy = y as f64;
        sum += fx * fx + fy * fy;
    }
    sum
}

#[inline(never)]
pub fn squared_sum_generic_f64(data: &[(f64, f64)]) -> f64 {
    let mut sum = 0.0f64;
    for &(x, y) in data {
        sum += x * x + y * y;
    }
    sum
}

// --- Approach D: manual specialization ---
#[inline(never)]
pub fn squared_sum_manual_i32(data: &[(i32, i32)]) -> f64 {
    let mut sum = 0i64;
    for &(x, y) in data {
        sum += (x as i64) * (x as i64) + (y as i64) * (y as i64);
    }
    sum as f64
}

#[inline(never)]
pub fn squared_sum_manual_f64(data: &[(f64, f64)]) -> f64 {
    let mut sum = 0.0f64;
    for &(x, y) in data {
        sum += x * x + y * y;
    }
    sum
}

