use std::num::{ParseIntError, ParseFloatError};

use crate::Validator;

use super::ValidationError;

#[derive(Default, Debug, Clone)]
pub struct Integer {
    pub min_value: Option<i128>,
    pub max_value: Option<i128>,
    pub leading_plus: bool,
}

impl From<ParseIntError> for ValidationError {
    fn from(err: ParseIntError) -> Self {
        ValidationError { details: err.to_string() }
    }
}

impl Validator for Integer {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        match value.parse::<i128>() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

    fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
        match value.parse::<i128>() {
                Ok(parsed) => {
                self.min_value = self
                    .min_value
                    .map_or(Some(parsed), |min| Some(std::cmp::min(min, parsed)));
                self.max_value = self
                    .max_value
                    .map_or(Some(parsed), |max| Some(std::cmp::max(max, parsed)));
                self.leading_plus |= value.starts_with('+');
                Ok(())
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Float {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub leading_plus: bool,
    pub e_notation: bool,
}

impl From<ParseFloatError> for ValidationError {
    fn from(err: ParseFloatError) -> Self {
        ValidationError { details: err.to_string() }
    }
}

impl Validator for Float {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        match value.parse::<f64>() {
            Ok(parsed) => {
                if self.min_value.is_some_and(|min| parsed < min) {
                    Err(ValidationError {details: "value is smaller".to_string()})
                } else if self.max_value.is_some_and(|max| parsed > max) {
                    Err(ValidationError {details: "value is larger".to_string()})
                } else if !self.leading_plus && value.starts_with('+') {
                    Err(ValidationError {details: "value written in e notation".to_string()})
                } else {
                    Ok(())
                }
            },
            Err(e) => Err(e.into())
        }
    }

    fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
        match value.parse::<f64>() {
            Ok(parsed) => {
                self.min_value = self
                    .min_value
                    .map_or(Some(parsed), |min| Some(min.min(parsed)));
                self.max_value = self
                    .max_value
                    .map_or(Some(parsed), |max| Some(max.max(parsed)));
                self.leading_plus |= value.starts_with('+');
                self.e_notation |= value.contains('e');
                Ok(())
            },
            Err(e) => Err(e.into())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Float, Integer, Validator};

    #[test]
    fn integer() {
        let mut validator = Integer::default();
        assert!(validator.consider("2").is_ok());
        assert!(validator.consider("1").is_ok());
        assert!(validator.consider("Ferris").is_err());
        assert_eq!(validator.min_value, Some(1));
        assert!(validator.consider("-1").is_ok());
        assert_eq!(validator.min_value, Some(-1));
        assert_eq!(validator.max_value, Some(2));
    }

    #[test]
    fn float() {
        let mut validator = Float::default();
        assert!(validator.consider("0.1").is_ok());
        assert!(validator.consider("1.1").is_ok());
        assert_eq!(validator.min_value, Some(0.1));
        assert_eq!(validator.max_value, Some(1.1));
        assert!(validator.consider("-1").is_ok());
        assert_eq!(validator.min_value, Some(-1.0));
        assert!(validator.consider("Ferris").is_err());
    }
}
