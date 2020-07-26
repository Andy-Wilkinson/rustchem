pub mod errors;
pub mod format_pdb;
pub mod formats_mol;
mod line_reader;
mod utils;

pub use errors::FileReadError;
pub use errors::ParseError;
pub use format_pdb::read_pdb;
pub use formats_mol::read_mol;
use line_reader::LineReader;
