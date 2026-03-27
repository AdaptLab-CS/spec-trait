// Example 21 — WITH macros: Conditional combinators (any, all, not)
use spec_trait_macro::{spec, when};

pub struct Processor;

pub trait ProcessTrait<T> {
    fn process(&self, data: &[T]) -> usize;
}

impl<T> ProcessTrait<T> for Processor {
    #[inline(never)]
    fn process(&self, data: &[T]) -> usize {
        data.len()
    }
}

#[when(any(T = i32, T = i64))]
impl<T> ProcessTrait<T> for Processor {
    #[inline(never)]
    fn process(&self, data: &[T]) -> usize {
        // Integers: sum the bytes
        data.len() * std::mem::size_of::<T>()
    }
}

#[when(not(T = u8))]
impl<T> ProcessTrait<T> for Processor {
    #[inline(never)]
    fn process(&self, data: &[T]) -> usize {
        // Everything except u8: use size_of
        data.len() * std::mem::size_of::<T>()
    }
}

#[when(all(T = Vec<U>, U: Copy))]
impl<T, U> ProcessTrait<T> for Processor {
    #[inline(never)]
    fn process(&self, data: &[T]) -> usize {
        // Vec of Copy types: count total inner elements
        let mut total = 0;
        for v in data {
            total += v.len();
        }
        total
    }
}

pub fn process_i32(p: &Processor, data: &[i32]) -> usize {
    spec! { p.process(data); Processor; [&[i32]] }
}

pub fn process_i64(p: &Processor, data: &[i64]) -> usize {
    spec! { p.process(data); Processor; [&[i64]] }
}

pub fn process_u8(p: &Processor, data: &[u8]) -> usize {
    spec! { p.process(data); Processor; [&[u8]] }
}

pub fn process_f64(p: &Processor, data: &[f64]) -> usize {
    spec! { p.process(data); Processor; [&[f64]] }
}

pub fn process_vec_i32(p: &Processor, data: &[Vec<i32>]) -> usize {
    spec! { p.process(data); Processor; [&[Vec<i32>]]; i32: Copy }
}

pub fn process_generic<T>(p: &Processor, data: &[T]) -> usize {
    ProcessTrait::process(p, data)
}
