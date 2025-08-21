mod env;

use proc_macro2::TokenStream;
use quote::quote;
use chrono::Local;
use std::fs;

/// It is assumed to be used in `build.rs` or similar context.
pub fn handle_order() {
    println!("cargo:warning=Running spec-trait-order/build.rs at {}", Local::now().to_rfc3339());
    println!("cargo:rerun-if-changed={}", env::get_cache_path().to_string_lossy());
    println!("cargo:rerun-if-changed=.."); // TODO: remove after development

    fs::write(env::get_cache_path(), "{}").expect("Failed to write file cache");

    let s: &str =
        "
#[when(T: MyType)]
impl<T, U> Foo2<T, U> for ZST {
    fn foo(&self, x: T, y: U) {
        println!(\"Foo2 for ZST where T is MyType\");
    }
}"; // For each file in the project. In realtà dovremmo guardare il Cargo.toml per capire qual'è la struttura.
    let parsed: syn::Item = syn::parse_str(s).expect("Failed to parse the string as a syn::Item");
    let ts: TokenStream = quote! {
        #parsed
    };
    println!("cargo:warning=parsed: {:?}", s);
    println!("cargo:warning=TS: {:?}", ts);

    // Qui facciamo un dump su file di ciò che abbiamo collezionato. Something like:
    // ```
    // {
    //  crate1: {
    //   specializable: [ ... ]
    //   default_and_when: [ ... ] // Consideriamo di dividerli
    //   spec!: [ ... ] // For fl-macro, probably not needed
    //  },
    //  crate2: { ... }
    // }
    // ```
}
