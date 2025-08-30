mod annotations;
mod generics;
mod spec;
mod constraints;

use spec_trait_utils::conditions::WhenCondition;
use spec_trait_utils::cache;
use spec_trait_utils::impls::ImplBody;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use annotations::AnnotationBody;

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
```ignore
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
    let cond = WhenCondition::try_from(TokenStream2::from(attr)).expect(
        "Failed to parse TokenStream into WhenCondition"
    );
    let impl_body = ImplBody::try_from((TokenStream2::from(item), Some(cond))).expect(
        "Failed to parse TokenStream into ImplBody"
    );

    let mut trait_body = cache
        ::get_trait_by_name(&impl_body.trait_name)
        .expect("Trait not found in cache");

    trait_body.name = impl_body.spec_trait_name.clone();

    let trait_token_stream = TokenStream2::from(&trait_body);
    let impl_token_stream = TokenStream2::from(&impl_body);

    //TODO: infer generics from conditions (e.g. with condition "T = Type" generic "T" is replaced with type "Type")

    let combined = quote::quote! {
        #trait_token_stream
        #impl_token_stream
    };

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
```ignore
use spec_trait_macro::spec;

let x = MyType;
...
spec! { x.my_method(1u8); MyType; [u8] };
spec! { x.my_method("str", 1); MyType; [&str, i32], i32 = MyAlias  };
```
*/
#[proc_macro]
pub fn spec(item: TokenStream) -> TokenStream {
    let ann = AnnotationBody::try_from(TokenStream2::from(item)).expect(
        "Failed to parse TokenStream into AnnotationBody"
    );

    let traits = cache::get_traits_by_fn(&ann.fn_, ann.args.len());
    let impls = cache::get_impls_by_type_and_traits(&ann.var_type, &traits);

    let (impl_, constraints) = spec::get_most_specific_impl(&impls, &traits, &ann);
    let trait_ = traits
        .iter()
        .find(|tr| tr.name == impl_.trait_name)
        .unwrap();

    let generics = generics::get_for_impl(&trait_, &constraints);

    let res = spec::create_spec(&impl_, &generics, &ann);
    res.into()
}
