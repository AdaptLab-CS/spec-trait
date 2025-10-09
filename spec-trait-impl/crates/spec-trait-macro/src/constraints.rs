use proc_macro2::TokenStream;
use spec_trait_utils::conversions::{str_to_generics, str_to_type_name, to_string};
use spec_trait_utils::parsing::get_generics_types;
use spec_trait_utils::types::{Aliases, replace_type, strip_lifetimes, type_assignable};
use std::cmp::Ordering;
use std::collections::HashMap;
use syn::Type;

/// constraint related to a single generic attribute
#[derive(Debug, Default, Clone)]
pub struct Constraint {
    /// the generics (types and lifetimes) that are present in type_ or not_types
    pub generics: String,
    pub type_: Option<String>,
    pub traits: Vec<String>,
    pub not_types: Vec<String>,
    pub not_traits: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Constraints {
    pub inner: HashMap<String /* type definition (generic) */, Constraint>,
}

impl Ord for Constraint {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_type(self, other)
            .then(cmp_lifetimes(self, other))
            .then(self.traits.len().cmp(&other.traits.len()))
            .then(self.not_types.len().cmp(&other.not_types.len()))
            .then(self.not_traits.len().cmp(&other.not_traits.len()))
    }
}

impl PartialOrd for Constraint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Constraint {}

pub fn cmp_type_or_lifetime(
    this: &Constraint,
    other: &Constraint,
    replace_fn: &dyn Fn(&mut Type, &str),
) -> Ordering {
    fn norm(ty: &Option<String>) -> Option<String> {
        ty.as_ref()
            .and_then(|s| if s == "_" { None } else { Some(s.clone()) })
    }

    let a = norm(&this.type_);
    let b = norm(&other.type_);

    match (&a, &b) {
        // ('Vec<_>', 'Vec<T>')
        (Some(a), Some(b))
            if type_assignable(a, b, &other.generics, &Aliases::default())
                || type_assignable(b, a, &this.generics, &Aliases::default()) =>
        {
            let mut a = str_to_type_name(a);
            let mut b = str_to_type_name(b);

            replace_fn(&mut a, &this.generics);
            replace_fn(&mut b, &other.generics);

            to_string(&a).len().cmp(&to_string(&b).len())
        }
        _ => a.is_some().cmp(&b.is_some()),
    }
}

fn cmp_type(this: &Constraint, other: &Constraint) -> Ordering {
    fn replace_fn(ty: &mut Type, generics: &str) {
        let empty_type = Type::Verbatim(TokenStream::new());

        replace_type(ty, "_", &empty_type);
        strip_lifetimes(ty, &str_to_generics(generics));
        strip_lifetimes(ty, &str_to_generics("<'static>"));
        for g in get_generics_types::<Vec<_>>(generics) {
            replace_type(ty, &g, &empty_type);
        }
    }
    cmp_type_or_lifetime(this, other, &replace_fn)
}

fn cmp_lifetimes(this: &Constraint, other: &Constraint) -> Ordering {
    fn replace_fn(ty: &mut Type, generics: &str) {
        let empty_type = Type::Verbatim(TokenStream::new());

        replace_type(ty, "_", &empty_type);
        strip_lifetimes(ty, &str_to_generics(generics));
        for g in get_generics_types::<Vec<_>>(generics) {
            replace_type(ty, &g, &empty_type);
        }
    }
    cmp_type_or_lifetime(this, other, &replace_fn)
}

