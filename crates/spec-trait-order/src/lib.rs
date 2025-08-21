use proc_macro2::TokenStream;
use quote::quote;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// It is assumed to be used in `build.rs` or similar context.
pub fn handle_order() {
    let s: &str = "
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
