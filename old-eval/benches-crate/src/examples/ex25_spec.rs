// Example 25 — WITH macros: &dyn Trait type pattern specialization
use spec_trait_macro::{spec, when};
use std::fmt::Display;

pub struct Renderer;

pub trait RenderTrait<T> {
    fn render(&self, data: &T) -> usize;
}

impl<T> RenderTrait<T> for Renderer {
    #[inline(never)]
    fn render(&self, _data: &T) -> usize {
        // Generic: just return type size
        std::mem::size_of::<T>()
    }
}

#[when(T = &dyn Display)]
impl<T> RenderTrait<T> for Renderer {
    #[inline(never)]
    fn render(&self, _data: &T) -> usize {
        // Trait object: known fat pointer, 16 bytes
        16
    }
}

#[when(T = i32)]
impl<T> RenderTrait<T> for Renderer {
    #[inline(never)]
    fn render(&self, data: &T) -> usize {
        // Concrete i32: known fixed size
        4
    }
}

#[when(T = String)]
impl<T> RenderTrait<T> for Renderer {
    #[inline(never)]
    fn render(&self, data: &T) -> usize {
        // String: heap-allocated, 24 bytes on stack
        24
    }
}

pub fn render_dyn_display(r: &Renderer, data: &&dyn Display) -> usize {
    spec! { r.render(data); Renderer; [&dyn Display] }
}

pub fn render_i32(r: &Renderer, data: &i32) -> usize {
    spec! { r.render(data); Renderer; [&i32] }
}

pub fn render_string(r: &Renderer, data: &String) -> usize {
    spec! { r.render(data); Renderer; [&String] }
}

pub fn render_generic<T>(r: &Renderer, data: &T) -> usize {
    RenderTrait::render(r, data)
}

#[inline(never)]
pub fn sum_render_i32(r: &Renderer, items: &[i32]) -> usize {
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(spec! { r.render(item); Renderer; [&i32] });
    }
    total
}

#[inline(never)]
pub fn sum_render_generic<T>(r: &Renderer, items: &[T]) -> usize {
    let mut total = 0usize;
    for item in items {
        total = total.wrapping_add(RenderTrait::render(r, item));
    }
    total
}
