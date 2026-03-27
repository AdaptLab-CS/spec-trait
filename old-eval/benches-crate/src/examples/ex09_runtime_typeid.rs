// Example 09 — RUNTIME TYPEID: Display-based formatting
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn fmt_runtime<T: std::fmt::Debug + 'static>(items: &[T]) -> String {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        let data = unsafe { &*(items as *const [T] as *const [i32]) };
        let mut buf = itoa::Buffer::new();
        data.iter().map(|x| buf.format(*x).to_owned()).collect::<Vec<_>>().join(", ")
    } else if tid == TypeId::of::<f64>() {
        let data = unsafe { &*(items as *const [T] as *const [f64]) };
        let mut buf = ryu::Buffer::new();
        data.iter().map(|x| buf.format(*x).to_owned()).collect::<Vec<_>>().join(", ")
    } else {
        items.iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(", ")
    }
}

