// Legacy compatibility facade for non-spec variants.
// Prefer importing ex24_{dyn,enum,generic,runtime_typeid} directly.

pub use super::ex24_dyn::*;
pub use super::ex24_enum::*;
pub use super::ex24_generic::*;
pub use super::ex24_runtime_typeid::*;

#[inline(never)]
pub fn sum_hash_enum(data: &[i32]) -> u64 {
	let mut total = 0u64;
	for item in data {
		total = total.wrapping_add(super::ex24_enum::hash_enum_i32(&super::ex24_enum::HashEnum::StaticI32, item));
	}
	total
}

#[inline(never)]
pub fn sum_hash_runtime(data: &[i32]) -> u64 {
	let mut total = 0u64;
	for item in data {
		total = total.wrapping_add(super::ex24_runtime_typeid::hash_runtime_i32(item));
	}
	total
}

#[inline(never)]
pub fn sum_hash_dyn(data: &[i32]) -> u64 {
	let mut total = 0u64;
	let d = super::ex24_dyn::DynStaticHasher;
	for item in data {
		total = total.wrapping_add(super::ex24_dyn::DynHash::hash_dyn(&d, item));
	}
	total
}

#[inline(never)]
pub fn sum_hash_manual(data: &[i32]) -> u64 {
	let mut total = 0u64;
	for item in data {
		total = total.wrapping_add(super::ex24_generic::hash_manual_i32(item));
	}
	total
}
