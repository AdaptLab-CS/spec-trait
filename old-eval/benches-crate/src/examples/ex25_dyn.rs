// Example 25 — DYN: &dyn Trait type pattern specialization


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

