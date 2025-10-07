#![feature(rustc_private)]

fn main() {
    env_logger::init();
    spec_trait_inst::instrument::driver_main(spec_trait_inst::SpecRustInst);
}
