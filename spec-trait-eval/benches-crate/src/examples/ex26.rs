// Example 26 — WITH macros: Fn trait bound specialization
use spec_trait_macro::{spec, when};

pub struct Dispatcher;

pub trait DispatchTrait<T> {
    fn dispatch(&self, x: T) -> i64;
}

impl<T> DispatchTrait<T> for Dispatcher {
    #[inline(never)]
    fn dispatch(&self, _x: T) -> i64 {
        0 // default: unknown type
    }
}

#[when(T: Fn(i32) -> i32)]
impl<T> DispatchTrait<T> for Dispatcher {
    #[inline(never)]
    fn dispatch(&self, x: T) -> i64 {
        x(42) as i64
    }
}

#[when(T: for<'a> Fn(&'a i32) -> i32)]
impl<T> DispatchTrait<T> for Dispatcher {
    #[inline(never)]
    fn dispatch(&self, x: T) -> i64 {
        let val = 42i32;
        x(&val) as i64
    }
}

#[when(T: Fn(i32, i32) -> i32)]
impl<T> DispatchTrait<T> for Dispatcher {
    #[inline(never)]
    fn dispatch(&self, x: T) -> i64 {
        x(21, 21) as i64
    }
}

pub fn dispatch_fn_i32(d: &Dispatcher, f: fn(i32) -> i32) -> i64 {
    spec! { d.dispatch(f); Dispatcher; [fn(i32) -> i32]; fn(i32) -> i32: Fn(i32) -> i32 }
}

pub fn dispatch_fn_ref_i32(d: &Dispatcher, f: fn(&i32) -> i32) -> i64 {
    spec! { d.dispatch(f); Dispatcher; [fn(&i32) -> i32]; fn(&i32) -> i32: for<'a> Fn(&'a i32) -> i32 }
}

pub fn dispatch_fn_i32_i32(d: &Dispatcher, f: fn(i32, i32) -> i32) -> i64 {
    spec! { d.dispatch(f); Dispatcher; [fn(i32, i32) -> i32]; fn(i32, i32) -> i32: Fn(i32, i32) -> i32 }
}

pub fn dispatch_generic<T>(d: &Dispatcher, x: T) -> i64 {
    DispatchTrait::dispatch(d, x)
}

// Bulk benchmarkable
#[inline(never)]
pub fn sum_dispatch_fn_i32(d: &Dispatcher, fns: &[fn(i32) -> i32]) -> i64 {
    let mut total = 0i64;
    for &f in fns {
        total = total.wrapping_add(spec! { d.dispatch(f); Dispatcher; [fn(i32) -> i32]; fn(i32) -> i32: Fn(i32) -> i32 });
    }
    total
}

#[inline(never)]
pub fn sum_dispatch_fn_ref_i32(d: &Dispatcher, fns: &[fn(&i32) -> i32]) -> i64 {
    let mut total = 0i64;
    for &f in fns {
        total = total.wrapping_add(spec! { d.dispatch(f); Dispatcher; [fn(&i32) -> i32]; fn(&i32) -> i32: for<'a> Fn(&'a i32) -> i32 });
    }
    total
}

#[inline(never)]
pub fn sum_dispatch_generic<T: Copy>(d: &Dispatcher, items: &[T]) -> i64 {
    let mut total = 0i64;
    for &item in items {
        total = total.wrapping_add(DispatchTrait::dispatch(d, item));
    }
    total
}
