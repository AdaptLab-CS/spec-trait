// Example 07 — DYN: Map / transform


// --- dyn dispatch ---
pub trait DynMap {
    fn map_double_dyn(&self) -> Vec<u8>; // erased
}

