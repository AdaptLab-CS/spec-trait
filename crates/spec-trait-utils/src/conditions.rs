use proc_macro2::TokenStream;
use serde::{ Deserialize, Serialize };
use std::fmt::Debug;
use syn::{ Error, Type, Ident, Token };
use syn::parse::{ Parse, ParseStream };
use quote::quote;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
pub enum WhenCondition {
    Type(String /* generic */, String /* type */),
    Trait(String /* generic */, Vec<String> /* traits */),
    All(Vec<WhenCondition>),
    Any(Vec<WhenCondition>),
    Not(Box<WhenCondition>),
}

impl TryFrom<TokenStream> for WhenCondition {
    type Error = syn::Error;

    fn try_from(tokens: TokenStream) -> Result<Self, Self::Error> {
        let parsed_condition = syn::parse2(tokens)?;
        Ok(normalize(&parsed_condition))
    }
}

impl Parse for WhenCondition {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let ident = input.parse::<Ident>()?;
        let ident_str = ident.to_string();

        match ident_str.as_str() {
            "all" | "any" | "not" => parse_aggregation(ident, input),
            _ => parse_type_or_trait(ident, input),
        }
    }
}

fn parse_type_or_trait(ident: Ident, input: ParseStream) -> Result<WhenCondition, Error> {
    if input.peek(Token![=]) {
        parse_type(ident, input)
    } else if input.peek(Token![:]) {
        parse_trait(ident, input)
    } else {
        Err(Error::new(ident.span(), "Expected ':' or '=' after identifier"))
    }
}

fn parse_type(ident: Ident, input: ParseStream) -> Result<WhenCondition, Error> {
    input.parse::<Token![=]>()?; // consume the '=' token
    let type_name = input.parse::<Type>()?;
    Ok(WhenCondition::Type(ident.to_string(), quote!(#type_name).to_string()))
}

fn parse_trait(ident: Ident, input: ParseStream) -> Result<WhenCondition, Error> {
    input.parse::<Token![:]>()?; // consume the ':' token
    let traits = input.parse_terminated(Ident::parse, Token![+])?;

    if traits.is_empty() {
        return Err(Error::new(ident.span(), "Expected at least one trait after ':'"));
    }

    let traits = traits
        .into_iter()
        .map(|t| t.to_string())
        .collect();
    Ok(WhenCondition::Trait(ident.to_string(), traits))
}

fn parse_aggregation(ident: Ident, input: ParseStream) -> Result<WhenCondition, Error> {
    let content;
    syn::parenthesized!(content in input); // consume the '(' and ')' token pair

    let conditions = content
        .parse_terminated(WhenCondition::parse, Token![,])?
        .into_iter()
        .collect();

    match ident.to_string().as_str() {
        "all" => Ok(WhenCondition::All(conditions)),
        "any" => Ok(WhenCondition::Any(conditions)),
        "not" => {
            if conditions.len() != 1 {
                return Err(Error::new(ident.span(), "`not` must have exactly one argument"));
            }
            Ok(WhenCondition::Not(Box::new(conditions.into_iter().next().unwrap())))
        }
        _ => Err(Error::new(ident.span(), format!("Unknown aggregation function: {}", ident))),
    }
}

fn normalize(condition: &WhenCondition) -> WhenCondition {
    let mut current = condition.clone();
    loop {
        let next = to_dnf(&current);
        if next == current {
            return current;
        }
        current = next;
    }
}

fn to_dnf(condition: &WhenCondition) -> WhenCondition {
    match condition {
        WhenCondition::All(inner) => all_to_dnf(inner),
        WhenCondition::Any(inner) => any_to_dnf(inner),
        WhenCondition::Not(inner) => not_to_dnf(inner),
        // type and trait conditions are already in dnf
        _ => condition.clone(),
    }
}

fn all_to_dnf(conditions: &Vec<WhenCondition>) -> WhenCondition {
    // outer vec = or, inner vec = and
    let mut dnf = vec![vec![]];

    for cond in conditions {
        let cond_dnf = match to_dnf(cond) {
            WhenCondition::Any(inner) => inner,
            other => vec![other],
        };

        // A and (B or C) -> (A and B) or (A and C)
        dnf = dnf
            .iter()
            .flat_map(|existing| {
                cond_dnf.iter().map(move |c| [existing.clone(), vec![c.clone()]].concat())
            })
            .collect();
    }

    WhenCondition::Any(dnf.into_iter().map(WhenCondition::All).collect())
}

fn any_to_dnf(conditions: &[WhenCondition]) -> WhenCondition {
    WhenCondition::Any(
        conditions
            .iter()
            .map(to_dnf)
            .flat_map(|cond| {
                match cond {
                    // A or (B or C) -> A or B or C
                    WhenCondition::Any(inner) => inner,
                    // A or B -> A or B
                    other => vec![other],
                }
            })
            .collect()
    )
}

fn not_to_dnf(condition: &WhenCondition) -> WhenCondition {
    match condition {
        // not(A and B) -> not(A) or not(B)
        WhenCondition::All(inner) => {
            let negated = inner.iter().cloned().map(Box::new).map(WhenCondition::Not).collect();
            to_dnf(&WhenCondition::Any(negated))
        }
        // not(A or B) -> not(A) and not(B)
        WhenCondition::Any(inner) => {
            let negated = inner.iter().cloned().map(Box::new).map(WhenCondition::Not).collect();
            to_dnf(&WhenCondition::All(negated))
        }
        // not(not(A)) -> A
        WhenCondition::Not(inner) => to_dnf(inner),
        // not(A) -> not(A)
        _ => WhenCondition::Not(Box::new(to_dnf(condition))),
    }
}
