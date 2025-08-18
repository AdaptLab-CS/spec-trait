use core::panic;
use proc_macro::{ TokenStream, TokenTree };
use std::{ fmt::Debug, iter::Peekable };

#[derive(Debug)]
pub enum Annotation {
    Trait(String /* type */, Vec<String> /* traits */),
    Alias(String /* type */, String /* alias */),
}

#[derive(Debug)]
pub struct AnnotationBody {
    pub var: String,
    pub fn_: String,
    pub args: Vec<String>,
    pub var_type: String,
    pub args_types: Vec<String>,
    pub annotations: Vec<Annotation>,
}

pub fn parse(attr: TokenStream) -> AnnotationBody {
    let mut tokens = attr.into_iter().peekable();
    parse_tokens(&mut tokens)
}

fn parse_tokens(tokens: &mut Peekable<impl Iterator<Item = TokenTree>>) -> AnnotationBody {
    let mut segments = Vec::new();
    let mut current = String::new();

    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Punct(ref punct) if punct.as_char() == ';' => {
                segments.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push_str(&token.to_string()),
        }
    }
    if !current.trim().is_empty() {
        segments.push(current.trim().to_string());
    }

    let call = segments.get(0).unwrap_or_else(|| panic!("Method call not found"));
    let (var, fn_, args) = parse_call(&call);

    let var_type = segments
        .get(1)
        .unwrap_or_else(|| panic!("Variable type not found"))
        .clone();

    assert!(
        !var_type.contains(':') && !var_type.contains('='),
        "Invalid variable type format: {}",
        var_type
    );

    let args_types: Vec<_> = segments
        .get(2)
        .map(|s| {
            let s = s.trim();
            if s.starts_with('[') && s.ends_with(']') {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|x| {
                        let x = x.trim();
                        if x.contains(':') || x.contains('=') {
                            panic!("Invalid argument type format: {}", x);
                        }
                        x.to_string()
                    })
                    .filter(|x| !x.is_empty())
                    .collect()
            } else {
                panic!("Invalid arguments types format: {}", s);
            }
        })
        .unwrap_or_default();

    assert!(
        args.len() == args_types.len(),
        "Number of arguments does not match number of argument types"
    );

    let annotations = segments
        .iter()
        .skip(3)
        .filter(|s| !s.is_empty())
        .map(|s| parse_annotation(s))
        .collect();

    AnnotationBody {
        var,
        fn_,
        args,
        var_type,
        args_types,
        annotations,
    }
}

fn parse_call(call: &str) -> (String, String, Vec<String>) {
    if let Some((var_fn, args)) = call.split_once('(') {
        let args = args
            .trim_end_matches(')')
            .split(',')
            .map(|arg| arg.trim().to_string())
            .collect();

        if let Some((var, fn_)) = var_fn.split_once('.') {
            return (var.trim().to_string(), fn_.trim().to_string(), args);
        }
    }

    panic!("Invalid call format: {}", call);
}

fn parse_annotation(segment: &str) -> Annotation {
    if let Some((param, traits)) = segment.split_once(':') {
        let traits = traits
            .split('+')
            .map(|s| s.trim().to_string())
            .collect();
        Annotation::Trait(param.trim().to_string(), traits)
    } else if let Some((param, ty)) = segment.split_once('=') {
        Annotation::Alias(param.trim().to_string(), ty.trim().to_string())
    } else {
        panic!("Invalid annotation format: {}", segment);
    }
}

pub fn get_type_aliases(type_: &str, ann: &Vec<Annotation>) -> Vec<String> {
    ann.iter()
        .filter_map(|a| {
            match a {
                Annotation::Alias(t, alias) if t == type_ => Some(alias.clone()),
                _ => None,
            }
        })
        .collect()
}

pub fn get_type_traits(type_: &str, ann: &Vec<Annotation>) -> Vec<String> {
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
