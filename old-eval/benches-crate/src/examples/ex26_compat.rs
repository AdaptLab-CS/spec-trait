// Legacy compatibility facade for non-spec variants.
// Prefer importing ex26_{dyn,enum,generic,runtime_typeid} directly.

pub use super::ex26_enum::*;
pub use super::ex26_generic::*;
pub use super::ex26_runtime_typeid::*;

#[inline(never)]
pub fn sum_dedup_enum_u8(chunks: &[&[u8]]) -> usize {
	let mut total = 0usize;
	for chunk in chunks {
		let e = super::ex26_enum::DedupEnum::U8(chunk);
		total = total.wrapping_add(super::ex26_enum::dedup_enum(&e));
	}
	total
}

#[inline(never)]
pub fn sum_dedup_runtime_u8(chunks: &[&[u8]]) -> usize {
	let mut total = 0usize;
	for chunk in chunks {
		total = total.wrapping_add(super::ex26_runtime_typeid::dedup_runtime_u8(chunk));
	}
	total
}

#[inline(never)]
pub fn sum_dedup_manual_u8(chunks: &[&[u8]]) -> usize {
	let mut total = 0usize;
	for chunk in chunks {
		total = total.wrapping_add(super::ex26_generic::dedup_manual_u8(chunk));
	}
	total
}
