trait Test {
    fn a(&self, x: i32) -> Self;
    fn b(&self, x: fn(i32) -> i32) -> Self;
    fn c(&self) -> Self;
}

impl Test for i32 {
    fn a(&self, _: i32) -> Self {
        *self
    }
    fn b(&self, x: fn(i32) -> i32) -> Self {
        x(*self)
    }
    fn c(&self) -> Self {
        *self
    }
}

fn main() -> i32 {
    let x = 10;

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let result = add(x, add(5, 15));

    let multiply = |a: i32, b: i32| a * b;

    let result = multiply(result, 2);

    fn plus_one(x: i32) -> i32 {
        add(x, 1)
    }

    let times_two = |a: i32| multiply(a, 2);

    result
        .a(result)
        .b(|x| multiply(x, 3))
        .b(plus_one)
        .b(times_two)
        .c()
}
