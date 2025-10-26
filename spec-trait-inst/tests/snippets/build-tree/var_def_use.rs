fn main() {
    const INIT: i32 = 10;
    static ADDER: i32 = 5;
    let x = INIT;
    let y = x + ADDER;
    let z;
    z = x * y / 2;
}
