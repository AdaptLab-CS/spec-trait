// Example 28 — ENUM: Tuple type specialization


// --- Approach A: enum dispatch ---
pub enum PointEnum<'a> {
    I32(&'a [(i32, i32)]),
    F64(&'a [(f64, f64)]),
    F32(&'a [(f32, f32)]),
}

#[inline(never)]
pub fn squared_sum_enum(variant: &PointEnum) -> f64 {
    match variant {
        PointEnum::I32(data) => {
            let mut sum = 0i64;
            for &(x, y) in *data {
                sum += (x as i64) * (x as i64) + (y as i64) * (y as i64);
            }
            sum as f64
        }
        PointEnum::F64(data) => {
            let mut sum = 0.0f64;
            for &(x, y) in *data {
                sum += x * x + y * y;
            }
            sum
        }
        PointEnum::F32(data) => {
            let mut sum = 0.0f32;
            for &(x, y) in *data {
                sum += x * x + y * y;
            }
            sum as f64
        }
    }
}

