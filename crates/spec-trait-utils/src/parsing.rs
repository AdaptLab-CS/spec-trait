use syn::{
    Error,
    GenericParam,
    Generics,
    Ident,
    PredicateLifetime,
    PredicateType,
    Token,
    Type,
    WherePredicate,
};
use syn::parse::ParseStream;
use quote::ToTokens;
use crate::conversions::to_string;

pub trait ParseTypeOrTrait {
    fn from_type(ident: String, type_name: String) -> Self;
    fn from_trait(ident: String, traits: Vec<String>) -> Self;
}

/**
    Parses either a type or a trait based on the next token in the input stream.
    - If it's '=', it parses a type
    - If it's ':', it parses a trait
    - If neither token is found returns an error
 */
pub fn parse_type_or_trait<T: ParseTypeOrTrait>(
    ident: &str,
    input: ParseStream
) -> Result<T, Error> {
    if input.peek(Token![=]) {
        parse_type::<T>(ident, input)
    } else if input.peek(Token![:]) {
        parse_trait::<T>(ident, input)
    } else {
        Err(Error::new(input.span(), "Expected ':' or '=' after identifier"))
    }
}

fn parse_type<T: ParseTypeOrTrait>(ident: &str, input: ParseStream) -> Result<T, Error> {
    input.parse::<Token![=]>()?; // consume the '=' token
    let type_name = input.parse::<Type>()?;
    Ok(T::from_type(ident.to_string(), to_string(&type_name)))
}

fn parse_trait<T: ParseTypeOrTrait>(ident: &str, input: ParseStream) -> Result<T, Error> {
    input.parse::<Token![:]>()?; // Consume the ':' token

    let mut traits = vec![];

    while !input.is_empty() && !input.peek(Token![,]) && !input.peek(Token![;]) {
        traits.push(input.parse::<Ident>()?.to_string());

        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?; // consume the '+' token
        }
    }

    if traits.is_empty() {
        return Err(Error::new(input.span(), "Expected at least one trait after ':'"));
    }

    Ok(T::from_trait(ident.to_string(), traits))
}

/**
    adds the generics in the where clause in the params

    e.g.
    ```ignore
    <T: Clone + Debug> ... where T: Default, U: Copy
    ```
    becomes
    ```ignore
    <T: Clone + Debug + Default, U: Copy>
    ```

*/
pub fn parse_generics(mut generics: Generics) -> Generics {
    let predicates = generics.where_clause
        .as_ref()
        .map(|wc| wc.predicates.clone())
        .unwrap_or_default();

    for predicate in predicates {
        match predicate {
            WherePredicate::Type(predicate) => {
                handle_type_predicate(&predicate, &mut generics);
            }
            WherePredicate::Lifetime(predicate) => {
                handle_lifetime_predicate(&predicate, &mut generics);
            }
            _ => {}
        }
    }

    generics
}

fn handle_type_predicate(predicate: &PredicateType, generics: &mut Generics) {
    let ident = match &predicate.bounded_ty {
        Type::Path(tp) => &tp.path.segments.first().unwrap().ident,
        _ => panic!("Ident not found in bounded type"),
    };

    let param = generics.params
        .iter_mut()
        .find_map(|param| {
            match param {
                GenericParam::Type(tp) if tp.ident == *ident => Some(tp),
                _ => None,
            }
        })
        .expect("Type parameter not found in generics");

    for bound in predicate.bounds.iter().cloned() {
        let bound_str = bound.to_token_stream().to_string();
        if !param.bounds.iter().any(|b| b.to_token_stream().to_string() == bound_str) {
            param.bounds.push(bound);
        }
    }
}

