use crate::Validator;

#[derive(Debug, Clone)]
pub struct Empty;

impl Validator for Empty {
    fn validate(&mut self, value: &str) -> bool {
        value.is_empty()
    }
}
