// Example 23 — WITH macros: Multiple self types (same trait across different receivers)
use spec_trait_macro::{spec, when};

pub struct TypeA;
pub struct TypeB;

pub trait TransformTrait<T> {
    fn transform(&self, x: T) -> i64;
}

// Default impls
impl<T> TransformTrait<T> for TypeA {
    #[inline(never)]
    fn transform(&self, _x: T) -> i64 { 0 }
}

impl<T> TransformTrait<T> for TypeB {
    #[inline(never)]
    fn transform(&self, _x: T) -> i64 { 0 }
}

impl<T, U> TransformTrait<T> for Vec<U> {
    #[inline(never)]
    fn transform(&self, _x: T) -> i64 { self.len() as i64 }
}

impl<T> TransformTrait<T> for i32 {
    #[inline(never)]
    fn transform(&self, _x: T) -> i64 { *self as i64 }
}

// Specializations for TypeA
#[when(T = u8)]
impl<T> TransformTrait<T> for TypeA {
    #[inline(never)]
    fn transform(&self, _x: T) -> i64 { 1 }
}

#[when(T = i32)]
impl<T> TransformTrait<T> for TypeA {
    #[inline(never)]
    fn transform(&self, _x: T) -> i64 { 2 }
}

// Specializations for TypeB
#[when(T = u8)]
impl<T> TransformTrait<T> for TypeB {
    #[inline(never)]
    fn transform(&self, _x: T) -> i64 { 10 }
}

// Specializations for Vec<U>
#[when(T = u8)]
impl<T, U> TransformTrait<T> for Vec<U> {
    #[inline(never)]
    fn transform(&self, _x: T) -> i64 { (self.len() * 2) as i64 }
}

// Specializations for i32
#[when(T = u8)]
impl<T> TransformTrait<T> for i32 {
    #[inline(never)]
    fn transform(&self, _x: T) -> i64 { (*self as i64) * 3 }
}

pub fn transform_a_u8(a: &TypeA, x: u8) -> i64 {
    spec! { a.transform(x); TypeA; [u8] }
}

pub fn transform_a_i32(a: &TypeA, x: i32) -> i64 {
    spec! { a.transform(x); TypeA; [i32] }
}

pub fn transform_b_u8(b: &TypeB, x: u8) -> i64 {
    spec! { b.transform(x); TypeB; [u8] }
}

pub fn transform_vec_u8(v: &Vec<i32>, x: u8) -> i64 {
    spec! { v.transform(x); Vec<i32>; [u8] }
}

pub fn transform_i32_u8(n: &i32, x: u8) -> i64 {
    spec! { n.transform(x); i32; [u8] }
}

pub fn transform_generic_a<T>(a: &TypeA, x: T) -> i64 {
    TransformTrait::transform(a, x)
}

pub fn transform_generic_b<T>(b: &TypeB, x: T) -> i64 {
    TransformTrait::transform(b, x)
}

// Bulk benchmarkable
#[inline(never)]
pub fn sum_transform_a_u8(a: &TypeA, data: &[u8]) -> i64 {
    let mut total = 0i64;
    for &x in data {
        total = total.wrapping_add(spec! { a.transform(x); TypeA; [u8] });
    }
    total
}

#[inline(never)]
pub fn sum_transform_b_u8(b: &TypeB, data: &[u8]) -> i64 {
    let mut total = 0i64;
    for &x in data {
        total = total.wrapping_add(spec! { b.transform(x); TypeB; [u8] });
    }
    total
}

#[inline(never)]
pub fn sum_transform_generic_a<T: Copy>(a: &TypeA, data: &[T]) -> i64 {
    let mut total = 0i64;
    for &x in data {
        total = total.wrapping_add(TransformTrait::transform(a, x));
    }
    total
}
