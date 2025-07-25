mod conditions;

use proc_macro::TokenStream;
use syn::{ItemImpl, parse_macro_input};

const FOLDER_CACHE: &str = "/tmp";
const FILE_CACHE: &str = "file_cache.cache";

// TODO: add support to other cases
/**
`attr` can be one of these forms:
- `T: TraitName`
- `T: TraitName1 + TraitName2`
- `T = TypeName`
- `T = &TypeName`
- `all(attr1, attr2, ...)`
- `any(attr1, attr2, ...)`
- `not(attr)`

`item` can be one of these forms:
- `impl<T> TraitName<T> for TypeName { ... }`
- `impl<T> TraitName for TypeName<T> { ... }`
*/
#[proc_macro_attribute]
pub fn when(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("***");

    let cond = conditions::parse(attr);
    println!("parsed condition: {:?}", cond);

    let bod = parse_macro_input!(item as ItemImpl);
    let impl_generics = &bod.generics;
    let impl_trait = &bod.trait_;
    let impl_type = &bod.self_ty;
    println!("parsed body: {}", quote::quote! { #bod });
    println!("generics: {}", quote::quote! { #impl_generics });
    if let Some((_, path, _)) = impl_trait {
        println!("trait: {}", quote::quote! { #path });
    }
    println!("type: {}", quote::quote! { #impl_type });

    // TODO: Leggi `file_cache.cache` (usando serde) e verifica se le condizioni sono soddisfatte

    TokenStream::new()
}
