use crate::Validator;

#[derive(Default, Debug, Clone)]
pub struct Integer {
    pub min_value: Option<i128>,
    pub max_value: Option<i128>,
    pub leading_plus: bool,
}

impl Validator for Integer {
    fn validate(&mut self, value: &str) -> bool {
        let parsed = value.parse::<i128>();
        if let Ok(parsed) = parsed {
            self.min_value = self
                .min_value
                .map_or(Some(parsed), |min| Some(std::cmp::min(min, parsed)));
            self.max_value = self
                .max_value
                .map_or(Some(parsed), |max| Some(std::cmp::max(max, parsed)));
            self.leading_plus |= value.starts_with('+');
        }
        parsed.is_ok()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Float {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub leading_plus: bool,
    pub e_notation: bool,
}

impl Validator for Float {
    fn validate(&mut self, value: &str) -> bool {
        let parsed = value.parse::<f64>();
        if let Ok(parsed) = parsed {
            self.min_value = self
                .min_value
                .map_or(Some(parsed), |min| Some(min.min(parsed)));
            self.max_value = self
                .max_value
                .map_or(Some(parsed), |max| Some(max.max(parsed)));
            self.leading_plus |= value.starts_with('+');
            self.e_notation |= value.contains('e');
        }
        parsed.is_ok()
    }
}

#[cfg(test)]
mod test {
    use crate::{Float, Integer, Validator};

    #[test]
    fn integer() {
        let mut validator = Integer::default();
        assert!(validator.validate("2"));
        assert!(validator.validate("1"));
        assert!(!validator.validate("Ferris"));
        assert_eq!(validator.min_value, Some(1));
        assert!(validator.validate("-1"));
        assert_eq!(validator.min_value, Some(-1));
        assert_eq!(validator.max_value, Some(2));
    }

    #[test]
    fn float() {
        let mut validator = Float::default();
        assert!(validator.validate("0.1"));
        assert!(validator.validate("1.1"));
        assert_eq!(validator.min_value, Some(0.1));
        assert_eq!(validator.max_value, Some(1.1));
        assert!(validator.validate("-1"));
        assert_eq!(validator.min_value, Some(-1.0));
        assert!(!validator.validate("Ferris"));
    }
}
