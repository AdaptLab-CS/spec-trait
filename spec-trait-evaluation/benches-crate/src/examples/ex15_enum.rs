// Ex15 -- enum dispatch: compute_sum — cannot dispatch on trait object types via enum

pub trait Summable {
    fn value(&self) -> i64;
}

impl Summable for i32 {
    fn value(&self) -> i64 {
        *self as i64
    }
}

pub struct Computer;

pub fn compute_sum_enum(c: &Computer, items: &[&dyn Summable]) -> i64 {
    let _ = c;
    let mut acc: i64 = 0;
    for item in items {
        acc += item.value();
    }
    acc
}
