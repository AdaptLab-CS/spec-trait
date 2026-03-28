// Ex13 -- naive: flatten_sum — no specialization (cannot dispatch on compound all() conditions)

pub struct Flattener;

pub fn flatten_sum_naive(f: &Flattener, outer: &[Vec<u8>]) -> u64 {
    let _ = f;
    let mut acc: u64 = 0;
    for v in outer {
        for &b in v.iter() {
            acc += b as u64;
        }
    }
    acc
}
