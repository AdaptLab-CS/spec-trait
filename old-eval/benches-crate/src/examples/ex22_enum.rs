// Example 22 — ENUM: Fn trait bound specialization


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

