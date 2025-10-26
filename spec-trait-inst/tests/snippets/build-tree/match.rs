fn main() {
    let x = 10;

    match x * 2 {
        0 => return 0,
        10 | 20 => return 10,
        5..=15 => return 15,
        n @ 30..50 => return n,
        oth if oth % 2 == 0 => return oth / 2,
        _ => {}
    }

    enum Value {
        Number(i32),
        Text(String),
    }

    match Value::Number(42) {
        Value::Number(n) if n > 0 => n,
        Value::Number(_) => -1,
        Value::Text(_) => -10,
    }
}
