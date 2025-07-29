mod annotations;
mod body;
mod cache;
mod conditions;
mod traits;

use cache::{FILE_CACHE, FOLDER_CACHE};
use proc_macro::TokenStream;
use std::fs;
use std::path::Path;

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

    let dest_path = Path::new(&FOLDER_CACHE).join(&FILE_CACHE);
    let file_cache = fs::read(&dest_path).expect("Failed to read file cache");

    let mut cache: serde_json::Value =
        serde_json::from_slice(&file_cache).expect("Failed to parse file cache");

    cache["traits"] = if cache["traits"].is_null() {
        serde_json::Value::Array(vec![])
    } else {
        cache["traits"].take()
    };

    cache["traits"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!(tr));

    let serialized = serde_json::to_string(&cache).expect("Failed to serialize cache");

    fs::write(&dest_path, serialized).expect("Failed to write file cache");

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
    let ann = annotations::parse(item);
    println!("Parsed annotation: {:?}", ann);

    // TODO: read from `file_cache.cache` and apply the specialization

    TokenStream::new()
}
