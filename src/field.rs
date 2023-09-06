use crate::{DataType, Validator};

#[derive(Default, Debug)]
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
            .retain_mut(|data_type| data_type.validate(value))
    }
}
