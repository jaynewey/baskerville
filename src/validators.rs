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

pub trait Validator: std::fmt::Debug {
    fn validate(&mut self, value: &str) -> bool;
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
