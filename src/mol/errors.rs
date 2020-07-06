use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoleculeError {
    #[error("Unknown atomic number '{0}'")]
    UnknownAtomicNumber(u32),
    #[error("Unknown element symbol '{0}'")]
    UnknownElementSymbol(String),
}

#[derive(Error, Debug)]
pub enum PropertyError {
    #[error("Expected property to be of type {expected_type}, but found {actual_type}")]
    IncorrectType {
        expected_type: String,
        actual_type: String,
    },
}
