use proc_macro2::TokenStream;
use serde::{ Deserialize, Serialize };
use std::collections::HashSet;
use std::fmt::Debug;
use syn::{ Error, Type, Ident, Token, parenthesized };
use syn::parse::{ Parse, ParseStream };
use quote::quote;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

        match ident.to_string().as_str() {
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
    input.parse::<Token![:]>()?; // Consume the ':' token

    let mut traits = vec![];

    while !input.is_empty() && !input.peek(Token![,]) {
        traits.push(input.parse::<Ident>()?.to_string());

        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?; // consume the '+' token
        }
    }

    if traits.is_empty() {
        return Err(Error::new(ident.span(), "Expected at least one trait after ':'"));
    }

    Ok(WhenCondition::Trait(ident.to_string(), traits))
}

fn parse_aggregation(ident: Ident, input: ParseStream) -> Result<WhenCondition, Error> {
    let content;
    parenthesized!(content in input); // consume the '(' and ')' token pair

    let mut conditions = vec![];

    while !content.is_empty() {
        conditions.push(content.parse::<WhenCondition>()?);

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?; // consume the ',' token
        }
    }

    if conditions.is_empty() {
        return Err(Error::new(ident.span(), format!("`{}` requires at least one argument", ident)));
    }

    match ident.to_string().as_str() {
        "all" => Ok(WhenCondition::All(conditions)),
        "any" => Ok(WhenCondition::Any(conditions)),
        "not" =>
            match conditions.as_slice() {
                [condition] => Ok(WhenCondition::Not(Box::new(condition.clone()))),
                _ => Err(Error::new(ident.span(), "`not` must have exactly one argument")),
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

    let dnf_conditions = dnf
        .into_iter()
        .map(|inner| flatten_and_deduplicate(inner, WhenCondition::All))
        .collect::<Vec<_>>();

    flatten_and_deduplicate(dnf_conditions, WhenCondition::Any)
}

fn any_to_dnf(conditions: &[WhenCondition]) -> WhenCondition {
    let dnf = conditions
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
        .collect::<Vec<_>>();

    flatten_and_deduplicate(dnf, WhenCondition::Any)
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

// TODO: check deduplication with [all(A, B), all(B, A)]
fn flatten_and_deduplicate(
    conditions: Vec<WhenCondition>,
    wrapper: fn(Vec<WhenCondition>) -> WhenCondition
) -> WhenCondition {
    // remove duplicates
    let mut seen = HashSet::new();
    let unique_conditions = conditions
        .into_iter()
        .filter(|cond| seen.insert(cond.clone()))
        .collect::<Vec<_>>();

    // flatten if there's only one condition
    if unique_conditions.len() == 1 {
        unique_conditions.first().cloned().unwrap()
    } else {
        wrapper(unique_conditions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_type_condition() {
        let input = quote! { T = u32 };
        let condition = WhenCondition::try_from(input).unwrap();
        assert_eq!(condition, WhenCondition::Type("T".into(), "u32".into()));
    }

    #[test]
    fn parse_single_trait_condition() {
        let input = quote! { T: Clone };
        let condition = WhenCondition::try_from(input).unwrap();
        assert_eq!(condition, WhenCondition::Trait("T".into(), vec!["Clone".into()]));
    }

    #[test]
    fn parse_multiple_trait_condition() {
        let input = quote! { T: Clone + Debug };
        let condition = WhenCondition::try_from(input).unwrap();
        assert_eq!(
            condition,
            WhenCondition::Trait("T".into(), vec!["Clone".into(), "Debug".into()])
        );
    }

    #[test]
    fn parse_all_condition() {
        let input = quote! { all(T: Clone, U = u32) };
        let condition = WhenCondition::try_from(input).unwrap();
        assert_eq!(
            condition,
            WhenCondition::All(
                vec![
                    WhenCondition::Trait("T".into(), vec!["Clone".into()]),
                    WhenCondition::Type("U".into(), "u32".into())
                ]
            )
        );
    }

    #[test]
    fn parse_any_condition() {
        let input = quote! { any(U = u32, T: Clone) };
        let condition = WhenCondition::try_from(input).unwrap();
        assert_eq!(
            condition,
            WhenCondition::Any(
                vec![
                    WhenCondition::Type("U".into(), "u32".into()),
                    WhenCondition::Trait("T".into(), vec!["Clone".into()])
                ]
            )
        );
    }

    #[test]
    fn parse_not_condition() {
        let input = quote! { not(T: Clone) };
        let condition = WhenCondition::try_from(input).unwrap();
        assert_eq!(
            condition,
            WhenCondition::Not(Box::new(WhenCondition::Trait("T".into(), vec!["Clone".into()])))
        );
    }

    #[test]
    fn flatten() {
        let inputs = vec![
            quote! { any(T = i32) },
            quote! { any(any(T = i32)) },
            quote! { all(T = i32) },
            quote! { all(all(T = i32)) },
            quote! { not(not(T = i32)) }
        ];

        for input in inputs {
            let condition = WhenCondition::try_from(input).unwrap();
            assert_eq!(condition, WhenCondition::Type("T".into(), "i32".into()));
        }
    }

    #[test]
    fn normalization() {
        let input =
            quote! { any(not(all(T = A, all(T = B, T = C), any(U = D, U = C), not(not(T = A)), all(T = D), any(U = D))), all(T = A, any(T = B, T = C), T = D)) };
        let condition = WhenCondition::try_from(input).unwrap();
        let expected = WhenCondition::Any(
            vec![
                WhenCondition::Not(Box::new(WhenCondition::Type("T".into(), "A".into()))),
                WhenCondition::Not(Box::new(WhenCondition::Type("T".into(), "B".into()))),
                WhenCondition::Not(Box::new(WhenCondition::Type("T".into(), "C".into()))),
                WhenCondition::All(
                    vec![
                        WhenCondition::Not(Box::new(WhenCondition::Type("U".into(), "D".into()))),
                        WhenCondition::Not(Box::new(WhenCondition::Type("U".into(), "C".into())))
                    ]
                ),
                WhenCondition::Not(Box::new(WhenCondition::Type("T".into(), "D".into()))),
                WhenCondition::Not(Box::new(WhenCondition::Type("U".into(), "D".into()))),
                WhenCondition::All(
                    vec![
                        WhenCondition::Type("T".into(), "A".into()),
                        WhenCondition::Type("T".into(), "B".into()),
                        WhenCondition::Type("T".into(), "D".into())
                    ]
                ),
                WhenCondition::All(
                    vec![
                        WhenCondition::Type("T".into(), "A".into()),
                        WhenCondition::Type("T".into(), "C".into()),
                        WhenCondition::Type("T".into(), "D".into())
                    ]
                )
            ]
        );
        assert_eq!(condition, expected);
    }
}
