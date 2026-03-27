// Legacy compatibility facade for non-spec variants.
// Prefer importing ex21_{dyn,enum,generic,runtime_typeid} directly.

pub use super::ex21_enum::*;
pub use super::ex21_generic::*;
pub use super::ex21_runtime_typeid::*;

#[inline(never)]
pub fn process_manual_i32(data: &[i32]) -> usize {
	data.len() * std::mem::size_of::<i32>()
}
