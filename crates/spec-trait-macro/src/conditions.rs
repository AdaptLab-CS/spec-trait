use core::panic;
use proc_macro::{TokenStream, TokenTree};
use std::fmt::Debug;
use std::iter::Peekable;

#[derive(Debug)]
pub enum WhenCondition {
    Type(String /* parameter */, String /* type */),
    Trait(String /* parameter */, Vec<String> /* traits */),
    All(Vec<WhenCondition>),
    Any(Vec<WhenCondition>),
    Not(Box<WhenCondition>),
}

pub fn parse(attr: TokenStream) -> WhenCondition {
    let mut tokens = attr.into_iter().peekable();
    parse_tokens(&mut tokens)
}

fn parse_tokens(tokens: &mut Peekable<impl Iterator<Item = TokenTree>>) -> WhenCondition {
    if let Some(token) = tokens.next() {
        match token {
            TokenTree::Ident(ident) => {
                let ident_str = ident.to_string();

                if ident_str == "all" || ident_str == "any" || ident_str == "not" {
                    handle_aggr(ident_str, tokens)
                } else {
                    handle_type_or_trait(ident_str, tokens)
                }
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
    } else {
        panic!("Unexpected end of tokens");
    }
}

fn handle_aggr(
    ident: String,
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> WhenCondition {
    if let Some(TokenTree::Group(group)) = tokens.next() {
        let group_tokens = &mut group.stream().into_iter().peekable();
        parse_aggr(ident, group_tokens)
    } else {
        panic!("Expected a group after `{}`", ident);
    }
}

fn handle_type_or_trait(
    ident: String,
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> WhenCondition {
    if let Some(TokenTree::Punct(punct)) = tokens.next() {
        match punct.as_char() {
            ':' => parse_trait(ident, tokens),
            '=' => parse_type(ident, tokens),
            _ => panic!("Unexpected punctuation: {}", punct),
        }
    } else {
        panic!("Expected ':' or '=' after identifier");
    }
}

fn parse_type(
    ident: String,
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> WhenCondition {
    let mut type_name = String::new();

    if let Some(TokenTree::Punct(punct)) = tokens.peek() {
        if punct.as_char() == '&' {
            tokens.next();
            type_name.push('&');
        }
    }

    if let Some(TokenTree::Ident(name)) = tokens.next() {
        type_name.push_str(&name.to_string());
        WhenCondition::Type(ident, type_name)
    } else {
        panic!("Expected a type name after '='");
    }
}

fn parse_trait(
    ident: String,
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> WhenCondition {
    let mut traits = Vec::new();

    while let Some(TokenTree::Ident(name)) = tokens.peek() {
        traits.push(name.to_string());
        tokens.next();

        if let Some(TokenTree::Punct(punct)) = tokens.peek() {
            if punct.as_char() == '+' {
                tokens.next();
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if traits.is_empty() {
        panic!("Expected at least one trait after ':'");
    }

    WhenCondition::Trait(ident, traits)
}

fn parse_aggr(
    ident: String,
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> WhenCondition {
    let mut args = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Ident(_) => {
                let mut inline_tokens = std::iter::once(token).chain(tokens.by_ref()).peekable();
                args.push(parse_tokens(&mut inline_tokens));
            }
            TokenTree::Punct(punct) => {
                if punct.as_char() != ',' {
                    panic!("Unexpected punctuation: '{}'", punct.to_string());
                }
            }
            _ => panic!("Unexpected token in aggregation function: {:?}", token),
        }
    }

    if args.is_empty() {
        panic!("Expected at least one arg for `{}`", ident);
    }

    match ident.as_str() {
        "all" => WhenCondition::All(args),
        "any" => WhenCondition::Any(args),
        "not" => {
            if args.len() != 1 {
                panic!("`not` must have exactly one argument");
            }
            WhenCondition::Not(Box::new(args.into_iter().next().unwrap()))
        }
        _ => panic!("Unknown aggregation function: {}", ident),
    }
}
