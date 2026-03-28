// Ex16 -- runtime TypeId: total_capacity — cannot dispatch on wildcard/alias types via TypeId

pub struct CapacityOp;

pub fn total_capacity_runtime(op: &CapacityOp, items: &[Vec<u8>]) -> usize {
    let _ = op;
    items.len() * std::mem::size_of::<Vec<u8>>()
}
