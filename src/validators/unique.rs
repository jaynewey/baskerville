use std::{collections::HashSet, sync::{RwLock, PoisonError, RwLockWriteGuard, Arc}};

use crate::Validator;

use super::ValidationError;

#[derive(Default, Debug)]
pub struct Unique {
    values: Arc<RwLock<HashSet<String>>>,
}

impl Clone for Unique {
    fn clone(&self) -> Self {
        Unique {
            values: self.values.clone(),
        }
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, HashSet<String>>>> for ValidationError {
    fn from(err: PoisonError<RwLockWriteGuard<HashSet<String>>>) -> Self {
        ValidationError { details: err.to_string() }
    }
}

impl Validator for Unique {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        let mut values = self.values.write().map_err(ValidationError::from)?;
        if values.insert(value.to_string()) {
            Ok(())
        } else {
            Err(ValidationError { details: "value has been seen before".to_string() })
        }
    }

    fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
        self.validate(value)
    }
}

#[cfg(test)]
mod test {
    use crate::{Unique, Validator};

    #[test]
    fn id() {
        let validator = Unique::default();
        assert!(validator.validate("Ferris").is_ok());
        assert!(validator.validate("Corro").is_ok());
        assert!(validator.validate("Ferris").is_err());
    }
}
