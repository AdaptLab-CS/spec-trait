// Ex09 -- enum dispatch: format_collect — cannot dispatch on lifetime bounds via enum

pub struct Formatter;

pub fn format_collect_enum(f: &Formatter, data: &[i32]) -> usize {
    let _ = f;
    data.len()
}
