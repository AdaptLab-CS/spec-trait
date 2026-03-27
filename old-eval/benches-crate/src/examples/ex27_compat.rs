// Legacy compatibility facade for non-spec variants.
// Prefer importing ex27_{dyn,enum,generic,runtime_typeid} directly.

pub use super::ex27_dyn::*;
pub use super::ex27_enum::*;
pub use super::ex27_generic::*;
pub use super::ex27_runtime_typeid::*;

#[inline(never)]
pub fn sum_encode_enum_i32(chunks: &[&[i32]]) -> usize {
	let mut total = 0usize;
	for chunk in chunks {
		let e = super::ex27_enum::EncodeEnum::I32(chunk);
		total = total.wrapping_add(super::ex27_enum::encode_enum(&e));
	}
	total
}

#[inline(never)]
pub fn sum_encode_runtime_i32(chunks: &[&[i32]]) -> usize {
	let mut total = 0usize;
	for chunk in chunks {
		total = total.wrapping_add(super::ex27_runtime_typeid::encode_runtime_i32(chunk));
	}
	total
}

#[inline(never)]
pub fn sum_encode_manual_i32(chunks: &[&[i32]]) -> usize {
	let mut total = 0usize;
	for chunk in chunks {
		total = total.wrapping_add(super::ex27_generic::encode_manual_i32(chunk));
	}
	total
}

#[inline(never)]
pub fn sum_encode_dyn_i32(chunks: &[&[i32]]) -> usize {
	let mut total = 0usize;
	for chunk in chunks {
		let d = super::ex27_dyn::DynI32Encoder(chunk);
		total = total.wrapping_add(super::ex27_dyn::DynEncode::encode_dyn(&d));
	}
	total
}
