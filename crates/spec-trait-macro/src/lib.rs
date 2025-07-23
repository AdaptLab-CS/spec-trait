use core::panic;
use proc_macro::{TokenStream, TokenTree};
use std::fmt::Debug;
use std::iter::Peekable;

const FOLDER_CACHE: &str = "/tmp";
const FILE_CACHE: &str = "file_cache.cache";

/**
 * attr can be one of these forms:
 * - `T: TraitName`
 * - `T: TraitName1 + TraitName2`
 * - `T = TypeName`
 * - `T = &TypeName`
 * - `all(attr1, attr2, ...)`
 * - `any(attr1, attr2, ...)`
 * - `not(attr)`
 */

#[proc_macro_attribute]
pub fn when(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("\n* when macro *\nattr: {:?}", attr);

    // let expr = parse_macro_input!(attr);
    let cond = parse(attr);
    print!("* resulting condition *: {:?}", cond);

    // Leggi `file_cache.cache` (usando serde) e verifica se le condizioni sono soddisfatte

    item
}

#[derive(Debug)]
enum ComplexCond {
    Simple(TypeOrTrait),
    All(Vec<ComplexCond>),
    Any(Vec<ComplexCond>),
    Not(Box<ComplexCond>),
}

#[derive(Debug)]
enum TypeOrTrait {
    Type(String /* generic parameter */, String /* type */),
    Trait(
        String,      /* generic parameter */
        Vec<String>, /* traits */
    ),
}

fn parse(attr: TokenStream) -> ComplexCond {
    println!("Parsing attribute: {:?}", attr);
    let mut tokens = attr.into_iter().peekable();
    parse_tokens(&mut tokens)
}

fn parse_tokens(tokens: &mut Peekable<impl Iterator<Item = TokenTree>>) -> ComplexCond {
    if let Some(token) = tokens.peek() {
        match token {
            TokenTree::Ident(ident) => {
                let ident_str = ident.to_string();
                tokens.next();

                println!("Parsed identifier: {}", ident_str);

                if ident_str == "all" || ident_str == "any" || ident_str == "not" {
                    if let Some(TokenTree::Group(group)) = tokens.next() {
                        println!("Parsing group for '{}': {:?}", ident_str, group);
                        let group_stream = group.stream();
                        let group_tokens = &mut group_stream.into_iter().peekable();
                        return parse_aggr(&ident_str, group_tokens);
                    } else {
                        panic!("Expected a group after '{}'", ident_str);
                    }
                }

                if let Some(TokenTree::Punct(punct)) = tokens.peek() {
                    match punct.as_char() {
                        ':' => {
                            tokens.next();
                            parse_trait(tokens, ident_str)
                        }
                        '=' => {
                            tokens.next();
                            parse_type(tokens, ident_str)
                        }
                        _ => panic!("Unexpected punctuation: {}", punct),
                    }
                } else {
                    panic!("Expected ':' or '=' after identifier");
                }
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
    } else {
        panic!("Unexpected end of tokens");
    }
}

fn parse_trait(
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
    param: String,
) -> ComplexCond {
    let mut traits = Vec::new();

    while let Some(TokenTree::Ident(ident)) = tokens.peek() {
        traits.push(ident.to_string());
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

    println!("Parsed traits for generic {}: {:?}", param, traits);

    if traits.is_empty() {
        panic!("Expected at least one trait after ':'");
    }

    ComplexCond::Simple(TypeOrTrait::Trait(param, traits))
}

fn parse_type(
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
    param: String,
) -> ComplexCond {
    let mut type_name = String::new();

    if let Some(TokenTree::Punct(punct)) = tokens.peek() {
        if punct.as_char() == '&' {
            tokens.next(); // Consume '&'
            type_name.push('&');
        }
    }

    if let Some(TokenTree::Ident(ident)) = tokens.next() {
        type_name.push_str(&ident.to_string());
        println!("Parsed type for generic {}: {:?}", param, type_name);
        ComplexCond::Simple(TypeOrTrait::Type(param, type_name))
    } else {
        panic!("Expected a type name after '='");
    }
}

fn parse_aggr(
    func_name: &String,
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> ComplexCond {
    let mut args = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Group(inner_group) => {
                let inner_tokens = &mut inner_group.stream().into_iter().peekable();
                args.push(parse_tokens(inner_tokens));
            }
            TokenTree::Ident(_) | TokenTree::Punct(_) => {
                if let TokenTree::Punct(punct) = &token {
                    if punct.as_char() == ',' {
                        // Skip commas
                        continue;
                    }
                }
                let mut inline_tokens = std::iter::once(token).chain(tokens.by_ref()).peekable();
                args.push(parse_tokens(&mut inline_tokens));
            }
            _ => panic!("Unexpected token in aggregation function: {:?}", token),
        }
    }

    println!("Function: {}, Args: {:?}", func_name, args);

    match func_name.as_str() {
        "all" => ComplexCond::All(args),
        "any" => ComplexCond::Any(args),
        "not" => {
            if args.len() != 1 {
                panic!("`not` must have exactly one argument");
            }
            ComplexCond::Not(Box::new(args.into_iter().next().unwrap()))
        }
        _ => panic!("Unknown aggregation function: {}", func_name),
    }
}
