// Example 29 — ENUM: Nested generic type specialization


// --- Approach A: enum dispatch ---
pub enum FlatEnum<'a> {
    VecVecU8(&'a [Vec<Vec<u8>>]),
    VecString(&'a [Vec<String>]),
    VecI32(&'a [Vec<i32>]),
}

#[inline(never)]
pub fn flat_enum(variant: &FlatEnum) -> usize {
    match variant {
        FlatEnum::VecVecU8(data) => {
            let mut total = 0usize;
            for outer in *data {
                for inner in outer {
                    total += inner.len();
                }
            }
            total
        }
        FlatEnum::VecString(data) => {
            let mut total = 0usize;
            for outer in *data {
                for s in outer {
                    total += s.len();
                }
            }
            total
        }
        FlatEnum::VecI32(data) => {
            let mut total = 0usize;
            for outer in *data {
                total += outer.len() * 4;
            }
            total
        }
    }
}

