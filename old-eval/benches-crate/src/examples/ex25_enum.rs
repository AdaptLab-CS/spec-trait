// Example 25 — ENUM: &dyn Trait type pattern specialization


// --- Approach A: enum dispatch ---
pub enum RenderEnum {
    I32,
    Str,
    DynDisplay,
    Generic,
}

#[inline(never)]
pub fn render_enum_i32(variant: &RenderEnum, _data: &i32) -> usize {
    match variant {
        RenderEnum::I32 => 4,
        _ => std::mem::size_of::<i32>(),
    }
}

