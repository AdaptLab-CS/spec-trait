mod all;
mod base;
mod lifetimes;
mod nested;
mod sself;
mod traits;
mod types;

fn main() {
    base::run();
    types::run();
    traits::run();
    lifetimes::run();
    nested::run();
    sself::run();
    all::run();
}
