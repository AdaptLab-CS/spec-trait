// Example 35 — WITH macros: Two-generic concrete pair specialization
use spec_trait_macro::{spec, when};

pub struct Merger;

pub trait MergeTrait<T, U> {
    fn merge_count(&self, ts: &[T], us: &[U]) -> usize;
}

impl<T, U> MergeTrait<T, U> for Merger {
    #[inline(never)]
    fn merge_count(&self, ts: &[T], us: &[U]) -> usize {
        // Generic: sum lengths
        ts.len() + us.len()
    }
}

#[when(T = i32)]
impl<T, U> MergeTrait<T, U> for Merger {
    #[inline(never)]
    fn merge_count(&self, ts: &[T], us: &[U]) -> usize {
        // T=i32: weighted left
        ts.len() * 4 + us.len()
    }
}

#[when(U = u8)]
impl<T, U> MergeTrait<T, U> for Merger {
    #[inline(never)]
    fn merge_count(&self, ts: &[T], us: &[U]) -> usize {
        // U=u8: weighted right
        ts.len() + us.len() * 2
    }
}

#[when(all(T = i32, U = u8))]
impl<T, U> MergeTrait<T, U> for Merger {
    #[inline(never)]
    fn merge_count(&self, ts: &[T], us: &[U]) -> usize {
        // Both specialized: real work
        let mut total = 0usize;
        for &t in ts {
            total = total.wrapping_add(t as usize);
        }
        for &u in us {
            total = total.wrapping_add(u as usize);
        }
        total
    }
}

pub fn merge_i32_u8(m: &Merger, ts: &[i32], us: &[u8]) -> usize {
    spec! { m.merge_count(ts, us); Merger; [&[i32], &[u8]] }
}

pub fn merge_i32_f64(m: &Merger, ts: &[i32], us: &[f64]) -> usize {
    spec! { m.merge_count(ts, us); Merger; [&[i32], &[f64]] }
}

pub fn merge_f64_u8(m: &Merger, ts: &[f64], us: &[u8]) -> usize {
    spec! { m.merge_count(ts, us); Merger; [&[f64], &[u8]] }
}

pub fn merge_generic<T, U>(m: &Merger, ts: &[T], us: &[U]) -> usize {
    MergeTrait::merge_count(m, ts, us)
}

#[inline(never)]
pub fn sum_merge_i32_u8(m: &Merger, ts_chunks: &[&[i32]], us_chunks: &[&[u8]]) -> usize {
    let mut total = 0usize;
    for (ts, us) in ts_chunks.iter().zip(us_chunks.iter()) {
        total = total.wrapping_add(spec! { m.merge_count(*ts, *us); Merger; [&[i32], &[u8]] });
    }
    total
}

#[inline(never)]
pub fn sum_merge_generic<T, U>(m: &Merger, ts_chunks: &[&[T]], us_chunks: &[&[U]]) -> usize {
    let mut total = 0usize;
    for (ts, us) in ts_chunks.iter().zip(us_chunks.iter()) {
        total = total.wrapping_add(MergeTrait::merge_count(m, *ts, *us));
    }
    total
}
