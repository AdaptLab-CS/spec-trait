// Legacy compatibility facade for non-spec variants.
// Prefer importing ex22_{dyn,enum,generic,runtime_typeid} directly.

pub use super::ex22_dyn::*;
pub use super::ex22_enum::*;
pub use super::ex22_generic::*;

#[inline(never)]
pub fn sum_dispatch_enum(fns: &[fn(i32) -> i32]) -> i64 {
	let mut total = 0i64;
	for &f in fns {
		let e = super::ex22_enum::DispatchEnum::FnI32(f);
		total = total.wrapping_add(super::ex22_enum::dispatch_enum(&e));
	}
	total
}

#[inline(never)]
pub fn sum_dispatch_manual(fns: &[fn(i32) -> i32]) -> i64 {
	let mut total = 0i64;
	for &f in fns {
		total = total.wrapping_add(super::ex22_generic::dispatch_fn_i32(f));
	}
	total
}

#[inline(never)]
pub fn sum_dispatch_dyn(fns: &[fn(i32) -> i32]) -> i64 {
	let mut total = 0i64;
	for &f in fns {
		let d = super::ex22_dyn::DynFnI32(f);
		total = total.wrapping_add(super::ex22_dyn::DynDispatch::dispatch_dyn(&d));
	}
	total
}
