// Ex09 -- runtime TypeId: format_collect — cannot dispatch on lifetime bounds via TypeId

pub struct Formatter;

pub fn format_collect_runtime(f: &Formatter, data: &[i32]) -> usize {
    let _ = f;
    data.len()
}
