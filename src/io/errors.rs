use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileReadError {
    #[error("Error in line {line}")]
    LineParse { source: ParseError, line:usize },

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("'{value}' is not a valid {name}")]
    Parse { name: String, value: String },

    #[error("'{value}' is not a valid {name}")]
    ParseInt { source: std::num::ParseIntError, name: String, value: String },

    #[error("'{value}' is not a valid {name}")]
    ParseFloat { source: std::num::ParseFloatError, name: String, value: String },

    #[error("The line is too short")]
    LineTooShort,

    #[error("{message}")]
    UnexpectedTag { message: String },

    #[error(transparent)]
    MoleculeError(#[from] crate::mol::MoleculeError),
}