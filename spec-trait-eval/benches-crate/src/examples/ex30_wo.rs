// Example 30 — WITHOUT macros: Combined trait bounds (T: Copy + Eq)
use std::any::TypeId;

// --- Approach A: enum dispatch ---
pub enum DedupEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}

#[inline(never)]
pub fn dedup_enum(variant: &DedupEnum) -> usize {
    match variant {
        DedupEnum::U8(data) => {
            let mut seen = [false; 256];
            let mut count = 0usize;
            for &b in *data {
                let idx = b as usize;
                if !seen[idx] {
                    seen[idx] = true;
                    count += 1;
                }
            }
            count
        }
        DedupEnum::I32(data) => {
            let mut buf: Vec<i32> = Vec::with_capacity(data.len());
            let mut count = 0usize;
            for &item in *data {
                if !buf.contains(&item) {
                    buf.push(item);
                    count += 1;
                }
            }
            count
        }
    }
}

// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn dedup_runtime_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut seen = [false; 256];
        let mut count = 0usize;
        for &b in data {
            if !seen[b as usize] {
                seen[b as usize] = true;
                count += 1;
            }
        }
        count
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn dedup_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let mut buf: Vec<i32> = Vec::with_capacity(data.len());
        let mut count = 0usize;
        for &item in data {
            if !buf.contains(&item) {
                buf.push(item);
                count += 1;
            }
        }
        count
    } else {
        data.len()
    }
}

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

// --- Bulk functions ---
#[inline(never)]
pub fn sum_dedup_enum_u8(chunks: &[&[u8]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        let e = DedupEnum::U8(chunk);
        total = total.wrapping_add(dedup_enum(&e));
    }
    total
}

#[inline(never)]
pub fn sum_dedup_runtime_u8(chunks: &[&[u8]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(dedup_runtime_u8(chunk));
    }
    total
}

#[inline(never)]
pub fn sum_dedup_generic_u8(chunks: &[&[u8]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(dedup_generic_u8(chunk));
    }
    total
}

#[inline(never)]
pub fn sum_dedup_manual_u8(chunks: &[&[u8]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(dedup_manual_u8(chunk));
    }
    total
}
