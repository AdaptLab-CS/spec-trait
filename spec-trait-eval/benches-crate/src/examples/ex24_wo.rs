// Example 24 — WITHOUT macros: Wildcard and complex type patterns

// --- enum dispatch ---
pub enum MeasureEnum<'a> {
    VecI32(&'a Vec<i32>),
    VecU8(&'a Vec<u8>),
    Tuple(&'a (i32, f64)),
    Slice(&'a [i32]),
}

#[inline(never)]
pub fn measure_enum(variant: &MeasureEnum<'_>) -> usize {
    match variant {
        MeasureEnum::VecI32(_) => std::mem::size_of::<Vec<i32>>() + 24,
        MeasureEnum::VecU8(_) => std::mem::size_of::<Vec<u8>>() + 24,
        MeasureEnum::Tuple(_) => 4 + std::mem::size_of::<(i32, f64)>() - 4,
        MeasureEnum::Slice(_) => 16,
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn measure_generic<T>(_data: &T) -> usize {
    std::mem::size_of::<T>()
}

// --- manual specialization ---
#[inline(never)]
pub fn measure_vec_i32(_data: &Vec<i32>) -> usize {
    std::mem::size_of::<Vec<i32>>() + 24
}

#[inline(never)]
pub fn measure_vec_u8(_data: &Vec<u8>) -> usize {
    std::mem::size_of::<Vec<u8>>() + 24
}

// Bulk benchmarkable functions
#[inline(never)]
pub fn sum_measure_enum_vec_i32(items: &[Vec<i32>]) -> usize {
    let mut total = 0usize;
    for item in items {
        let e = MeasureEnum::VecI32(item);
        total = total.wrapping_add(measure_enum(&e));
    }
    total
}

#[inline(never)]
pub fn sum_measure_generic(items: &[Vec<i32>]) -> usize {
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(measure_generic(item));
    }
    total
}

#[inline(never)]
pub fn sum_measure_manual(items: &[Vec<i32>]) -> usize {
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(measure_vec_i32(item));
    }
    total
}
