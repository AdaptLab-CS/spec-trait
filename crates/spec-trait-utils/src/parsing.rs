use syn::{ Error, Ident, Type, Token };
use syn::parse::ParseStream;
use quote::quote;

pub trait ParseTypeOrTrait {
    fn from_type(ident: String, type_name: String) -> Self;
    fn from_trait(ident: String, traits: Vec<String>) -> Self;
}

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
