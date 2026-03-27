// Example 17 — WITH macros: Lifetime specialization (static vs borrowed)
use spec_trait_macro::{spec, when};

pub struct Parser;

pub trait ParseTrait<T> {
    fn parse_len(&self, data: T) -> usize;
}

impl<T> ParseTrait<T> for Parser {
    #[inline(never)]
    fn parse_len(&self, _data: T) -> usize {
        0
    }
}

#[when(T = &'static _)]
impl<T> ParseTrait<T> for Parser {
    #[inline(never)]
    fn parse_len(&self, _data: T) -> usize {
        // Static data: can assume stable pointer, use len directly
        // (simulated by returning a sentinel)
        42
    }
}

#[when(all(T = &_, T: 'a))]
impl<'a, T> ParseTrait<T> for Parser {
    #[inline(never)]
    fn parse_len(&self, _data: T) -> usize {
        // Borrowed data: generic reference handling
        1
    }
}

pub fn parse_static_str(p: &Parser, data: &'static str) -> usize {
    spec! { p.parse_len(data); Parser; [&'static str] }
}

pub fn parse_borrowed_str<'a>(p: &Parser, data: &'a str) -> usize {
    spec! { p.parse_len(data); Parser; [&'a str] }
}

pub fn parse_static_bytes(p: &Parser, data: &'static [u8]) -> usize {
    spec! { p.parse_len(data); Parser; [&'static [u8]] }
}

pub fn parse_borrowed_bytes<'a>(p: &Parser, data: &'a [u8]) -> usize {
    spec! { p.parse_len(data); Parser; [&'a [u8]] }
}

pub fn parse_generic<T>(p: &Parser, data: T) -> usize {
    ParseTrait::parse_len(p, data)
}

// Benchmarkable: process a slice of static strings
#[inline(never)]
pub fn sum_parse_static(p: &Parser, items: &[&'static str]) -> usize {
    let mut total = 0usize;
    for &s in items {
        total = total.wrapping_add(spec! { p.parse_len(s); Parser; [&'static str] });
    }
    total
}

#[inline(never)]
pub fn sum_parse_borrowed<'a>(p: &Parser, items: &[&'a str]) -> usize {
    let mut total = 0usize;
    for &s in items {
        total = total.wrapping_add(spec! { p.parse_len(s); Parser; [&'a str] });
    }
    total
}

#[inline(never)]
pub fn sum_parse_generic<T: Copy>(p: &Parser, items: &[T]) -> usize {
    let mut total = 0usize;
    for &item in items {
        total = total.wrapping_add(ParseTrait::parse_len(p, item));
    }
    total
}
