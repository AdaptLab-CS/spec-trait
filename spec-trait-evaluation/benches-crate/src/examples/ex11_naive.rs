// Ex11 -- naive: map_transform — no specialization (cannot dispatch on fn pointer types)

pub struct Mapper;

fn double_i32(x: &i32) -> i32 {
    x.wrapping_mul(2)
}

pub fn map_transform_naive(m: &Mapper, data: &[i32]) -> Vec<i32> {
    let _ = m;
    data.iter().map(double_i32).collect()
}
