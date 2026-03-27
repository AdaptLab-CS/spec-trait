// Example 19 — ENUM: Multi-generic repeating (zip/combine)


// --- enum dispatch ---
pub enum CombineEnum<'a> {
    U8VecU8(&'a [u8], &'a [Vec<u8>]),
    I32VecI32(&'a [i32], &'a [Vec<i32>]),
    VecI32I32(&'a [Vec<i32>], &'a [i32]),
}

#[inline(never)]
pub fn combine_enum(variant: &CombineEnum<'_>) -> usize {
    match variant {
        CombineEnum::U8VecU8(xs, ys) => {
            let mut total = xs.len();
            for v in *ys { total += v.len(); }
            total
        }
        CombineEnum::I32VecI32(xs, ys) => {
            let mut total = xs.len();
            for v in *ys { total += v.len(); }
            total
        }
        CombineEnum::VecI32I32(xs, ys) => {
            let mut total = 0;
            for v in *xs { total += v.len(); }
            total + ys.len()
        }
    }
}

