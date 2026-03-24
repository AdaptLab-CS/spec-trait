#![feature(rustc_private)]

fn main() {
    env_logger::init();
    spec_trait_inst::instrument::cli_main(
        spec_trait_inst::SpecRustInst,
        spec_trait_inst::SpecRustInst::before_exec,
        spec_trait_inst::SpecRustInst::after_exec,
    );
}
