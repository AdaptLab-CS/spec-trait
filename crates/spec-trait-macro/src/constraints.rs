use std::{ cmp::Ordering, collections::HashMap };

/// constraint related to a single generic attribute
#[derive(Debug, Default, Clone)]
pub struct Constraint {
    pub type_: Option<String>,
    pub traits: Vec<String>,
    pub not_types: Vec<String>,
    pub not_traits: Vec<String>,
}

pub type Constraints = HashMap<String /* type definition (generic) */, Constraint>;

impl Ord for Constraint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_
            .is_some()
            .cmp(&other.type_.is_some())
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

pub fn cmp_constraints(this: &Constraints, other: &Constraints) -> Ordering {
    let all_keys: Vec<&String> = {
        let mut keys = this.keys().chain(other.keys()).collect::<Vec<_>>();
        keys.sort();
        keys.dedup();
        keys
    };

    let mut sum = 0;
    for key in all_keys {
        let self_constraint = this.get(key);
        let other_constraint = other.get(key);

        let ord = match (self_constraint, other_constraint) {
            (Some(s), Some(o)) => s.cmp(o),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        };

        sum += match ord {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };
    }

    sum.cmp(&0)
}

impl Constraint {
    /// reverses the constraint, i.e. type_ becomes not_types and viceversa
    pub fn reverse(&self) -> Self {
        if self.not_types.len() > 1 {
            panic!("can't reverse with multiple not_types");
        }
        Constraint {
            type_: self.not_types.first().cloned(),
            traits: self.not_traits.clone(),
            not_types: self.type_.clone().into_iter().collect(),
            not_traits: self.traits.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_by_type() {
        let c1 = Constraint {
            type_: Some("TypeA".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            type_: None,
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 > c2);
        assert!(c2 < c1);
    }

    #[test]
    fn ordering_by_traits() {
        let c1 = Constraint {
            type_: None,
            traits: vec!["Trait1".to_string()],
            not_types: vec![],
            not_traits: vec![],
        };

        let c2 = Constraint {
            type_: None,
            traits: vec!["Trait1".to_string(), "Trait2".to_string()],
            not_types: vec![],
            not_traits: vec![],
        };

        assert!(c1 < c2);
        assert!(c2 > c1);
    }

    #[test]
    fn ordering_by_not_types() {
        let c1 = Constraint {
            type_: None,
            traits: vec![],
            not_types: vec!["NotType1".to_string()],
            not_traits: vec![],
        };

        let c2 = Constraint {
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
            type_: None,
            traits: vec![],
            not_types: vec![],
            not_traits: vec!["NotTrait1".to_string()],
        };

        let c2 = Constraint {
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
            type_: Some("TypeA".to_string()),
            traits: vec!["Trait1".to_string()],
            not_types: vec!["NotType1".to_string()],
            not_traits: vec!["NotTrait1".to_string()],
        };

        let c2 = Constraint {
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
    fn test_cmp_constraints() {
        let mut c1 = Constraints::new();
        let mut c2 = Constraints::new();

        c1.insert("T".to_string(), Constraint {
            type_: Some("TypeA".to_string()),
            traits: vec!["Trait1".to_string()],
            not_types: vec![],
            not_traits: vec![],
        });
        c1.insert("V".to_string(), Constraint {
            type_: Some("TypeA".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        });
        c2.insert("T".to_string(), Constraint {
            type_: Some("TypeB".to_string()),
            traits: vec![],
            not_types: vec![],
            not_traits: vec![],
        });
        c2.insert("U".to_string(), Constraint {
            type_: None,
            traits: vec!["Trait2".to_string()],
            not_types: vec![],
            not_traits: vec![],
        });

        let res = cmp_constraints(&c1, &c2);
        assert_eq!(res, Ordering::Greater);
    }
}
