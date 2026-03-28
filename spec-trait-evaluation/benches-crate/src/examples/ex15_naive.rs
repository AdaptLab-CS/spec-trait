// Ex15 -- naive: compute_sum — no specialization (cannot dispatch on trait object types)

pub trait Summable {
    fn value(&self) -> i64;
}

impl Summable for i32 {
    fn value(&self) -> i64 {
        *self as i64
    }
}

pub struct Computer;

pub fn compute_sum_naive(c: &Computer, items: &[&dyn Summable]) -> i64 {
    let _ = c;
    let mut acc: i64 = 0;
    for item in items {
        acc += item.value();
    }
    acc
}
