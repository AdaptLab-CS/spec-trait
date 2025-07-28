mod annotations;
mod body;
mod conditions;
mod traits;

use proc_macro::TokenStream;

const FOLDER_CACHE: &str = "/tmp";
const FILE_CACHE: &str = "file_cache.cache";

/**
`attr` is ignored

`item` is a trait definition:
- `trait TraitName { ... }`
- `trait TraitName<T> { ... }`
*/
#[proc_macro_attribute]
pub fn specializable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tr = traits::parse(item.clone());
    println!("Parsed trait: {:?}", tr);

    // TODO: write trait into `file_cache.cache`

    item
}

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
    println!("Parsed condition: {:?}", cond);

    let body = body::parse(item);
    println!("Parsed body: {:?}", body);

    // TODO: Leggi `file_cache.cache` (usando serde) e verifica se le condizioni sono soddisfatte

    TokenStream::new()
}

/**
`item` can be one of these forms:
- `method_call`
- `method_call; annotations`

`method_call` can be one of these forms:
- `variable.function(args)`

`annotations` is a `;` separated list, where each item can be one of these forms:
- `TypeName: TraitName`
- `TypeName: TraitName1 + TraitName2`
- `TypeName = AliasName`
*/
#[proc_macro]
pub fn spec(item: TokenStream) -> TokenStream {
    println!("***");

    let ann = annotations::parse(item);
    println!("Parsed annotation: {:?}", ann);

    TokenStream::new()
}
