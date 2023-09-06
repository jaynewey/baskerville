use crate::Validator;

#[derive(Default, Debug, Clone)]
pub struct Text {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Validator for Text {
    fn validate(&mut self, value: &str) -> bool {
        self.min_length = self.min_length.map_or(Some(value.len()), |min| {
            Some(std::cmp::min(min, value.len()))
        });
        self.max_length = self.max_length.map_or(Some(value.len()), |max| {
            Some(std::cmp::max(max, value.len()))
        });
        true
    }
}

/// Validates on literal values provided at creation.
/// For example, you could match on the values "True" and "False" to implement
/// a boolean type.
#[derive(Debug, Clone)]
pub struct Literal {
    // TODO: can we make this &[&str] while still exposing to PyLiteral?
    pub values: Vec<String>,
}

impl Literal {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }
}

impl Validator for Literal {
    fn validate(&mut self, value: &str) -> bool {
        self.values.contains(&value.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::{Literal, Text, Validator};

    #[test]
    fn text() {
        let mut validator = Text::default();
        assert!(validator.validate("Ferris"));
        assert!(validator.validate("🦀"));
        assert_eq!(Some(4), validator.min_length);
        assert_eq!(Some(6), validator.max_length);
    }

    #[test]
    fn literal() {
        let mut validator = Literal::new(vec!["Ferris".into(), "Corro".into()]);
        assert!(validator.validate("Ferris"));
        assert!(validator.validate("Corro"));
        assert!(!validator.validate("Duke"));
    }
}
