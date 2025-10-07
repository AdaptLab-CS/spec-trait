pub mod sti_analysis;
pub mod utils;

use crate::CliArgs;
use rustc_middle::mir;
use rustc_middle::ty;
use sti_analysis::STIAnalysis;

pub struct Analyzer<'tcx> {
    tcx: ty::TyCtxt<'tcx>,
    cli_args: CliArgs,
}

impl<'tcx> Analyzer<'tcx> {
    pub fn new(tcx: ty::TyCtxt<'tcx>, cli_args: CliArgs) -> Self {
        Self { tcx, cli_args }
    }

    fn pre_process_cli_args(&self) {
        log::debug!("Pre-processing CLI arguments");
        if self.cli_args.print_crate {
            log::debug!("Printing the crate");
            let resolver_and_krate = self.tcx.resolver_for_lowering().borrow();
            let krate = &*resolver_and_krate.1;
            println!("{:#?}", krate);
        }

        if self.cli_args.print_mir {
            log::debug!("Printing the MIR");
            mir::write_mir_pretty(self.tcx, None, &mut std::io::stdout())
                .expect("write_mir_pretty failed");
        }
    }

    fn post_process_cli_args(&self) {
        log::debug!("Post-processing CLI arguments");
    }

    fn run_analysis(&self, name: &str, f: impl FnOnce(&Self)) {
        log::debug!("Running analysis: {}", name);
        f(self);
        log::debug!("Finished analysis: {}", name);
    }

    pub fn run(&self) {
        self.pre_process_cli_args();
        println!("CIAO");
        self.run_analysis("STIAnalysis", |analyzer| {
            STIAnalysis::new(analyzer).run();
        });
        self.post_process_cli_args();
    }
}
