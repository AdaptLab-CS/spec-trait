use syn::{ Error, Ident, Type, Token };
use syn::parse::ParseStream;
use quote::quote;

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
    ident: Ident,
    input: ParseStream
) -> Result<T, Error> {
    if input.peek(Token![=]) {
        parse_type::<T>(ident, input)
    } else if input.peek(Token![:]) {
        parse_trait::<T>(ident, input)
    } else {
        Err(Error::new(ident.span(), "Expected ':' or '=' after identifier"))
    }
}

fn parse_type<T: ParseTypeOrTrait>(ident: Ident, input: ParseStream) -> Result<T, Error> {
    input.parse::<Token![=]>()?; // consume the '=' token
    let type_name = input.parse::<Type>()?;
    Ok(T::from_type(ident.to_string(), quote!(#type_name).to_string()))
}

fn parse_trait<T: ParseTypeOrTrait>(ident: Ident, input: ParseStream) -> Result<T, Error> {
    input.parse::<Token![:]>()?; // Consume the ':' token

    let mut traits = vec![];

    while !input.is_empty() && !input.peek(Token![,]) && !input.peek(Token![;]) {
        traits.push(input.parse::<Ident>()?.to_string());

        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?; // consume the '+' token
        }
    }

    if traits.is_empty() {
        return Err(Error::new(ident.span(), "Expected at least one trait after ':'"));
    }

    Ok(T::from_trait(ident.to_string(), traits))
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse::Parse;

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
            parse_type_or_trait(ident, input)
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
}
