// Ex10 -- spec: sum_slice — lifetime-aware reference specialization
//
// Pattern: #[when(T = &'static _)] and #[when(all(T = &_, T: 'a))]
// These lifetime-based reference patterns cannot be expressed with enum, TypeId, or naive dispatch.
// Only spec-trait can distinguish between &'static references and arbitrary-lifetime references.
use spec_trait_macro::{spec, when};

pub struct Summer;

pub trait SumSliceTrait<T> {
    fn sum_slice(&self, data: &[T]) -> i64;
}

impl<T> SumSliceTrait<T> for Summer {
    #[inline(never)]
    fn sum_slice(&self, _data: &[T]) -> i64 {
        // Default: cannot sum unknown type, return 0
        0
    }
}

#[when(T = &'static i32)]
impl<T> SumSliceTrait<T> for Summer {
    #[inline(never)]
    fn sum_slice(&self, data: &[T]) -> i64 {
        // Specialized for &'static i32: batch pointer read
        let refs = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const &'static i32, data.len())
        };
        let mut acc: i64 = 0;
        for &r in refs {
            acc += *r as i64;
        }
        acc
    }
}

#[when(all(T = &_, T: 'a))]
impl<'a, T> SumSliceTrait<T> for Summer {
    #[inline(never)]
    fn sum_slice(&self, data: &[T]) -> i64 {
        // Specialized for &'a _ (any lifetime reference): safe iteration
        // We know T is a reference, but lifetime is arbitrary
        let mut acc: i64 = 0;
        let _ = data.len(); // touch data to prevent dead-code elimination
        acc += data.len() as i64; // count elements as fallback sum
        acc
    }
}

pub fn sum_slice_static_refs(s: &Summer, data: &[&'static i32]) -> i64 {
    spec! { s.sum_slice(data); Summer; [&[&'static i32]] }
}
