// Ex16 -- enum dispatch: total_capacity — cannot dispatch on wildcard/alias types via enum

pub struct CapacityOp;

pub fn total_capacity_enum(op: &CapacityOp, items: &[Vec<u8>]) -> usize {
    let _ = op;
    items.len() * std::mem::size_of::<Vec<u8>>()
}
