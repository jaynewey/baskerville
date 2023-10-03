use crate::{DataType, Validator};
#[derive(Default, Debug, Clone)]
pub struct Field {
    pub name: Option<String>,
    pub valid_types: Vec<DataType>,
    pub nullable: bool,
}

impl Field {
    pub fn new(name: Option<String>, valid_types: Vec<DataType>) -> Self {
        Field {
            name,
            valid_types,
            nullable: false,
        }
    }

    pub fn new_with_nullable(
        name: Option<String>,
        valid_types: Vec<DataType>,
        nullable: bool,
    ) -> Self {
        Field {
            name,
            valid_types,
            nullable,
        }
    }

    pub fn consider(&mut self, value: &str) {
        self.valid_types
            .retain_mut(|data_type| data_type.consider(value).is_ok())
    }
}

use std::{
    fmt,
    ops::{Deref, DerefMut},
};
use tabled::{builder::Builder, settings::Style};

pub struct Fields(pub Vec<Field>);

impl Deref for Fields {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Fields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = Builder::default();
        builder.set_header(
            self.iter()
                .map(|field| field.name.clone().unwrap_or_default())
                .collect::<Vec<_>>(),
        );
        for i in 0..(self
            .iter()
            .map(|field| field.valid_types.len())
            .max()
            .unwrap_or_default())
        {
            builder.push_record(
                self.iter()
                    .map(|field| {
                        field
                            .valid_types
                            .get(i)
                            .map_or("", |data_type| match data_type {
                                DataType::Text(_) => "Text",
                                DataType::Integer(_) => "Integer",
                                DataType::Float(_) => "Float",
                                DataType::Empty(_) => "Empty",
                                DataType::Literal(_) => "Literal",
                                DataType::Unique(_) => "Unique",
                                DataType::Date(_) => "Date",
                                DataType::Time(_) => "Time",
                                DataType::DateTime(_) => "DateTime",
                                #[cfg(feature = "python")]
                                DataType::Py(_) => "PyObject",
                            })
                    })
                    .collect::<Vec<_>>(),
            );
        }
        let mut table = builder.build();
        table.with(Style::rounded());
        write!(f, "{table}")
    }
}
