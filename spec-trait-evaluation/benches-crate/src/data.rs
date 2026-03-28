/// Test data generators for benchmarks.

pub fn make_u8(n: usize) -> Vec<u8> {
    (0..n).map(|i| (i % 256) as u8).collect()
}

pub fn make_i32(n: usize) -> Vec<i32> {
    (0..n as i32).collect()
}

pub fn make_f64(n: usize) -> Vec<f64> {
    (0..n).map(|i| i as f64 * 0.1).collect()
}

pub fn make_sorted_u8(n: usize) -> Vec<u8> {
    let mut data = make_u8(n);
    data.sort();
    data
}

pub fn make_u64(n: usize) -> Vec<u64> {
    (0..n as u64).collect()
}

pub fn make_strings(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("item_{}", i)).collect()
}

/// Static i32 values for reference-based benchmarks.
/// Returns a Vec of &'static i32 references to a leaked array.
pub fn make_static_i32_refs(n: usize) -> Vec<&'static i32> {
    let data: Vec<i32> = (0..n as i32).collect();
    let leaked: &'static [i32] = Box::leak(data.into_boxed_slice());
    leaked.iter().collect()
}

/// Nested Vec<u8> for flatten benchmarks.
/// Creates `n` inner vectors, each of size `chunk`.
pub fn make_nested_vec_u8(n: usize, chunk: usize) -> Vec<Vec<u8>> {
    (0..n)
        .map(|i| (0..chunk).map(|j| ((i * chunk + j) % 256) as u8).collect())
        .collect()
}

/// Vec<u8> items for capacity benchmarks.
pub fn make_vec_u8_items(n: usize, inner_cap: usize) -> Vec<Vec<u8>> {
    (0..n)
        .map(|_| {
            let mut v = Vec::with_capacity(inner_cap);
            v.extend((0..inner_cap).map(|j| (j % 256) as u8));
            v
        })
        .collect()
}
