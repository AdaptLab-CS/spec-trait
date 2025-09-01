use proc_macro2::TokenStream;
use spec_trait_utils::conversions::to_string;
use spec_trait_utils::parsing::{ parse_type_or_trait, ParseTypeOrTrait };
use std::fmt::Debug;
use syn::parse::{ Parse, ParseStream };
use syn::{ bracketed, parenthesized, Error, Expr, Ident, Type, Token, token };

#[derive(Debug, PartialEq, Clone)]
pub enum Annotation {
    Trait(String /* type */, Vec<String> /* traits */),
    Alias(String /* type */, String /* alias */),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnnotationBody {
    pub var: String,
    pub fn_: String,
    pub args: Vec<String>,
    pub var_type: String,
    pub args_types: Vec<String>,
    pub annotations: Vec<Annotation>,
}

impl ParseTypeOrTrait for Annotation {
    fn from_type(ident: String, type_name: String) -> Self {
        Annotation::Alias(ident, type_name)
    }

    fn from_trait(ident: String, traits: Vec<String>) -> Self {
        Annotation::Trait(ident, traits)
    }
}

impl Parse for Annotation {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let ident: Ident = input.parse()?;
        parse_type_or_trait(ident, input)
    }
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

        content
            .parse_terminated(Type::parse, Token![,])?
            .iter()
            .map(to_string)
            .collect()
    } else {
        vec![]
    };

    if input.peek(Token![;]) {
        input.parse::<Token![;]>()?; // consume the ';' token
    }

    Ok((var_type.to_string(), args_types))
}

fn parse_annotations(input: ParseStream) -> Result<Vec<Annotation>, Error> {
    input
        .parse_terminated(Annotation::parse, Token![;])
        .map(|annotations| annotations.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

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
    fn arguments_formats() {
        let input =
            quote! { zst.foo(1, vec![2i8], Vec::new(3), x, (4, 5)); ZST; [i32, Vec<i8>, Vec<i32>, &[i32], (i32, i32)] };
        let result = AnnotationBody::try_from(input).unwrap();

        assert_eq!(result.var, "zst");
        assert_eq!(result.fn_, "foo");
        assert_eq!(result.args, vec!["1", "vec ! [2i8]", "Vec :: new (3)", "x", "(4 , 5)"]);
        assert_eq!(result.var_type, "ZST");
        assert_eq!(
            result.args_types,
            vec!["i32", "Vec < i8 >", "Vec < i32 >", "& [i32]", "(i32 , i32)"]
        );
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
