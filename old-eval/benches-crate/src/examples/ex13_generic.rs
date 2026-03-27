// Example 13 — GENERIC: Clamping


// --- generic fallback ---
#[inline(never)]
pub fn clamp_generic<T: Clone + PartialOrd>(data: &[T], lo: T, hi: T) -> Vec<T> {
    data.iter().map(|x| {
        if *x < lo { lo.clone() }
        else if *x > hi { hi.clone() }
        else { x.clone() }
    }).collect()
}
