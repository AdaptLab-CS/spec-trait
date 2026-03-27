// Example 22 — DYN: Fn trait bound specialization


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

