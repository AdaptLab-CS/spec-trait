/// Test data generators for benchmarks.

pub fn make_u8(n: usize) -> Vec<u8> {
    (0..n).map(|i| (i % 256) as u8).collect()
}

pub fn make_u32(n: usize) -> Vec<u32> {
    (0..n).map(|i| i as u32).collect()
}

pub fn make_u64(n: usize) -> Vec<u64> {
    (0..n).map(|i| i as u64).collect()
}

pub fn make_i32(n: usize) -> Vec<i32> {
    (0..n as i32).collect()
}

pub fn make_f64(n: usize) -> Vec<f64> {
    (0..n).map(|i| i as f64 * 0.1).collect()
}

pub fn make_f32(n: usize) -> Vec<f32> {
    (0..n).map(|i| i as f32 * 0.1).collect()
}

pub fn make_i64(n: usize) -> Vec<i64> {
    (0..n as i64).collect()
}

pub fn make_sorted_u8(n: usize) -> Vec<u8> {
    let mut data = make_u8(n);
    data.sort();
    data
}

pub fn make_strings(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("item_{}", i)).collect()
}
