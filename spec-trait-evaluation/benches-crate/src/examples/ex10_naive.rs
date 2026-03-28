// Ex10 -- naive: sum_slice — no specialization (cannot dispatch on reference lifetimes)

pub struct Summer;

pub fn sum_slice_naive(s: &Summer, data: &[&'static i32]) -> i64 {
    let _ = s;
    let mut acc: i64 = 0;
    for &r in data {
        acc += *r as i64;
    }
    acc
}
