// Ex12 -- spec: fold_with — higher-ranked trait bound specialization
//
// Pattern: #[when(T: for<'a> Fn(&'a i32) -> i32)]
// This higher-ranked trait bound cannot be expressed with enum, TypeId, or naive dispatch.
// Only spec-trait can dispatch based on for<'a> Fn(&'a ...) bounds.
use spec_trait_macro::{spec, when};

pub struct Folder;

pub trait FoldWithTrait<T> {
    fn fold_with(&self, data: &[i32], f: T) -> i64;
}

impl<T> FoldWithTrait<T> for Folder {
    #[inline(never)]
    fn fold_with(&self, data: &[i32], _f: T) -> i64 {
        // Default: simple sum without applying f (cannot call unknown T)
        let mut acc: i64 = 0;
        for &x in data {
            acc += x as i64;
        }
        acc
    }
}

#[when(T: for<'a> Fn(&'a i32) -> i32)]
impl<T> FoldWithTrait<T> for Folder {
    #[inline(never)]
    fn fold_with(&self, data: &[i32], f: T) -> i64 {
        // Specialized: T implements for<'a> Fn(&'a i32) -> i32
        // Unrolled 4-wide accumulation for better ILP
        let mut acc0: i64 = 0;
        let mut acc1: i64 = 0;
        let mut acc2: i64 = 0;
        let mut acc3: i64 = 0;
        let chunks = data.chunks_exact(4);
        let remainder = chunks.remainder();
        for chunk in chunks {
            acc0 += f(&chunk[0]) as i64;
            acc1 += f(&chunk[1]) as i64;
            acc2 += f(&chunk[2]) as i64;
            acc3 += f(&chunk[3]) as i64;
        }
        for &x in remainder {
            acc0 += f(&x) as i64;
        }
        acc0 + acc1 + acc2 + acc3
    }
}

pub fn fold_with_hrtb(folder: &Folder, data: &[i32]) -> i64 {
    let f = |x: &i32| x.wrapping_mul(2);
    spec! { folder.fold_with(data, f); Folder; [&[i32], T]; T: for<'a> Fn(&'a i32) -> i32 }
}
