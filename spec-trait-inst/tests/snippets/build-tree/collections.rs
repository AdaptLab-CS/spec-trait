fn main() -> usize {
    let x = [1, 2, 3, 4, 5];
    let y = &x[1..4];
    let z = [0; 10];
    let w = (x[2], 3);
    let (v, _) = w;
    let [u, ..] = x;
    match x {
        [first, _, third, ..] if first + third > 3 => return first + z[0..2].len(),
        _ => {}
    };
    let sum = x[0] + y[1] + z.len();
    sum += u + v + w.1;
    sum
}
