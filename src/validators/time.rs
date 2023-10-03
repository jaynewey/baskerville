use crate::Validator;
use chrono::prelude::DateTime as ChronoDateTime;
use chrono::prelude::{NaiveDate, NaiveDateTime, NaiveTime};

use super::ValidationError;

#[derive(Debug, Clone)]
pub struct Date {
    pub formats: Vec<String>,
}

impl Default for Date {
    fn default() -> Self {
        Date {
            formats: vec![
                // Common (-ish) formats
                "%Y-%m-%d".into(),
                "%d-%m-%Y".into(),
                "%d/%m/%Y".into(),
                "%m/%d/%Y".into(),
                "%d/%m/%y".into(),
                "%m/%d/%y".into(),
            ],
        }
    }
}

impl Validator for Date {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        if self.formats.iter().any(|format| NaiveDate::parse_from_str(value, format).is_ok()) {
            Ok(())
        } else {
            Err(ValidationError { details: format!("value does not match any of {:?}", self.formats) })
        }
    }

    fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
        self.formats
            .retain(|format| NaiveDate::parse_from_str(value, format).is_ok());
        if self.formats.is_empty() {
            Err(ValidationError { details: format!("value does not match formats seen: {:?}", self.formats) })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Time {
    pub formats: Vec<String>,
}

impl Default for Time {
    fn default() -> Self {
        Time {
            formats: vec![
                // Common (-ish) formats
                "T%H:%M:%S".into(),
                "%H:%M:%S".into(),
                "%H:%M".into(),
                "%I:%M%p".into(),
            ],
        }
    }
}

impl Validator for Time {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        if self.formats.iter().any(|format| NaiveTime::parse_from_str(value, format).is_ok()) {
            Ok(())
        } else {
            Err(ValidationError { details: format!("value does not match any of {:?}", self.formats) })
        }
    }

    fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
        self.formats
            .retain(|format| NaiveTime::parse_from_str(value, format).is_ok());
        if self.formats.is_empty() {
            Err(ValidationError { details: format!("value does not match formats seen: {:?}", self.formats) })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct DateTime {
    // TODO: can we make this &[DateTimeFormat] while still exposing to PyDateTime?
    pub formats: Vec<DateTimeFormat>,
    // TODO: Earliest / Latest datetime seen.
}

impl DateTime {
    pub fn new(formats: Vec<DateTimeFormat>) -> Self {
        DateTime { formats }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateTimeFormat {
    RFC2822,
    RFC3339,
    Strftime(String),
    Unix,
}

impl Default for DateTime {
    fn default() -> Self {
        DateTime {
            formats: vec![DateTimeFormat::RFC2822, DateTimeFormat::RFC3339],
        }
    }
}

impl DateTime {
    fn is_formatted(&self, format: &DateTimeFormat, value: &str) -> bool {
        match format {
            DateTimeFormat::RFC2822 => ChronoDateTime::parse_from_rfc2822(value).is_ok(),
            DateTimeFormat::RFC3339 => ChronoDateTime::parse_from_rfc3339(value).is_ok(),
            DateTimeFormat::Strftime(strftime) => {
                NaiveDateTime::parse_from_str(value, strftime).is_ok()
            }
            DateTimeFormat::Unix => value
                .parse::<i64>()
                .map(|timestamp| NaiveDateTime::from_timestamp_opt(timestamp, 0))
                .ok()
                .flatten()
                .is_some(),
        }
    }
}

impl Validator for DateTime {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        if self.formats.iter().any(|format| self.is_formatted(format, value)) {
            Ok(())
        } else {
            Err(ValidationError { details: format!("value does not match any of {:?}", self.formats.clone()) })
        }
    }

    fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
        self.formats = self.formats.iter().cloned()
            .filter(|format| self.is_formatted(format, value)).collect();
        if self.formats.is_empty() {
            Err(ValidationError { details: format!("value does not match formats seen: {:?}", self.formats.clone()) })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::validators::time::DateTimeFormat;
    use crate::Validator;
    use crate::{Date, DateTime, Time};

    #[test]
    fn date() {
        let mut validator = Date::default();
        assert!(validator.consider("2001-01-22").is_ok());
        assert_eq!(1, validator.formats.len());
        assert_eq!("%Y-%m-%d", validator.formats[0]);
        assert!(validator.consider("22/01/2001").is_err());

        let mut validator = Date {
            formats: vec!["%Y %m %d".into()],
        };
        assert!(validator.consider("2001 01 22").is_ok());
        assert!(validator.consider("2001-01-22").is_err());
    }

    #[test]
    fn time() {
        let mut validator = Time::default();
        assert!(validator.consider("12:34:56").is_ok());
        assert_eq!(1, validator.formats.len());
        assert_eq!("%H:%M:%S", validator.formats[0]);
        assert!(validator.consider("12:34PM").is_err());

        let mut validator = Time {
            formats: vec!["T%H:%M".into()],
        };
        assert!(validator.consider("T12:34").is_ok());
        assert!(validator.consider("12:34PM").is_err());
    }

    #[test]
    fn date_time() {
        let mut validator = DateTime::default();
        assert!(validator.consider("2001-01-22T00:00:00+00:00").is_ok());
        assert!(validator.consider("2001-01-22T00:00:00Z").is_ok());
        assert_eq!(1, validator.formats.len());
        assert_eq!(DateTimeFormat::RFC3339, validator.formats[0]);
        assert!(validator.consider("Mon, 22 Jan 2001 00:00:00 GMT").is_err());

        let mut validator = DateTime::default();
        assert!(validator.consider("Mon, 22 Jan 2001 00:00:00 GMT").is_ok());
        assert_eq!(1, validator.formats.len());
        assert_eq!(DateTimeFormat::RFC2822, validator.formats[0]);
        assert!(validator.consider("2001-01-22T00:00:00+00:00").is_err());

        let mut validator = DateTime {
            formats: vec![DateTimeFormat::Unix],
        };
        assert!(validator.consider("980121600").is_ok());
        assert!(validator.consider("2001-01-22T00:00:00+00:00").is_err());
    }
}
