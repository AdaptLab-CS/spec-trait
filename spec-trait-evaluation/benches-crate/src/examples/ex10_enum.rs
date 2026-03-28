// Ex10 -- enum dispatch: sum_slice — cannot dispatch on reference lifetimes via enum

pub struct Summer;

pub fn sum_slice_enum(s: &Summer, data: &[&'static i32]) -> i64 {
    let _ = s;
    let mut acc: i64 = 0;
    for &r in data {
        acc += *r as i64;
    }
    acc
}
