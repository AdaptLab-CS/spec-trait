use proc_macro2::TokenStream;
use spec_trait_utils::conversions::to_string;
use std::fmt::Debug;
use syn::parse::{ Parse, ParseStream };
use syn::{ bracketed, parenthesized, Error, Expr, Ident, Token, token, Type };
use quote::quote;

#[derive(Debug, PartialEq)]
pub enum Annotation {
    Trait(String /* type */, Vec<String> /* traits */),
    Alias(String /* type */, String /* alias */),
}

#[derive(Debug, PartialEq)]
pub struct AnnotationBody {
    pub var: String,
    pub fn_: String,
    pub args: Vec<String>,
    pub var_type: String,
    pub args_types: Vec<String>,
    pub annotations: Vec<Annotation>,
}

impl TryFrom<TokenStream> for AnnotationBody {
    type Error = syn::Error;

    fn try_from(tokens: TokenStream) -> Result<Self, Self::Error> {
        syn::parse2(tokens)
    }
}

impl Parse for AnnotationBody {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let (var, fn_, args) = parse_call(input)?;
        let (var_type, args_types) = parse_types(input)?;
        let annotations = parse_annotations(input)?;

        if args.len() != args_types.len() {
            return Err(
                Error::new(
                    input.span(),
                    "Number of arguments does not match number of argument types"
                )
            );
        }

        Ok(AnnotationBody {
            var,
            fn_,
            args,
            var_type,
            args_types,
            annotations,
        })
    }
}

fn parse_call(input: ParseStream) -> Result<(String, String, Vec<String>), Error> {
    let var: Ident = input.parse()?;

    input.parse::<Token![.]>()?; // consume the '.' token

    let fn_: Ident = input.parse()?;

    let content;
    parenthesized!(content in input); // consume the '(' and ')' token pair

    let args = content.parse_terminated(Expr::parse, Token![,])?;

    if input.peek(Token![;]) {
        input.parse::<Token![;]>()?; // consume the ';' token
    }

    Ok((var.to_string(), fn_.to_string(), args.iter().map(to_string).collect()))
}

fn parse_types(input: ParseStream) -> Result<(String, Vec<String>), Error> {
    let var_type: Ident = input.parse()?;

    if input.peek(Token![;]) {
        input.parse::<Token![;]>()?; // consume the ';' token
    }

    let args_types = if input.peek(token::Bracket) {
        let content;
        bracketed!(content in input); // consume the '[' and ']' token pair

        let args = content.parse_terminated(Ident::parse, Token![,])?;

        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?; // consume the ';' token
        }

        args.into_iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        vec![]
    };

    Ok((var_type.to_string(), args_types))
}

fn parse_annotations(input: ParseStream) -> Result<Vec<Annotation>, Error> {
    let mut annotations = vec![];

    while !input.is_empty() {
        let ident: Ident = input.parse()?;
        annotations.push(parse_type_or_trait(ident, input)?);

        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?; // consume the ';' token
        }
    }

    Ok(annotations)
}

fn parse_type_or_trait(ident: Ident, input: ParseStream) -> Result<Annotation, Error> {
    if input.peek(Token![=]) {
        parse_type(ident, input)
    } else if input.peek(Token![:]) {
        parse_trait(ident, input)
    } else {
        Err(Error::new(ident.span(), "Expected ':' or '=' after identifier"))
    }
}

fn parse_type(ident: Ident, input: ParseStream) -> Result<Annotation, Error> {
    input.parse::<Token![=]>()?; // consume the '=' token
    let type_name = input.parse::<Type>()?;
    Ok(Annotation::Alias(ident.to_string(), quote!(#type_name).to_string()))
}

fn parse_trait(ident: Ident, input: ParseStream) -> Result<Annotation, Error> {
    input.parse::<Token![:]>()?; // Consume the ':' token

    let mut traits = vec![];

    while !input.is_empty() && !input.peek(Token![;]) {
        traits.push(input.parse::<Ident>()?.to_string());

        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?; // consume the '+' token
        }
    }

    if traits.is_empty() {
        return Err(Error::new(ident.span(), "Expected at least one trait after ':'"));
    }

    Ok(Annotation::Trait(ident.to_string(), traits))
}

pub fn get_type_aliases(type_: &str, ann: &[Annotation]) -> Vec<String> {
    ann.iter()
        .filter_map(|a| {
            match a {
                Annotation::Alias(t, alias) if t == type_ => Some(alias.clone()),
                _ => None,
            }
        })
        .collect()
}

pub fn get_type_traits(type_: &str, ann: &[Annotation]) -> Vec<String> {
    ann.iter()
        .filter_map(|a| {
            match a {
                Annotation::Trait(t, traits) if t == type_ => Some(traits.clone()),
                _ => None,
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn single_argument() {
        let input = quote! { zst.foo(1u8); ZST; [u8] };
        let result = AnnotationBody::try_from(input).unwrap();

        assert_eq!(result.var, "zst");
        assert_eq!(result.fn_, "foo");
        assert_eq!(result.args, vec!["1u8"]);
        assert_eq!(result.var_type, "ZST");
        assert_eq!(result.args_types, vec!["u8"]);
        assert!(result.annotations.is_empty());
    }

    #[test]
    fn multiple_arguments() {
        let input = quote! { zst.foo(1, 2i8); ZST; [i32, i8] };
        let result = AnnotationBody::try_from(input).unwrap();

        assert_eq!(result.var, "zst");
        assert_eq!(result.fn_, "foo");
        assert_eq!(result.args, vec!["1", "2i8"]);
        assert_eq!(result.var_type, "ZST");
        assert_eq!(result.args_types, vec!["i32", "i8"]);
        assert!(result.annotations.is_empty());
    }

    #[test]
    fn no_arguments() {
        let inputs = vec![quote! { zst.foo(); ZST; [] }, quote! { zst.foo(); ZST }];

        for input in inputs {
            let result = AnnotationBody::try_from(input).unwrap();
            assert_eq!(result.var, "zst");
            assert_eq!(result.fn_, "foo");
            assert!(result.args.is_empty());
            assert_eq!(result.var_type, "ZST");
            assert!(result.args_types.is_empty());
            assert!(result.annotations.is_empty());
        }
    }

    #[test]
    fn annotations() {
        let input =
            quote! { 
            zst.foo(1u8, 2u8); ZST; [u8, u8]; T: Clone + Debug; u32 = MyType;
         };
        let result = AnnotationBody::try_from(input).unwrap();

        assert_eq!(result.var, "zst");
        assert_eq!(result.fn_, "foo");
        assert_eq!(result.args, vec!["1u8", "2u8"]);
        assert_eq!(result.var_type, "ZST");
        assert_eq!(result.args_types, vec!["u8", "u8"]);
        assert_eq!(
            result.annotations,
            vec![
                Annotation::Trait("T".to_string(), vec!["Clone".to_string(), "Debug".to_string()]),
                Annotation::Alias("u32".to_string(), "MyType".to_string())
            ]
        );
    }

    #[test]
    fn invalid_argument_count() {
        let input = quote! { zst.foo(1u8, 2u8); ZST; [u8]; };
        let result = AnnotationBody::try_from(input);

        assert!(result.is_err());
    }

    #[test]
    fn invalid_format() {
        let inputs = vec![
            quote! { zst.foo(1u8, 2u8); ZST; [u8, u8]; T Clone Debug; },
            quote! { zst.foo(1u8, 2u8) }
        ];

        for input in inputs {
            let result = AnnotationBody::try_from(input);
            assert!(result.is_err());
        }
    }
}
