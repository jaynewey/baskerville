use baskerville_macro::Validator;

#[cfg(feature = "python")]
use pyo3::prelude::*;

pub mod nullable;
pub mod numeric;
pub mod text;
#[cfg(feature = "time")]
pub mod time;
pub mod unique;
#[cfg(feature = "time")]
pub use time::{Date, DateTime, Time};

pub use nullable::Empty;
pub use numeric::{Float, Integer};
pub use text::{Literal, Text};
pub use unique::Unique;

use std::{fmt, error::Error};

#[derive(Debug)]
pub struct ValidationError {
    pub details: String,
}

impl Error for ValidationError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.details)
    }
}



#[derive(Debug)]
pub struct SchemaError {
    pub line_no: usize,
    pub column_no: usize,
    pub details: String,
    formatted: String,
}

impl SchemaError {
    pub fn new(line_no: usize, column_no: usize, details: String) -> Self {
        Self {
            line_no,
            column_no,
            details: details.clone(),
            formatted: format!("Error on line {line_no} in column {column_no}: {details}")
        }
    }
}

impl Error for SchemaError {
    fn description(&self) -> &str {
        &self.formatted
    }
}


impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.formatted)
    }
}


pub trait Validator: std::fmt::Debug {
    fn validate(&self, value: &str) -> Result<(), ValidationError>;
    fn consider(&mut self, value: &str) -> Result<(), ValidationError>;
}

#[cfg(feature = "python")]
impl Validator for PyObject {
    fn validate(&mut self, value: &str) -> bool {
        Python::with_gil(|py| self.call1(py, (value,)).unwrap().extract(py).unwrap())
    }
}

#[derive(Debug, Clone, Validator)]
pub enum DataType {
    Text(Text),
    Integer(Integer),
    Float(Float),
    Empty(Empty),
    Literal(Literal),
    Unique(Unique),
    #[cfg(feature = "time")]
    Date(Date),
    #[cfg(feature = "time")]
    Time(Time),
    #[cfg(feature = "time")]
    DateTime(DateTime),
    #[cfg(feature = "python")]
    Py(PyObject),
}
