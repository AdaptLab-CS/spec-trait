// Example 23 — WITHOUT macros: Multi-generic repeating (zip/combine)

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

// --- generic fallback ---
#[inline(never)]
pub fn combine_generic<T, U>(xs: &[T], _ys: &[U]) -> usize {
    xs.len()
}

// --- manual specialization ---
#[inline(never)]
pub fn combine_u8_vecu8(xs: &[u8], ys: &[Vec<u8>]) -> usize {
    let mut total = xs.len();
    for v in ys { total += v.len(); }
    total
}

#[inline(never)]
pub fn combine_i32_veci32(xs: &[i32], ys: &[Vec<i32>]) -> usize {
    let mut total = xs.len();
    for v in ys { total += v.len(); }
    total
}
