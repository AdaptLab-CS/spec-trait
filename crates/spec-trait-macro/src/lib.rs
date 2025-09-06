mod annotations;
mod vars;
mod spec;
mod constraints;
mod types;

use spec_trait_utils::conditions::{ self, WhenCondition };
use spec_trait_utils::cache;
use spec_trait_utils::impls::ImplBody;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use annotations::AnnotationBody;
use quote::quote;
use crate::spec::SpecBody;

// TODO: check support to other cases
/**
`attr` is a condition in one of these forms:
- `T: TraitName`
- `T: TraitName1 + TraitName2`
- `T = _`
- `T = TypeName`
- `T = &TypeName`
- `T = TypeName1<TypeName2, ...>`
- `T = (TypeName1, TypeName2, ...)`
- `T = &[TypeName]`
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
    let condition = WhenCondition::try_from(TokenStream2::from(attr)).expect(
        "Failed to parse TokenStream into WhenCondition"
    );

    let mut parts = vec![];
    for c in conditions::get_conjunctions(condition) {
        let impl_body = ImplBody::try_from((TokenStream2::from(item.clone()), Some(c))).expect(
            "Failed to parse TokenStream into ImplBody"
        );

        // TODO: can we somehow get condition and impl_body from cache instead of parsing them again?

        let trait_body = cache
            ::get_trait_by_name(&impl_body.trait_name)
            .expect("Trait not found in cache");

        let specialized_trait = trait_body.apply_impl(&impl_body);

        let trait_token_stream = TokenStream2::from(&specialized_trait);
        let impl_token_stream = TokenStream2::from(&impl_body);

        //TODO: infer generics from conditions (e.g. with condition "T = Type" generic "T" is replaced with type "Type")

        parts.push(quote! {
            #trait_token_stream
            #impl_token_stream
        });
    }

    let combined = quote! { #(#parts)* };
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

    let spec_body = SpecBody::try_from((&impls, &traits, &ann)).expect("Specialization failed");

    TokenStream2::from(&spec_body).into()
}