fn handle_lifetime_predicate(predicate: &PredicateLifetime, generics: &mut Generics) {
    let lifetime = &predicate.lifetime;

    let param = generics.params
        .iter_mut()
        .find_map(|param| {
            match param {
                GenericParam::Lifetime(lp) if lp.lifetime == *lifetime => Some(lp),
                _ => None,
            }
        })
        .expect("Lifetime parameter not found in generics");

    for bound in predicate.bounds.iter().cloned() {
        if !param.bounds.iter().any(|b| b == &bound) {
            param.bounds.push(bound);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse::Parse;
    use quote::quote;

    #[derive(Debug, PartialEq)]
    enum MockTypeOrTrait {
        Type(String, String), // (ident, type_name)
        Trait(String, Vec<String>), // (ident, traits)
    }

    impl ParseTypeOrTrait for MockTypeOrTrait {
        fn from_type(ident: String, type_name: String) -> Self {
            MockTypeOrTrait::Type(ident, type_name)
        }

        fn from_trait(ident: String, traits: Vec<String>) -> Self {
            MockTypeOrTrait::Trait(ident, traits)
        }
    }

    impl Parse for MockTypeOrTrait {
        fn parse(input: ParseStream) -> Result<Self, Error> {
            let ident: Ident = input.parse()?;
            parse_type_or_trait(&ident.to_string(), input)
        }
    }

    #[test]
    fn test_parse_type() {
        let input = quote! { MyType = u32 };

        let result: MockTypeOrTrait = syn::parse2(input).unwrap();

        assert_eq!(result, MockTypeOrTrait::Type("MyType".to_string(), "u32".to_string()));
    }

    #[test]
    fn parse_trait_single() {
        let input = quote! { MyType: Clone };
        let result: MockTypeOrTrait = syn::parse2(input).unwrap();

        assert_eq!(result, MockTypeOrTrait::Trait("MyType".to_string(), vec!["Clone".to_string()]));
    }

    #[test]
    fn parse_trait_multiple() {
        let input = quote! { MyType: Clone + Debug };
        let result: MockTypeOrTrait = syn::parse2(input).unwrap();

        assert_eq!(
            result,
            MockTypeOrTrait::Trait(
                "MyType".to_string(),
                vec!["Clone".to_string(), "Debug".to_string()]
            )
        );
    }

    #[test]
    fn parse_trait_empty() {
        let input = quote! { MyType: };
        let result = syn::parse2::<MockTypeOrTrait>(input);

        assert!(result.is_err());
    }

    #[test]
    fn parse_type_empty() {
        let input = quote! { MyType = };
        let result = syn::parse2::<MockTypeOrTrait>(input);

        assert!(result.is_err());
    }

    #[test]
    fn wrong_token() {
        let input = quote! { MyType ? u32 };
        let result = syn::parse2::<MockTypeOrTrait>(input);

        assert!(result.is_err());
    }

    #[test]
    fn parse_generics_trait() {
        let mut generics: Generics = syn::parse2(quote! { <T> }).unwrap();
        generics.where_clause = Some(syn::parse2(quote! { where T: Clone }).unwrap());

        let res = parse_generics(generics);

        assert_eq!(to_string(&res).replace(" ", ""), "<T: Clone>".to_string().replace(" ", ""));
    }

    #[test]
    fn parse_generics_trait_already_present() {
        let mut generics: Generics = syn::parse2(quote! { <T: Clone> }).unwrap();
        generics.where_clause = Some(syn::parse2(quote! { where T: Clone }).unwrap());

        let res = parse_generics(generics);

        assert_eq!(to_string(&res).replace(" ", ""), "<T: Clone>".to_string().replace(" ", ""));
    }

    #[test]
    fn parse_generics_trait_join() {
        let mut generics: Generics = syn::parse2(quote! { <T: Copy> }).unwrap();
        generics.where_clause = Some(syn::parse2(quote! { where T: Clone }).unwrap());

        let res = parse_generics(generics);

        assert_eq!(
            to_string(&res).replace(" ", ""),
            "<T: Copy + Clone>".to_string().replace(" ", "")
        );
    }

    #[test]
    fn parse_generics_lifetime() {
        let mut generics: Generics = syn::parse2(quote! { <'a, 'b> }).unwrap();
        generics.where_clause = Some(syn::parse2(quote! { where 'a: 'b }).unwrap());

        let res = parse_generics(generics);

        assert_eq!(to_string(&res).replace(" ", ""), "<'a: 'b, 'b>".to_string().replace(" ", ""));
    }

    #[test]
    fn parse_generics_lifetime_already_present() {
        let mut generics: Generics = syn::parse2(quote! { <'a: 'b, 'b> }).unwrap();
        generics.where_clause = Some(syn::parse2(quote! { where 'a: 'b }).unwrap());

        let res = parse_generics(generics);

        assert_eq!(to_string(&res).replace(" ", ""), "<'a: 'b, 'b>".to_string().replace(" ", ""));
    }

    #[test]
    fn parse_generics_trait_and_lifetime() {
        let mut generics: Generics = syn::parse2(quote! { <T: Clone, 'a, 'b> }).unwrap();
        generics.where_clause = Some(syn::parse2(quote! { where T: Copy, 'a: 'b }).unwrap());

        let res = parse_generics(generics);

        assert_eq!(
            to_string(&res).replace(" ", ""),
            "<'a: 'b, 'b, T: Clone + Copy,>".to_string().replace(" ", "")
        );
    }
}
