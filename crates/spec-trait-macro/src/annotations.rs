use crate::cache::Impl;
use core::panic;
use proc_macro::{TokenStream, TokenTree};
use std::{fmt::Debug, iter::Peekable};

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
    pub raw_call: String,
    pub annotations: Vec<Annotation>,
}

pub fn parse(attr: TokenStream) -> AnnotationBody {
    let mut tokens = attr.into_iter().peekable();
    parse_tokens(&mut tokens)
}

fn parse_tokens(tokens: &mut Peekable<impl Iterator<Item = TokenTree>>) -> AnnotationBody {
    let mut call = String::new();
    let mut annotations = Vec::new();
    let mut current_segment = String::new();

    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Punct(punct) if punct.as_char() == ';' => {
                if call.is_empty() {
                    call = current_segment.trim().to_string();
                } else {
                    annotations.push(parse_annotation(&current_segment));
                }
                current_segment.clear();
            }
            _ => {
                current_segment.push_str(&token.to_string());
            }
        }
    }

    if !current_segment.is_empty() {
        if call.is_empty() {
            call = current_segment.trim().to_string();
        } else {
            annotations.push(parse_annotation(&current_segment));
        }
    }

    let (var, fn_, args) = parse_call(&call);

    AnnotationBody {
        var,
        fn_,
        args,
        raw_call: call,
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
        let traits = traits.split('+').map(|s| s.trim().to_string()).collect();
        Annotation::Trait(param.trim().to_string(), traits)
    } else if let Some((param, ty)) = segment.split_once('=') {
        Annotation::Alias(param.trim().to_string(), ty.trim().to_string())
    } else {
        panic!("Invalid annotation format: {}", segment);
    }
}

pub fn get_most_specific_impl(ann: &AnnotationBody, impls: &Vec<Impl>) -> Impl {
    panic!("Not implemented yet");
}
