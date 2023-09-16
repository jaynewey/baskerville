use std::error::Error;

use csv::{Reader, ReaderBuilder};
pub use csv::{Terminator, Trim};

use crate::{
    field::Fields, DataType, Date, DateTime, Empty, Field, Float, Integer, Text, Time, Validator,
};

pub enum CsvInput<'a> {
    Path(&'a str),
    Value(&'a str),
}

pub struct InferOptions {
    pub data_types: Vec<DataType>,
    pub null_validator: DataType,
    pub has_headers: bool,
    pub flexible: bool,
    pub delimiter: u8,
    pub escape: Option<u8>,
    pub quote: u8,
    pub quoting: bool,
    pub trim: Trim,
    pub terminator: Terminator,
}

impl Default for InferOptions {
    fn default() -> Self {
        Self {
            data_types: vec![
                DataType::Integer(Integer::default()),
                DataType::Float(Float::default()),
                DataType::Text(Text::default()),
                #[cfg(feature = "time")]
                DataType::Date(Date::default()),
                #[cfg(feature = "time")]
                DataType::Time(Time::default()),
                #[cfg(feature = "time")]
                DataType::DateTime(DateTime::default()),
            ],
            null_validator: DataType::Empty(Empty),
            has_headers: false,
            flexible: false,
            delimiter: b',',
            escape: None,
            quote: b'"',
            quoting: true,
            trim: Trim::None,
            terminator: Terminator::CRLF,
        }
    }
}

#[derive(Debug)]
struct ValidationError {
    line_no: usize,
    column_no: usize,
    details: String,
    formatted: String,
}

impl ValidationError {
    fn new(line_no: usize, column_no: usize, details: &str) -> Self {
        Self {
            line_no,
            column_no,
            details: details.into(),
            formatted: format!("Error on line {line_no} in column {column_no}: {details}")
        }
    }
}

impl Error for ValidationError {
    fn description(&self) -> &str {
        &self.formatted
    }
}

use std::fmt;

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.formatted)
    }
}

fn validate_csv_with_reader<R>(
    fields: &mut [Field],
    options: &mut InferOptions,
    reader: &mut Reader<R>,
) -> Result<(), Box<dyn Error>>
where
    R: std::io::Read,
{
    for (line_no, record) in reader.records().enumerate() {
        let record = record?;

        for (column_no, (value, field)) in record.iter().zip(fields.iter_mut()).enumerate() {
            if !field.nullable && options.null_validator.validate(value) {
                return Err(Box::new(ValidationError::new(
                    line_no,
                    column_no,
                    "Nullable value"
                )))
            } else {
                for data_type in field.valid_types.iter_mut() {
                    if !data_type.validate(value) {
                        return Err(Box::new(ValidationError::new(
                            line_no,
                            column_no,
                            "Invalid value"
                        )))
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn validate_csv_with_options(
    fields: &mut [Field],
    input: CsvInput,
    options: &mut InferOptions,
) -> Result<(), Box<dyn Error>> {
    let mut reader_builder = ReaderBuilder::new();
    let reader_builder = reader_builder
        .has_headers(options.has_headers)
        .flexible(options.flexible)
        .delimiter(options.delimiter)
        .quote(options.quote)
        .quoting(options.quoting);

    match input {
        CsvInput::Path(path) => {
            validate_csv_with_reader(fields, options, &mut reader_builder.from_path(path)?)
        }
        CsvInput::Value(value) => {
            let mut reader_builder = reader_builder.from_reader(value.as_bytes());
            validate_csv_with_reader(fields, options, &mut reader_builder)
        }
    }
}


fn infer_csv_with_reader<R>(
    options: &mut InferOptions,
    reader: &mut Reader<R>,
) -> Result<Fields, Box<dyn Error>>
where
    R: std::io::Read,
{
    let mut fields = Fields(
        reader
            .headers()?
            .iter()
            .map(|value| {
                Field::new(
                    if options.has_headers {
                        if options.null_validator.validate(value) {
                            None
                        } else {
                            Some(value.to_string())
                        }
                    } else {
                        None
                    },
                    options.data_types.clone(),
                )
            })
            .collect(),
    );
    for record in reader.records() {
        let record = record?;

        if options.flexible {
            for _ in 0..record.len().saturating_sub(fields.len()) {
                fields.push(Field::new_with_nullable(
                    None,
                    options.data_types.to_owned(),
                    true,
                ))
            }
        }

        for (value, field) in record.iter().zip(fields.iter_mut()) {
            if options.null_validator.validate(value) {
                field.nullable = true
            } else {
                field.consider(value)
            }
        }
    }
    Ok(fields)
}

pub fn infer_csv_with_options(
    input: CsvInput,
    options: &mut InferOptions,
) -> Result<Fields, Box<dyn Error>> {
    let mut reader_builder = ReaderBuilder::new();
    let reader_builder = reader_builder
        .has_headers(options.has_headers)
        .flexible(options.flexible)
        .delimiter(options.delimiter)
        .escape(options.escape)
        .quote(options.quote)
        .quoting(options.quoting)
        .trim(options.trim)
        .terminator(options.terminator);

    match input {
        CsvInput::Path(path) => {
            infer_csv_with_reader(options, &mut reader_builder.from_path(path)?)
        }
        CsvInput::Value(value) => {
            let mut reader_builder = reader_builder.from_reader(value.as_bytes());
            infer_csv_with_reader(options, &mut reader_builder)
        }
    }
}

pub fn infer_csv(input: CsvInput) -> Result<Fields, Box<dyn Error>> {
    infer_csv_with_options(input, &mut InferOptions::default())
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{infer_csv, infer_csv_with_options, CsvInput, DataType, InferOptions};

    #[test]
    fn default() -> Result<(), Box<dyn Error>> {
        let fields = infer_csv(CsvInput::Value(
            "0,a,
1,b,0.1
2,c,0.2",
        ))?;

        assert_eq!(3, fields.len());
        assert_eq!(
            fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>(),
            vec![None, None, None]
        );
        assert!(matches!(fields[0].valid_types[0], DataType::Integer(_)));
        assert!(matches!(fields[0].valid_types[1], DataType::Float(_)));
        assert!(matches!(fields[0].valid_types[2], DataType::Text(_)));
        assert!(fields[2].nullable);
        Ok(())
    }

    #[test]
    fn headers() -> Result<(), Box<dyn Error>> {
        let fields = infer_csv_with_options(
            CsvInput::Value(
                "col1,col2,
0,a,
1,b,0.1
2,c,0.2",
            ),
            &mut InferOptions {
                has_headers: true,
                ..InferOptions::default()
            },
        )?;

        assert_eq!(3, fields.len());
        assert_eq!(
            fields.iter().cloned().map(|f| f.name).collect::<Vec<_>>(),
            vec![Some("col1".into()), Some("col2".into()), None]
        );
        Ok(())
    }
}
