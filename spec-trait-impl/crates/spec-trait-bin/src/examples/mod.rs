mod all;
mod base;
mod higher;
mod lifetimes;
mod lifetimes2;
mod order;
mod repeating;
mod sself;
mod traits;
mod types;

pub fn run() {
    base::run();
    types::run();
    traits::run();
    lifetimes::run();
    repeating::run();
    sself::run();
    higher::run();
    order::run();
    all::run();
    lifetimes2::run();
}
