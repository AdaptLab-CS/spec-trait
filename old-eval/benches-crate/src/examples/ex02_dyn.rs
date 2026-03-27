// Example 02 — DYN: Absolute value


// --- dyn dispatch ---
pub trait DynAbs {
    fn abs_dyn(&self) -> Vec<u8>; // erased to bytes
}

