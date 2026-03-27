// Example 23 — ENUM: Multiple self types (same trait across different receivers)


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

