// Example 29 — WITHOUT macros: &dyn Trait type pattern specialization
use std::any::TypeId;

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

// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn render_runtime_i32(_data: &i32) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        4
    } else {
        std::mem::size_of::<i32>()
    }
}

// --- Approach C: dyn dispatch ---
pub trait DynRender {
    fn render_dyn(&self, data: &i32) -> usize;
}

pub struct DynI32Renderer;
impl DynRender for DynI32Renderer {
    #[inline(never)]
    fn render_dyn(&self, _data: &i32) -> usize {
        4
    }
}

// --- Approach D: generic fallback ---
#[inline(never)]
pub fn render_generic_i32(_data: &i32) -> usize {
    std::mem::size_of::<i32>()
}

// --- Approach E: manual specialization ---
#[inline(never)]
pub fn render_manual_i32(_data: &i32) -> usize {
    4
}

// --- Bulk functions ---
#[inline(never)]
pub fn sum_render_enum(items: &[i32]) -> usize {
    let e = RenderEnum::I32;
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(render_enum_i32(&e, item));
    }
    total
}

#[inline(never)]
pub fn sum_render_runtime(items: &[i32]) -> usize {
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(render_runtime_i32(item));
    }
    total
}

#[inline(never)]
pub fn sum_render_dyn(items: &[i32]) -> usize {
    let d: &dyn DynRender = &DynI32Renderer;
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(d.render_dyn(item));
    }
    total
}

#[inline(never)]
pub fn sum_render_generic(items: &[i32]) -> usize {
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(render_generic_i32(item));
    }
    total
}

#[inline(never)]
pub fn sum_render_manual(items: &[i32]) -> usize {
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(render_manual_i32(item));
    }
    total
}
