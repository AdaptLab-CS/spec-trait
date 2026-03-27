// Legacy compatibility facade for non-spec variants.
// Prefer importing ex20_{dyn,enum,generic,runtime_typeid} directly.

pub use super::ex20_enum::*;
pub use super::ex20_generic::*;

#[inline(never)]
pub fn sum_measure_enum_vec_i32(items: &[Vec<i32>]) -> usize {
	let mut total = 0usize;
	for item in items {
		let e = super::ex20_enum::MeasureEnum::VecI32(item);
		total = total.wrapping_add(super::ex20_enum::measure_enum(&e));
	}
	total
}

#[inline(never)]
pub fn sum_measure_manual(items: &[Vec<i32>]) -> usize {
	let mut total = 0usize;
	for item in items {
		total = total.wrapping_add(super::ex20_generic::measure_vec_i32(item));
	}
	total
}

#[inline(never)]
pub fn sum_measure_generic(items: &[Vec<i32>]) -> usize {
	let mut total = 0usize;
	for item in items {
		total = total.wrapping_add(super::ex20_generic::measure_generic_vec_i32(item));
	}
	total
}
