// Legacy compatibility facade for non-spec variants.
// Prefer importing ex25_{dyn,enum,generic,runtime_typeid} directly.

pub use super::ex25_dyn::*;
pub use super::ex25_enum::*;
pub use super::ex25_generic::*;
pub use super::ex25_runtime_typeid::*;

#[inline(never)]
pub fn sum_render_enum(data: &[i32]) -> usize {
	let mut total = 0usize;
	for item in data {
		total = total.wrapping_add(super::ex25_enum::render_enum_i32(&super::ex25_enum::RenderEnum::I32, item));
	}
	total
}

#[inline(never)]
pub fn sum_render_runtime(data: &[i32]) -> usize {
	let mut total = 0usize;
	for item in data {
		total = total.wrapping_add(super::ex25_runtime_typeid::render_runtime_i32(item));
	}
	total
}

#[inline(never)]
pub fn sum_render_dyn(data: &[i32]) -> usize {
	let mut total = 0usize;
	let d = super::ex25_dyn::DynI32Renderer;
	for item in data {
		total = total.wrapping_add(super::ex25_dyn::DynRender::render_dyn(&d, item));
	}
	total
}

#[inline(never)]
pub fn sum_render_manual(data: &[i32]) -> usize {
	let mut total = 0usize;
	for item in data {
		total = total.wrapping_add(super::ex25_generic::render_manual_i32(item));
	}
	total
}
