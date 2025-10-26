fn main() {
    use std::collections::HashMap;
    use std::collections::*;

    struct S {
        map: HashMap<i32, i32>,
    }

    let x = S {
        map: std::collections::HashMap::new(),
    };

    loop {
        break;
    }
}
