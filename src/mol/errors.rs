use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoleculeError {
    #[error("Unknown atomic number '{0}'")]
    UnknownAtomicNumber(u32),
    #[error("Unknown element symbol '{0}'")]
    UnknownElementSymbol(String),
}
