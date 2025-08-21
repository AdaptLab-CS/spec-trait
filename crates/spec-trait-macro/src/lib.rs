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

# Examples
```no_run
use spec_trait_macro::specializable;

#[specializable]
trait MyTrait<T> {
    fn my_method(&self, arg: T);
}
```
*/
#[proc_macro_attribute]
pub fn specializable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tr = traits::parse(item.clone());
    cache::add_trait(tr);
    item
}

/**
`attr` is ignored

`item` is an implementation of a trait for a type:
- `impl<T> TraitName<T> for TypeName { ... }`

# Examples
```no_run
use spec_trait_macro::spec_default;

#[spec_default]
impl<T> MyTrait<T> for MyType {
    fn my_method(&self, arg: T) {
        println!("Default MyTrait for MyType");
    }
}
```
*/
#[proc_macro_attribute]
pub fn spec_default(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_body = impls::parse(item);
    handle_specialization(None, impl_body)
}

// TODO: add support to other cases (e.g. Vec<_>, &[_], (_,_), etc.)
/**
`attr` is a condition in one of these forms:
- `T: TraitName`
- `T: TraitName1 + TraitName2`
- `T = TypeName`
- `T = &TypeName`
- `all(attr1, attr2, ...)`
- `any(attr1, attr2, ...)`
- `not(attr)`

`item` is an implementation of a trait for a type:
- `impl<T> TraitName<T> for TypeName { ... }`
- `impl<T> TraitName for TypeName<T> { ... }`

# Examples
```no_run
use spec_trait_macro::when;

#[when(T: Foo + Bar)]
impl<T> MyTrait<T> for MyType {
    fn my_method(&self, arg: T) {
        println!("MyTrait for MyType where T implements Foo and Bar");
    }
}

#[when(not(T = i32))]
impl<T> MyTrait<T> for MyType {
    fn my_method(&self, arg: T) {
        println!("MyTrait for MyType where T is not i32");
    }
}
```
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
    let spec_trait_name = traits::generate_trait_name(&trait_body.name);

    let trait_token_stream = traits::create_spec(&trait_body, &spec_trait_name);
    let body_token_stream = impls::create_spec(&impl_body, &spec_trait_name);

    let combined = quote::quote! {
        #trait_token_stream
        #body_token_stream
    };

    cache::add_impl(Impl {
        condition,
        trait_name: trait_body.name.clone(),
        spec_trait_name,
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

`args_types` is a colon separated list of types for the arguments in the `method_call`.

`annotations` is a semi-colon separated list, where each item can be one of these forms:
- `TypeName: TraitName`
- `TypeName: TraitName1 + TraitName2`
- `TypeName = AliasName`

# Examples
```no_run
use spec_trait_macro::spec;

let x = MyType;
...
spec! { x.my_method(1u8); MyType; [u8] };
spec! { x.my_method("str", 1); MyType; [&str, i32], i32 = MyAlias  };
```
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
