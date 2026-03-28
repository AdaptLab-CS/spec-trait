// Ex11 -- runtime TypeId: map_transform — cannot dispatch on fn pointer types via TypeId

pub struct Mapper;

fn double_i32(x: &i32) -> i32 {
    x.wrapping_mul(2)
}

pub fn map_transform_runtime(m: &Mapper, data: &[i32]) -> Vec<i32> {
    let _ = m;
    data.iter().map(double_i32).collect()
}
