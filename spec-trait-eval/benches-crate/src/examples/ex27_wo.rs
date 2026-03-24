// Example 27 — WITHOUT macros: Multiple self types (same trait across different receivers)

// --- enum dispatch ---
pub enum ReceiverEnum {
    TypeA,
    TypeB,
    VecI32(usize),  // length
    I32Val(i32),
}

#[inline(never)]
pub fn transform_enum_u8(receiver: &ReceiverEnum) -> i64 {
    match receiver {
        ReceiverEnum::TypeA => 1,
        ReceiverEnum::TypeB => 10,
        ReceiverEnum::VecI32(len) => (*len as i64) * 2,
        ReceiverEnum::I32Val(n) => (*n as i64) * 3,
    }
}

#[inline(never)]
pub fn transform_enum_generic(receiver: &ReceiverEnum) -> i64 {
    match receiver {
        ReceiverEnum::TypeA => 0,
        ReceiverEnum::TypeB => 0,
        ReceiverEnum::VecI32(len) => *len as i64,
        ReceiverEnum::I32Val(n) => *n as i64,
    }
}

// --- manual specialization ---
#[inline(never)]
pub fn transform_a_u8() -> i64 { 1 }

#[inline(never)]
pub fn transform_a_i32() -> i64 { 2 }

#[inline(never)]
pub fn transform_b_u8() -> i64 { 10 }

#[inline(never)]
pub fn transform_generic() -> i64 { 0 }

// --- dyn dispatch ---
pub trait DynTransformU8 {
    fn transform_dyn(&self) -> i64;
}

pub struct DynTypeA;
impl DynTransformU8 for DynTypeA {
    #[inline(never)]
    fn transform_dyn(&self) -> i64 { 1 }
}

pub struct DynTypeB;
impl DynTransformU8 for DynTypeB {
    #[inline(never)]
    fn transform_dyn(&self) -> i64 { 10 }
}

// Bulk benchmarkable
#[inline(never)]
pub fn sum_transform_enum_u8(data: &[u8]) -> i64 {
    let receiver = ReceiverEnum::TypeA;
    let mut total = 0i64;
    for _ in data {
        total = total.wrapping_add(transform_enum_u8(&receiver));
    }
    total
}

#[inline(never)]
pub fn sum_transform_manual_u8(data: &[u8]) -> i64 {
    let mut total = 0i64;
    for _ in data {
        total = total.wrapping_add(transform_a_u8());
    }
    total
}

#[inline(never)]
pub fn sum_transform_dyn(data: &[u8]) -> i64 {
    let d: &dyn DynTransformU8 = &DynTypeA;
    let mut total = 0i64;
    for _ in data {
        total = total.wrapping_add(d.transform_dyn());
    }
    total
}

#[inline(never)]
pub fn sum_transform_generic(data: &[u8]) -> i64 {
    let mut total = 0i64;
    for _ in data {
        total = total.wrapping_add(transform_generic());
    }
    total
}
