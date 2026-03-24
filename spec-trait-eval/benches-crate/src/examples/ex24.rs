// Example 24 — WITH macros: Wildcard and complex type patterns
use spec_trait_macro::{spec, when};

pub struct Measurer;

pub trait MeasureTrait<T> {
    fn measure(&self, data: &T) -> usize;
}

impl<T> MeasureTrait<T> for Measurer {
    #[inline(never)]
    fn measure(&self, _data: &T) -> usize {
        std::mem::size_of::<T>()
    }
}

#[when(T = Vec<_>)]
impl<T> MeasureTrait<T> for Measurer {
    #[inline(never)]
    fn measure(&self, _data: &T) -> usize {
        // Wildcard Vec: any Vec, return capacity heuristic
        std::mem::size_of::<T>() + 24 // Vec header
    }
}

#[when(T = (i32, _))]
impl<T> MeasureTrait<T> for Measurer {
    #[inline(never)]
    fn measure(&self, _data: &T) -> usize {
        // Tuple with i32 first: known layout
        4 + std::mem::size_of::<T>() - 4
    }
}

#[when(T = &[i32])]
impl<T> MeasureTrait<T> for Measurer {
    #[inline(never)]
    fn measure(&self, _data: &T) -> usize {
        // Slice of i32
        16 // fat pointer
    }
}

#[when(T = fn(&u8) -> &u8)]
impl<T> MeasureTrait<T> for Measurer {
    #[inline(never)]
    fn measure(&self, _data: &T) -> usize {
        // Function pointer
        8
    }
}

pub fn measure_vec_i32(m: &Measurer, data: &Vec<i32>) -> usize {
    spec! { m.measure(data); Measurer; [Vec<i32>] }
}

pub fn measure_vec_u8(m: &Measurer, data: &Vec<u8>) -> usize {
    spec! { m.measure(data); Measurer; [Vec<u8>] }
}

pub fn measure_tuple(m: &Measurer, data: &(i32, f64)) -> usize {
    spec! { m.measure(data); Measurer; [(i32, f64)] }
}

pub fn measure_slice(m: &Measurer, data: &&[i32]) -> usize {
    spec! { m.measure(data); Measurer; [&[i32]] }
}

pub fn measure_fn_ptr(m: &Measurer, data: &fn(&u8) -> &u8) -> usize {
    spec! { m.measure(data); Measurer; [fn(&u8) -> &u8] }
}

pub fn measure_generic<T>(m: &Measurer, data: &T) -> usize {
    MeasureTrait::measure(m, data)
}

// Bulk benchmarkable functions
#[inline(never)]
pub fn sum_measure_vec_i32(m: &Measurer, items: &[Vec<i32>]) -> usize {
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(spec! { m.measure(item); Measurer; [Vec<i32>] });
    }
    total
}

#[inline(never)]
pub fn sum_measure_generic<T>(m: &Measurer, items: &[T]) -> usize {
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(MeasureTrait::measure(m, item));
    }
    total
}
