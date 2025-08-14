mod annotations;
mod cache;
mod conditions;
mod conversions;
mod env;
mod generics;
mod impls;
mod spec;
mod traits;

use cache::Impl;
use conditions::WhenCondition;
use impls::ImplBody;
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

/**
`attr` is ignored

`item` can be one of these forms:
- `impl<T> TraitName<T> for TypeName { ... }`
- `impl<T> TraitName for TypeName<T> { ... }`
*/
#[proc_macro_attribute]
pub fn spec_default(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_body = impls::parse(item);
    handle_specialization(None, impl_body)
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
    let normalized_cond = conditions::normalize(&cond);
    let impl_body = impls::parse(item);
    handle_specialization(Some(normalized_cond), impl_body)
}

fn handle_specialization(condition: Option<WhenCondition>, impl_body: ImplBody) -> TokenStream {
    let trait_body = cache::get_trait_by_name(&impl_body.trait_).expect("Trait not found in cache");
    let new_trait_name = traits::generate_trait_name(&trait_body.name);

    let trait_token_stream = traits::create_spec(&trait_body, &new_trait_name);
    let body_token_stream = impls::create_spec(&impl_body, &new_trait_name);

    let combined = quote::quote! {
        #trait_token_stream
        #body_token_stream
    };

    cache::add_impl(Impl {
        condition: condition,
        trait_name: trait_body.name.clone(),
        spec_trait_name: new_trait_name,
        type_name: impl_body.type_.clone(),
    });

    combined.into()
}

/**
`item` can be one of these forms:
- `method_call; variable_type; [args_types]`
- `method_call; variable_type; [args_types]; annotations`

`method_call` can be one of these forms:
- `variable.function(args)`

`variable_type` is the type of the variable in the `method_call`.

`args_types` is a list of types for the arguments in the `method_call`.

`annotations` is a semi-colon separated list, where each item can be one of these forms:
- `TypeName: TraitName`
- `TypeName: TraitName1 + TraitName2`
- `TypeName = AliasName`
*/
#[proc_macro]
pub fn spec(item: TokenStream) -> TokenStream {
    let ann = annotations::parse(item);

    let traits = cache::get_traits_by_fn(&ann.fn_, ann.args.len());
    let impls = cache::get_impls_by_type_and_traits(&ann.var_type, &traits);

    let (impl_, constraints) = spec::get_most_specific_impl(&impls, &traits, &ann);

    let generics = generics::get_for_impl(&impl_, &traits, &constraints);

    let res = spec::create_spec(&impl_, &generics, &ann);
    res.into()
}
