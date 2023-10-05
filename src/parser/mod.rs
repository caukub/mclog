// AFAIR some parts are implemented poorly, but it should work without any issues, so I won't touch it for now

pub mod delimiters;
pub mod log_level;
#[allow(clippy::module_inception)]
pub mod parser;

pub use parser::*;
