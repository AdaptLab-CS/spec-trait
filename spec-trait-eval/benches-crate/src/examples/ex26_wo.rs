// Example 26 — WITHOUT macros: Fn trait bound specialization

// --- enum dispatch ---
pub enum DispatchEnum {
    FnI32(fn(i32) -> i32),
    FnRefI32(fn(&i32) -> i32),
    FnI32I32(fn(i32, i32) -> i32),
}

#[inline(never)]
pub fn dispatch_enum(variant: &DispatchEnum) -> i64 {
    match variant {
        DispatchEnum::FnI32(f) => f(42) as i64,
        DispatchEnum::FnRefI32(f) => { let v = 42i32; f(&v) as i64 },
        DispatchEnum::FnI32I32(f) => f(21, 21) as i64,
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn dispatch_generic<T>(_x: T) -> i64 {
    0
}

// --- manual specialization ---
#[inline(never)]
pub fn dispatch_fn_i32(f: fn(i32) -> i32) -> i64 {
    f(42) as i64
}

#[inline(never)]
pub fn dispatch_fn_ref_i32(f: fn(&i32) -> i32) -> i64 {
    let v = 42i32;
    f(&v) as i64
}

#[inline(never)]
pub fn dispatch_fn_i32_i32(f: fn(i32, i32) -> i32) -> i64 {
    f(21, 21) as i64
}

// --- dyn dispatch ---
pub trait DynDispatch {
    fn dispatch_dyn(&self) -> i64;
}

pub struct DynFnI32(pub fn(i32) -> i32);
impl DynDispatch for DynFnI32 {
    #[inline(never)]
    fn dispatch_dyn(&self) -> i64 { self.0(42) as i64 }
}

pub struct DynFnRefI32(pub fn(&i32) -> i32);
impl DynDispatch for DynFnRefI32 {
    #[inline(never)]
    fn dispatch_dyn(&self) -> i64 { let v = 42i32; self.0(&v) as i64 }
}

// Bulk benchmarkable
#[inline(never)]
pub fn sum_dispatch_enum(fns: &[fn(i32) -> i32]) -> i64 {
    let mut total = 0i64;
    for &f in fns {
        let e = DispatchEnum::FnI32(f);
        total = total.wrapping_add(dispatch_enum(&e));
    }
    total
}

#[inline(never)]
pub fn sum_dispatch_manual(fns: &[fn(i32) -> i32]) -> i64 {
    let mut total = 0i64;
    for &f in fns {
        total = total.wrapping_add(dispatch_fn_i32(f));
    }
    total
}

#[inline(never)]
pub fn sum_dispatch_dyn(fns: &[fn(i32) -> i32]) -> i64 {
    let mut total = 0i64;
    for &f in fns {
        let d = DynFnI32(f);
        total = total.wrapping_add(d.dispatch_dyn());
    }
    total
}