impl Ord for Constraints {
    fn cmp(&self, other: &Self) -> Ordering {
        let all_keys: Vec<&String> = {
            let mut keys = self
                .inner
                .keys()
                .chain(other.inner.keys())
                .collect::<Vec<_>>();
            keys.sort();
            keys.dedup();
            keys
        };

        let default = Constraint::default();

        let mut sum = 0;
        for key in all_keys {
            let self_constraint = self.inner.get(key).unwrap_or(&default);
            let other_constraint = other.inner.get(key).unwrap_or(&default);

            let ord = self_constraint.cmp(other_constraint);

            sum += match ord {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
        }

        sum.cmp(&0)
    }
}

impl PartialOrd for Constraints {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Constraints {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Constraints {}

impl FromIterator<(String, Constraint)> for Constraints {
    fn from_iter<I: IntoIterator<Item = (String, Constraint)>>(iter: I) -> Self {
        let mut constraints = Constraints::default();
        for (generic, constraint) in iter {
            constraints.inner.insert(generic, constraint);
        }
        constraints
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_by_type() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: Some("TypeA".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: None,
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 > c2);
        assert!(c2 < c1);

        let c1 = Constraint {
            generics: "".to_string(),
            type_: Some("T".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "<T>".to_string(),
            type_: Some("T".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 > c2);
        assert!(c2 < c1);

        let c1 = Constraint {
            generics: "".to_string(),
            type_: Some("T".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: Some("_".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 > c2);
        assert!(c2 < c1);
    }

    #[test]
    fn ordering_by_lifetime() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: Some("&'static T".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "<'a>".to_string(),
            type_: Some("&'a T".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 > c2);
        assert!(c2 < c1);

        let c1 = Constraint {
            generics: "<'a, 'b>".to_string(),
            type_: Some("&'a T<&'b T>".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "<'c>".to_string(),
            type_: Some("&'c T<&'static T>".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 < c2);
        assert!(c2 > c1);
    }

    #[test]
    fn ordering_by_type_and_lifetime() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: Some("&'static _".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: Some("&TypeA".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 < c2);
        assert!(c2 > c1);
    }

    #[test]
    fn ordering_by_traits() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: None,
            traits: vec!["Trait1".to_string()],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: None,
            traits: vec!["Trait1".to_string(), "Trait2".to_string()],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 < c2);
        assert!(c2 > c1);
    }

    #[test]
    fn ordering_by_type_and_traits() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: Some("TypeA".to_string()),
            traits: vec!["Trait1".to_string()],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: None,
            traits: vec!["Trait1".to_string(), "Trait2".to_string()],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 > c2);
        assert!(c2 < c1);
    }

    #[test]
    fn ordering_by_not_types() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: None,
            traits: vec![],
            not_types: vec!["NotType1".to_string()],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: None,
            traits: vec![],
            not_types: vec!["NotType1".to_string(), "NotType2".to_string()],
            not_traits: vec![],
        };

        assert!(c1 < c2);
        assert!(c2 > c1);
    }

    #[test]
    fn ordering_by_not_traits() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: None,
            traits: vec![],
            not_types: vec![],
            not_traits: vec!["NotTrait1".to_string()],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: None,
            traits: vec![],
            not_types: vec![],
            not_traits: vec!["NotTrait1".to_string(), "NotTrait2".to_string()],
        };

        assert!(c1 < c2);
        assert!(c2 > c1);
    }

    #[test]
    fn equal_constraints() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: Some("TypeA".to_string()),
            traits: vec!["Trait1".to_string()],
            not_types: vec!["NotType1".to_string()],
            not_traits: vec!["NotTrait1".to_string()],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: Some("TypeB".to_string()),
            traits: vec!["Trait2".to_string()],
            not_types: vec!["NotType2".to_string()],
            not_traits: vec!["NotTrait2".to_string()],
        };

        assert_eq!(c1, c2);
        assert!(!(c1 < c2));
        assert!(!(c1 > c2));
    }

    #[test]
    fn ordering_by_type_with_wildcard() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: Some("TypeA<TypeB>".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: Some("TypeA<_>".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 > c2);
        assert!(c2 < c1);
    }

    #[test]
    fn ordering_by_type_with_generics() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: Some("TypeA<TypeB>".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "<T>".to_string(),
            type_: Some("TypeA<T>".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 > c2);
        assert!(c2 < c1);
    }

    #[test]
    fn ordering_by_type_only_wildcard() {
        let c1 = Constraint {
            generics: "".to_string(),
            type_: None,
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            generics: "".to_string(),
            type_: Some("_".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert_eq!(c1, c2);
    }

    #[test]
    fn test_cmp_constraints() {
        let mut c1 = Constraints::default();
        let mut c2 = Constraints::default();

        c1.inner.insert(
            "T".to_string(),
            Constraint {
                generics: "".to_string(),
                type_: Some("TypeA".to_string()),
                traits: vec!["Trait1".to_string()],
                not_types: vec![],
                not_traits: vec![],
            },
        );
        c1.inner.insert(
            "V".to_string(),
            Constraint {
                generics: "".to_string(),
                type_: Some("TypeA".to_string()),
                traits: vec![],
                not_types: vec![],
                not_traits: vec![],
            },
        );
        c2.inner.insert(
            "T".to_string(),
            Constraint {
                generics: "".to_string(),
                type_: Some("TypeB".to_string()),
                traits: vec![],
                not_types: vec![],
                not_traits: vec![],
            },
        );
        c2.inner.insert(
            "U".to_string(),
            Constraint {
                generics: "".to_string(),
                type_: None,
                traits: vec!["Trait2".to_string()],
                not_types: vec![],
                not_traits: vec![],
            },
        );

        assert!(c1 > c2);
        assert!(c2 < c1);
    }
}
