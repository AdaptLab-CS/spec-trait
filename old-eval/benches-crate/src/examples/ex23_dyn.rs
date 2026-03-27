// Example 23 — DYN: Multiple self types (same trait across different receivers)


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

