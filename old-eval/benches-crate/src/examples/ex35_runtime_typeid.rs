// Example 35 — RUNTIME TYPEID: Two-generic concrete pair specialization
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn merge_count_runtime_i32_u8(ts: &[i32], us: &[u8]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() && TypeId::of::<u8>() == TypeId::of::<u8>() {
        let mut total = 0usize;
        for &t in ts {
            total = total.wrapping_add(t as usize);
        }
        for &u in us {
            total = total.wrapping_add(u as usize);
        }
        total
    } else if TypeId::of::<i32>() == TypeId::of::<i32>() {
        ts.len() * 4 + us.len()
    } else {
        ts.len() + us.len()
    }
}

