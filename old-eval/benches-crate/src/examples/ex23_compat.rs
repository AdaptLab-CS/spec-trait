// Legacy compatibility facade for non-spec variants.
// Prefer importing ex23_{dyn,enum,generic,runtime_typeid} directly.

pub use super::ex23_dyn::*;
pub use super::ex23_enum::*;
pub use super::ex23_generic::*;

#[inline(never)]
pub fn sum_transform_enum_u8(data: &[u8]) -> i64 {
	let mut total = 0i64;
	for _ in data {
		let e = super::ex23_enum::ReceiverEnum::TypeA;
		total = total.wrapping_add(super::ex23_enum::transform_enum_u8(&e));
	}
	total
}

#[inline(never)]
pub fn sum_transform_manual_u8(data: &[u8]) -> i64 {
	let mut total = 0i64;
	for _ in data {
		total = total.wrapping_add(super::ex23_generic::transform_a_u8());
	}
	total
}

#[inline(never)]
pub fn sum_transform_dyn(data: &[u8]) -> i64 {
	let mut total = 0i64;
	let d = super::ex23_dyn::DynTypeA;
	for _ in data {
		total = total.wrapping_add(super::ex23_dyn::DynTransformU8::transform_dyn(&d));
	}
	total
}

#[inline(never)]
pub fn sum_transform_generic(data: &[u8]) -> i64 {
	let mut total = 0i64;
	for &x in data {
		total = total.wrapping_add(super::ex23_generic::transform_generic(x));
	}
	total
}
