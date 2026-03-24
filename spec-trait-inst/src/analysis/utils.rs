// tcx.crates(()).iter().for_each(|krate| {
//     log::debug!("Crate: {:?}", tcx.crate_name(*krate));
// });
pub const RUSTC_DEPENDENCIES: [&str; 19] = [
    "std",
    "core",
    "compiler_builtins",
    "rustc_std_workspace_core",
    "alloc",
    "libc",
    "unwind",
    "cfg_if",
    "miniz_oxide",
    "adler",
    "hashbrown",
    "rustc_std_workspace_alloc",
    "std_detect",
    "rustc_demangle",
    "addr2line",
    "gimli",
    "object",
    "memchr",
    "panic_unwind",
];
