pub mod errors;
pub mod format_mol;
pub mod format_pdb;
mod line_reader;
mod utils;

pub use errors::FileReadError;
pub use errors::ParseError;
pub use format_mol::read_mol;
pub use format_pdb::read_pdb;
use line_reader::LineReader;