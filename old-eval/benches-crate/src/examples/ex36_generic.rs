// Example 36 — GENERIC: Triple trait bounds (Clone + Eq + Ord)


// --- generic fallback ---
#[inline(never)]
pub fn sorted_unique_count_generic_i32(data: &[i32]) -> usize {
    let mut count = 0usize;
    for i in 0..data.len() {
        let mut unique = true;
        for j in 0..i {
            if data[j] == data[i] {
                unique = false;
                break;
            }
        }
        if unique {
            count += 1;
        }
    }
    count
}

#[inline(never)]
pub fn sorted_unique_count_generic_u8(data: &[u8]) -> usize {
    let mut count = 0usize;
    for i in 0..data.len() {
        let mut unique = true;
        for j in 0..i {
            if data[j] == data[i] {
                unique = false;
                break;
            }
        }
        if unique {
            count += 1;
        }
    }
    count
}

