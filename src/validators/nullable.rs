use crate::Validator;

use super::ValidationError;

#[derive(Debug, Clone)]
pub struct Empty;

impl Validator for Empty {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        if value.is_empty() {
            Ok(())
        } else {
            Err(ValidationError {details: "Non-empty".to_string()})
        }
    }

    fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
        self.validate(value)
    }
}
