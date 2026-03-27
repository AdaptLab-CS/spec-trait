// Example 26 — GENERIC: Combined trait bounds (T: Copy + Eq)


// --- Approach C: generic fallback (O(n²)) ---
#[inline(never)]
pub fn dedup_generic_u8(data: &[u8]) -> usize {
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
pub fn dedup_generic_i32(data: &[i32]) -> usize {
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

// --- Approach D: manual specialization ---
#[inline(never)]
pub fn dedup_manual_u8(data: &[u8]) -> usize {
    let mut seen = [false; 256];
    let mut count = 0usize;
    for &b in data {
        if !seen[b as usize] {
            seen[b as usize] = true;
            count += 1;
        }
    }
    count
}

#[inline(never)]
pub fn dedup_manual_i32(data: &[i32]) -> usize {
    let mut buf: Vec<i32> = Vec::with_capacity(data.len());
    let mut count = 0usize;
    for &item in data {
        if !buf.contains(&item) {
            buf.push(item);
            count += 1;
        }
    }
    count
}

