use std::collections::HashSet;

use crate::Validator;

#[derive(Default, Debug, Clone)]
pub struct Unique {
    values: HashSet<String>,
}

impl Validator for Unique {
    fn validate(&mut self, value: &str) -> bool {
        self.values.insert(value.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::{Unique, Validator};

    #[test]
    fn id() {
        let mut validator = Unique::default();
        assert!(validator.validate("Ferris"));
        assert!(validator.validate("Corro"));
        assert!(!validator.validate("Ferris"));
    }
}
