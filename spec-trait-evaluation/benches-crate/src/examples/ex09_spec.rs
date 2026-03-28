// Ex09 -- spec: format_collect — 'static-aware reference specialization
//
// Pattern: #[when(T: 'static)]  (parsed as T = &'static _)
// This lifetime bound cannot be expressed with enum dispatch, TypeId dispatch, or naive generics.
// Only the spec-trait #[when] macro can dispatch based on lifetime bounds.
use spec_trait_macro::{spec, when};

pub struct Formatter;

pub trait FormatCollectTrait<T> {
    fn format_collect(&self, data: &[T]) -> usize;
}

impl<T> FormatCollectTrait<T> for Formatter {
    #[inline(never)]
    fn format_collect(&self, data: &[T]) -> usize {
        // Default: just count elements (cannot inspect T)
        data.len()
    }
}

#[when(T: 'static)]
impl<T> FormatCollectTrait<T> for Formatter {
    #[inline(never)]
    fn format_collect(&self, data: &[T]) -> usize {
        // Specialized for &'static _: dereference and accumulate pointer values
        // Since T = &'static Something, we know pointers are stable
        let mut acc: usize = 0;
        for item in data {
            acc = acc.wrapping_add(item as *const T as usize);
        }
        acc
    }
}

pub fn format_collect_i32(f: &Formatter, data: &[i32]) -> usize {
    spec! { f.format_collect(data); Formatter; [&[i32]]; &i32: 'static }
}
