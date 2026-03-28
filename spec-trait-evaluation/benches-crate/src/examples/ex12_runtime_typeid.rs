// Ex12 -- runtime TypeId: fold_with — cannot dispatch on HRTB via TypeId

pub struct Folder;

pub fn fold_with_runtime(folder: &Folder, data: &[i32]) -> i64 {
    let _ = folder;
    let f = |x: &i32| x.wrapping_mul(2);
    let mut acc: i64 = 0;
    for &x in data {
        acc += f(&x) as i64;
    }
    acc
}
