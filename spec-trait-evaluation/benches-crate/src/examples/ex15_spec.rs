// Ex15 -- spec: compute_sum — trait object specialization
//
// Pattern: #[when(T = &dyn Summable)]
// Specializing on trait object types (&dyn Tr) cannot be expressed with enum, TypeId,
// or naive dispatch. Only spec-trait can match on &dyn Trait types directly.
use spec_trait_macro::{spec, when};

pub trait Summable {
    fn value(&self) -> i64;
}

impl Summable for i32 {
    fn value(&self) -> i64 {
        *self as i64
    }
}

pub struct Computer;

pub trait ComputeSumTrait<T> {
    fn compute_sum(&self, items: &[T]) -> i64;
}

impl<T> ComputeSumTrait<T> for Computer {
    #[inline(never)]
    fn compute_sum(&self, items: &[T]) -> i64 {
        // Default: cannot sum unknown T, just return count
        items.len() as i64
    }
}

#[when(T = &dyn Summable)]
impl<T> ComputeSumTrait<T> for Computer {
    #[inline(never)]
    fn compute_sum(&self, items: &[T]) -> i64 {
        // Specialized: T = &dyn Summable — call value() through vtable
        let dyn_items = unsafe {
            std::slice::from_raw_parts(items.as_ptr() as *const &dyn Summable, items.len())
        };
        let mut acc: i64 = 0;
        for item in dyn_items {
            acc += item.value();
        }
        acc
    }
}

pub fn compute_sum_dyn(c: &Computer, items: &[&dyn Summable]) -> i64 {
    spec! { c.compute_sum(items); Computer; [&[&dyn Summable]] }
}
