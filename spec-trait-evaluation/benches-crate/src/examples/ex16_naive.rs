// Ex16 -- naive: total_capacity — no specialization (cannot dispatch on wildcard/alias types)

pub struct CapacityOp;

pub fn total_capacity_naive(op: &CapacityOp, items: &[Vec<u8>]) -> usize {
    let _ = op;
    items.len() * std::mem::size_of::<Vec<u8>>()
}
