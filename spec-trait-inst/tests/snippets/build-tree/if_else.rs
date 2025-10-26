fn main() -> i32 {
    let x = 10;

    if x > 5 || x <= -20 {
        return 10;
    } else if x == 0 {
        break;
    } else {
        continue;
    }

    if x < 0 {
        return x;
    }

    enum Value {
        Number(i32),
    }

    if let Value::Number(x) = Value::Number(42) {
        return x;
    }

    return -1;
}
