// Example 24 — DYN: T: 'static lifetime bound


// --- Approach C: dyn dispatch ---
pub trait DynHash {
    fn hash_dyn(&self, data: &i32) -> u64;
}

pub struct DynStaticHasher;
impl DynHash for DynStaticHasher {
    #[inline(never)]
    fn hash_dyn(&self, data: &i32) -> u64 {
        let ptr = data as *const i32 as usize;
        (ptr as u64).wrapping_mul(0x517cc1b727220a95)
    }
}

