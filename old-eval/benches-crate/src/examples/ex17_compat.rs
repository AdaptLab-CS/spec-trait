// Legacy compatibility facade for non-spec variants.
// Prefer importing ex17_{dyn,enum,generic,runtime_typeid} directly.

pub use super::ex17_dyn::*;
pub use super::ex17_enum::*;
pub use super::ex17_generic::*;
pub use super::ex17_runtime_typeid::*;

#[inline(never)]
pub fn sum_parse_enum_static(items: &[&'static str]) -> usize {
	let mut total = 0usize;
	for &s in items {
		let e = super::ex17_enum::ParseEnum::StaticStr(s);
		total = total.wrapping_add(super::ex17_enum::parse_enum(&e));
	}
	total
}

#[inline(never)]
pub fn sum_parse_dyn_static(items: &[&'static str]) -> usize {
	let mut total = 0usize;
	for &s in items {
		let d = super::ex17_dyn::DynStaticStr(s);
		total = total.wrapping_add(super::ex17_dyn::DynParse::parse_dyn(&d));
	}
	total
}

#[inline(never)]
pub fn sum_parse_generic(items: &[&str]) -> usize {
	let mut total = 0usize;
	for &s in items {
		total = total.wrapping_add(super::ex17_generic::parse_generic(s));
	}
	total
}
