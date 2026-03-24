// Example 2 — WITHOUT macros: Bitcount / popcount
use std::any::TypeId;

// --- dyn dispatch ---
pub trait DynBitCount {
    fn count_ones_dyn(&self) -> u64;
}
impl DynBitCount for Vec<u8> {
    #[inline(never)]
    fn count_ones_dyn(&self) -> u64 {
        self.iter().map(|b| b.count_ones() as u64).sum()
    }
}
impl DynBitCount for Vec<u32> {
    #[inline(never)]
    fn count_ones_dyn(&self) -> u64 {
        self.iter().map(|v| v.count_ones() as u64).sum()
    }
}
impl DynBitCount for Vec<u64> {
    #[inline(never)]
    fn count_ones_dyn(&self) -> u64 {
        self.iter().map(|v| v.count_ones() as u64).sum()
    }
}

pub fn count_dyn(data: &dyn DynBitCount) -> u64 {
    data.count_ones_dyn()
}

// --- enum dispatch ---
pub enum BitCountEnum<'a> {
    U8(&'a [u8]),
    U32(&'a [u32]),
    U64(&'a [u64]),
}

#[inline(never)]
pub fn bitcount_enum_count_ones(variant: &BitCountEnum<'_>) -> u64 {
    match variant {
        BitCountEnum::U8(d) => d.iter().map(|b| b.count_ones() as u64).sum(),
        BitCountEnum::U32(d) => d.iter().map(|v| v.count_ones() as u64).sum(),
        BitCountEnum::U64(d) => d.iter().map(|v| v.count_ones() as u64).sum(),
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn count_runtime_u8(data: &[u8]) -> u64 {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        data.iter().map(|b| b.count_ones() as u64).sum()
    } else {
        0
    }
}

#[inline(never)]
pub fn count_runtime_u32(data: &[u32]) -> u64 {
    if TypeId::of::<u32>() == TypeId::of::<u32>() {
        data.iter().map(|v| v.count_ones() as u64).sum()
    } else {
        0
    }
}

// --- generic (no specialization) ---
#[inline(never)]
pub fn count_generic<T>(_data: &[T]) -> u64 {
    0
}
