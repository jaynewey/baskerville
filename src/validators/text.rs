use crate::Validator;

use super::ValidationError;

#[derive(Default, Debug, Clone)]
pub struct Text {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Validator for Text {
    fn validate(&self, _: &str) -> Result<(), ValidationError> {
        Ok(())
    }

    fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
        self.min_length = self.min_length.map_or(Some(value.len()), |min| {
            Some(std::cmp::min(min, value.len()))
        });
        self.max_length = self.max_length.map_or(Some(value.len()), |max| {
            Some(std::cmp::max(max, value.len()))
        });
        self.validate(value)
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
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        if self.values.contains(&value.to_string()) {
            Ok(())
        } else {
            Err(ValidationError { details: format!("value \"{value}\" is not one of {:?}", self.values) })
        }
    }

    fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
        self.validate(value)
    }
}

#[cfg(test)]
mod test {
    use crate::{Literal, Text, Validator};

    #[test]
    fn text() {
        let mut validator = Text::default();
        assert!(validator.consider("Ferris").is_ok());
        assert!(validator.consider("🦀").is_ok());
        assert_eq!(Some(4), validator.min_length);
        assert_eq!(Some(6), validator.max_length);
    }

    #[test]
    fn literal() {
        let mut validator = Literal::new(vec!["Ferris".into(), "Corro".into()]);
        assert!(validator.consider("Ferris").is_ok());
        assert!(validator.consider("Corro").is_ok());
        assert!(validator.consider("Duke").is_err());
    }
}
