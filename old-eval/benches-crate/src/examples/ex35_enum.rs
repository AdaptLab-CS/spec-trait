// Example 35 — ENUM: Two-generic concrete pair specialization


// --- enum dispatch ---
pub enum MergeEnum<'a> {
    I32U8(&'a [i32], &'a [u8]),
    I32F64(&'a [i32], &'a [f64]),
    F64U8(&'a [f64], &'a [u8]),
}

#[inline(never)]
pub fn merge_count_enum(variant: &MergeEnum<'_>) -> usize {
    match variant {
        MergeEnum::I32U8(ts, us) => {
            let mut total = 0usize;
            for &t in *ts {
                total = total.wrapping_add(t as usize);
            }
            for &u in *us {
                total = total.wrapping_add(u as usize);
            }
            total
        }
        MergeEnum::I32F64(ts, us) => ts.len() * 4 + us.len(),
        MergeEnum::F64U8(ts, us) => ts.len() + us.len() * 2,
    }
}

