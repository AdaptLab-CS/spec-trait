// Example 27 — DYN: Many specializations (8+ types)


// --- Approach E: dyn dispatch ---
pub trait DynEncode {
    fn encode_dyn(&self) -> usize;
}

pub struct DynU8Encoder<'a>(pub &'a [u8]);
impl DynEncode for DynU8Encoder<'_> {
    #[inline(never)]
    fn encode_dyn(&self) -> usize {
        self.0.len()
    }
}

pub struct DynI32Encoder<'a>(pub &'a [i32]);
impl DynEncode for DynI32Encoder<'_> {
    #[inline(never)]
    fn encode_dyn(&self) -> usize {
        let mut total = 0usize;
        for &v in self.0 {
            let uv = v as u32 as usize;
            total += if uv < 128 { 1 } else if uv < 16384 { 2 } else { 4 };
        }
        total
    }
}

