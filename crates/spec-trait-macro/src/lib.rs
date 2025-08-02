mod annotations;
mod body;
mod cache;
mod conditions;
mod conversions;
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

    let normalized_cond = conditions::normalize(&cond);

    let trait_body = cache::get_trait(&impl_body.trait_).expect("Trait not found in cache");
    let new_trait_name = traits::generate_trait_name(&trait_body.name);

    let trait_token_stream = traits::create_spec(&trait_body, &new_trait_name);
    let body_token_stream = body::create_spec(&impl_body, &new_trait_name);

    let combined = quote::quote! {
        #trait_token_stream
        #body_token_stream
    };

    cache::add_impl(Impl {
        condition: normalized_cond,
        trait_name: new_trait_name,
    });

    combined.into()
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
