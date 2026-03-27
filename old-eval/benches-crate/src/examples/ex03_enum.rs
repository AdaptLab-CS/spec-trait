// Example 03 — ENUM: Hash computation
use std::hash::{Hash, Hasher};


// --- enum dispatch ---
pub enum HashEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
}
#[inline(never)]
pub fn hash_enum_all(variant: &HashEnum<'_>) -> u64 {
    match variant {
        HashEnum::U8(d) => {
            let mut h: u64 = 0xcbf29ce484222325;
            for &b in *d { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); }
            h
        }
        HashEnum::I32(d) => {
            let mut h: u64 = 0xcbf29ce484222325;
            for &v in *d {
                for b in v.to_le_bytes() { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); }
            }
            h
        }
    }
}

