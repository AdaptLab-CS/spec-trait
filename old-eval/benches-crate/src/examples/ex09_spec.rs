// Example 09 — WITH macros: Display-based formatting
use spec_trait_macro::{spec, when};

pub struct Formatter;

pub trait FmtTrait<T> {
    fn fmt_all(&self, items: &[T]) -> String;
}

impl<T: std::fmt::Debug> FmtTrait<T> for Formatter {
    #[inline(never)]
    fn fmt_all(&self, items: &[T]) -> String {
        items.iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(", ")
    }
}

#[when(T = i32)]
impl<T> FmtTrait<T> for Formatter {
    #[inline(never)]
    fn fmt_all(&self, items: &[T]) -> String {
        let mut buf = itoa::Buffer::new();
        items.iter()
            .map(|x| buf.format(*x).to_owned())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[when(T = f64)]
impl<T> FmtTrait<T> for Formatter {
    #[inline(never)]
    fn fmt_all(&self, items: &[T]) -> String {
        let mut buf = ryu::Buffer::new();
        items.iter()
            .map(|x| buf.format(*x).to_owned())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[when(T = String)]
impl<T> FmtTrait<T> for Formatter {
    #[inline(never)]
    fn fmt_all(&self, items: &[T]) -> String {
        items.join(", ")
    }
}

pub fn fmt_i32(f: &Formatter, items: &[i32]) -> String {
    spec! { f.fmt_all(items); Formatter; [&[i32]] }
}

pub fn fmt_f64(f: &Formatter, items: &[f64]) -> String {
    spec! { f.fmt_all(items); Formatter; [&[f64]] }
}

pub fn fmt_string(f: &Formatter, items: &[String]) -> String {
    spec! { f.fmt_all(items); Formatter; [&[String]] }
}

pub fn fmt_generic<T: std::fmt::Debug>(f: &Formatter, items: &[T]) -> String {
    FmtTrait::fmt_all(f, items)
}
