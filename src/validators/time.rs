use crate::Validator;
use chrono::prelude::DateTime as ChronoDateTime;
use chrono::prelude::{NaiveDateTime, NaiveDate, NaiveTime};

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
    fn validate(&mut self, value: &str) -> bool {
        self.formats.retain(|format|
            NaiveDate::parse_from_str(value, format).is_ok()
        );
        !self.formats.is_empty()
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
    fn validate(&mut self, value: &str) -> bool {
        self.formats.retain(|format|
            NaiveTime::parse_from_str(value, format).is_ok()
        );
        !self.formats.is_empty()
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

impl Validator for DateTime {
    fn validate(&mut self, value: &str) -> bool {
        self.formats.retain(|format| match format {
            DateTimeFormat::RFC2822 => ChronoDateTime::parse_from_rfc2822(value).is_ok(),
            DateTimeFormat::RFC3339 => ChronoDateTime::parse_from_rfc3339(value).is_ok(),
            DateTimeFormat::Strftime(strftime) => {
                NaiveDateTime::parse_from_str(value, strftime).is_ok()
            },
            DateTimeFormat::Unix => {
                value.parse::<i64>().map(
                    |timestamp| NaiveDateTime::from_timestamp_opt(timestamp, 0)
                ).ok().flatten().is_some()
            }
        });
        !self.formats.is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::validators::time::DateTimeFormat;
    use crate::{Date, Time, DateTime};
    use crate::Validator;

    #[test]
    fn date() {
        let mut validator = Date::default();
        assert!(validator.validate("2001-01-22"));
        assert_eq!(1, validator.formats.len());
        assert_eq!("%Y-%m-%d", validator.formats[0]);
        assert!(!validator.validate("22/01/2001"));

        let mut validator = Date { formats: vec!["%Y %m %d".into()] };
        assert!(validator.validate("2001 01 22"));
        assert!(!validator.validate("2001-01-22"));
    }

    #[test]
    fn time() {
        let mut validator = Time::default();
        assert!(validator.validate("12:34:56"));
        assert_eq!(1, validator.formats.len());
        assert_eq!("%H:%M:%S", validator.formats[0]);
        assert!(!validator.validate("12:34PM"));

        let mut validator = Time { formats: vec!["T%H:%M".into()] };
        assert!(validator.validate("T12:34"));
        assert!(!validator.validate("12:34PM"));
    }

    #[test]
    fn date_time() {
        let mut validator = DateTime::default();
        assert!(validator.validate("2001-01-22T00:00:00+00:00"));
        assert!(validator.validate("2001-01-22T00:00:00Z"));
        assert_eq!(1, validator.formats.len());
        assert_eq!(DateTimeFormat::RFC3339, validator.formats[0]);
        assert!(!validator.validate("Mon, 22 Jan 2001 00:00:00 GMT"));

        let mut validator = DateTime::default();
        assert!(validator.validate("Mon, 22 Jan 2001 00:00:00 GMT"));
        assert_eq!(1, validator.formats.len());
        assert_eq!(DateTimeFormat::RFC2822, validator.formats[0]);
        assert!(!validator.validate("2001-01-22T00:00:00+00:00"));

        let mut validator = DateTime { formats: vec![DateTimeFormat::Unix] };
        assert!(validator.validate("980121600"));
        assert!(!validator.validate("2001-01-22T00:00:00+00:00"));
    }
}
