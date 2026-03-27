// Example 19 — WITH macros: Multi-generic repeating (zip/combine)
use spec_trait_macro::{spec, when};

pub struct Combiner;

pub trait CombineTrait<T, U> {
    fn combine(&self, xs: &[T], ys: &[U]) -> usize;
}

impl<T, U> CombineTrait<T, U> for Combiner {
    #[inline(never)]
    fn combine(&self, xs: &[T], _ys: &[U]) -> usize {
        // default: count xs
        xs.len()
    }
}

#[when(U = Vec<T>)]
impl<T, U> CombineTrait<T, U> for Combiner {
    #[inline(never)]
    fn combine(&self, xs: &[T], ys: &[U]) -> usize {
        // Specialized: U is Vec<T>, flatten and count total elements
        let mut total = xs.len();
        for v in ys {
            total += v.len();
        }
        total
    }
}

#[when(T = Vec<U>)]
impl<T, U> CombineTrait<T, U> for Combiner {
    #[inline(never)]
    fn combine(&self, xs: &[T], ys: &[U]) -> usize {
        // Specialized: T is Vec<U>, flatten xs and count
        let mut total = 0;
        for v in xs {
            total += v.len();
        }
        total + ys.len()
    }
}

#[when(all(U = Vec<T>, T = u8))]
impl<T, U> CombineTrait<T, U> for Combiner {
    #[inline(never)]
    fn combine(&self, xs: &[T], ys: &[U]) -> usize {
        // Most specialized: T=u8, U=Vec<u8>
        let mut total = xs.len();
        for v in ys {
            total += v.len();
        }
        total
    }
}

pub fn combine_u8_vecu8(c: &Combiner, xs: &[u8], ys: &[Vec<u8>]) -> usize {
    spec! { c.combine(xs, ys); Combiner; [&[u8], &[Vec<u8>]] }
}

pub fn combine_i32_veci32(c: &Combiner, xs: &[i32], ys: &[Vec<i32>]) -> usize {
    spec! { c.combine(xs, ys); Combiner; [&[i32], &[Vec<i32>]] }
}

pub fn combine_veci32_i32(c: &Combiner, xs: &[Vec<i32>], ys: &[i32]) -> usize {
    spec! { c.combine(xs, ys); Combiner; [&[Vec<i32>], &[i32]] }
}

pub fn combine_generic<T, U>(c: &Combiner, xs: &[T], ys: &[U]) -> usize {
    CombineTrait::combine(c, xs, ys)
}
