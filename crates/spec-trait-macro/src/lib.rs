mod annotations;
mod cache;
mod conditions;
mod conversions;
mod env;
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

    // TODO: get dynamically from annotations
    let var_type = "ZST";

    let traits = cache::get_traits_by_fn(&ann.fn_, ann.args.len());
    let impls = cache::get_impls_by_type_and_traits(&var_type, &traits);

    let impl_ = spec::get_most_specific_impl(&impls, &traits, &ann);
    println!("most specific impl: {:?}", impl_);

    let res = spec::create_spec(&impl_, &ann);
    println!("spec result: {}", res.to_string());
    res
}
