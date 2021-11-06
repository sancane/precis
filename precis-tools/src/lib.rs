pub use crate::generators::bidi_class::BidiClassGen;
pub use crate::generators::generator::CodeGenerator;
pub use crate::generators::space_separator::SpaceSeparatorGen;
pub use crate::generators::unicode_version::UnicodeVersionGen;
pub use crate::generators::width_mapping::MappingTablesGen;

pub use crate::csv_parser::{
    CsvLineParser, DerivedProperties, DerivedProperty, PrecisDerivedProperty,
};

pub use crate::error::Error;

#[cfg(feature = "networking")]
pub mod download;

macro_rules! err {
    ($($tt:tt)*) => {
        Err(crate::error::Error::parse(format!($($tt)*)))
    }
}

mod common;
mod csv_parser;
mod error;
mod file_writer;
mod generators;
mod ucd_parsers;
