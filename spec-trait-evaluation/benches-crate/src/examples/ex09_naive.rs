// Ex09 -- naive: format_collect — no specialization (cannot dispatch on lifetime bounds)

pub struct Formatter;

pub fn format_collect_naive(f: &Formatter, data: &[i32]) -> usize {
    let _ = f;
    // Naive: just count elements, no specialization possible
    data.len()
}
