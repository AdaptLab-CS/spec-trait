// Example 24 — ENUM: T: 'static lifetime bound


// --- Approach A: enum dispatch ---
pub enum HashEnum {
    StaticI32,
    Generic,
}

#[inline(never)]
pub fn hash_enum_i32(variant: &HashEnum, data: &i32) -> u64 {
    match variant {
        HashEnum::StaticI32 => {
            let ptr = data as *const i32 as usize;
            (ptr as u64).wrapping_mul(0x517cc1b727220a95)
        }
        HashEnum::Generic => std::mem::size_of::<i32>() as u64,
    }
}

