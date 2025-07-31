mod annotations;
mod body;
mod cache;
mod conditions;
mod env;
mod traits;

use cache::Impl;
use proc_macro::TokenStream;

/**
`attr` is ignored

`item` is a trait definition:
- `trait TraitName { ... }`
- `trait TraitName<T> { ... }`
*/
#[proc_macro_attribute]
pub fn specializable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tr = traits::parse(item.clone());
    cache::add_trait(tr);
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
    let impl_body = body::parse(item);

    let trait_name = &impl_body.trait_;
    let trait_body = cache::get_trait(trait_name).expect("Trait not found in cache");

    let (new_trait_name, spec) = body::create_spec(&impl_body, &trait_body);

    cache::add_impl(Impl {
        condition: cond,
        trait_name: new_trait_name,
    });

    println!("spec: {}", spec.to_string());

    spec
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
