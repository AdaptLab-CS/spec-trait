mod all;
mod base;
mod lifetimes;
mod nested;
mod order;
mod sself;
mod traits;
mod types;
mod higher;

pub fn run() {
    base::run();
    types::run();
    traits::run();
    lifetimes::run();
    nested::run();
    sself::run();
    higher::run();
    order::run();
    all::run();
}
