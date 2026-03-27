// Example 36 — WITH macros: Triple trait bounds (Clone + Eq + Ord)
use spec_trait_macro::{spec, when};

pub struct UniqueSorter;

pub trait UniqueSortTrait<T> {
    fn sorted_unique_count(&self, data: &[T]) -> usize;
}

impl<T: PartialEq> UniqueSortTrait<T> for UniqueSorter {
    #[inline(never)]
    fn sorted_unique_count(&self, data: &[T]) -> usize {
        // Generic PartialEq: O(n²) naive unique count
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
}

#[when(T: Ord)]
impl<T> UniqueSortTrait<T> for UniqueSorter {
    #[inline(never)]
    fn sorted_unique_count(&self, data: &[T]) -> usize {
        // Ord: sorted dedup pass, count steps
        if data.is_empty() {
            return 0;
        }
        let mut count = 1usize;
        for i in 1..data.len() {
            if data[i] != data[i - 1] {
                count += 1;
            }
        }
        count
    }
}

#[when(T: Clone + Eq + Ord)]
impl<T> UniqueSortTrait<T> for UniqueSorter {
    #[inline(never)]
    fn sorted_unique_count(&self, data: &[T]) -> usize {
        // Clone + Eq + Ord: sort a clone, dedup, count
        let mut buf: Vec<T> = data.to_vec();
        buf.sort();
        buf.dedup();
        buf.len()
    }
}

#[when(T = i32)]
impl<T> UniqueSortTrait<T> for UniqueSorter {
    #[inline(never)]
    fn sorted_unique_count(&self, data: &[T]) -> usize {
        // i32: native sort + dedup
        let mut buf: Vec<i32> = data.to_vec();
        buf.sort_unstable();
        buf.dedup();
        buf.len()
    }
}

pub fn sorted_unique_count_i32(s: &UniqueSorter, data: &[i32]) -> usize {
    spec! { s.sorted_unique_count(data); UniqueSorter; [&[i32]]; i32: Clone + Eq + Ord }
}

pub fn sorted_unique_count_u8(s: &UniqueSorter, data: &[u8]) -> usize {
    spec! { s.sorted_unique_count(data); UniqueSorter; [&[u8]]; u8: Clone + Eq + Ord }
}

pub fn sorted_unique_count_generic<T: PartialEq>(s: &UniqueSorter, data: &[T]) -> usize {
    UniqueSortTrait::sorted_unique_count(s, data)
}

#[inline(never)]
pub fn sum_sorted_i32(s: &UniqueSorter, chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { s.sorted_unique_count(*chunk); UniqueSorter; [&[i32]]; i32: Clone + Eq + Ord });
    }
    total
}

#[inline(never)]
pub fn sum_sorted_generic<T: PartialEq>(s: &UniqueSorter, chunks: &[&[T]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(UniqueSortTrait::sorted_unique_count(s, *chunk));
    }
    total
}
