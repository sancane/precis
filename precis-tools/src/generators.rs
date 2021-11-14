use crate::error::Error;
use crate::file_writer;
use std::fs::File;
use std::path::Path;

pub mod ascii7;
pub mod backward_compatible;
pub mod bidi_class;
pub mod codepoints;
pub mod derived_property;
pub mod exceptions;
pub mod ucd_generator;
pub mod unicode_version;

/// This is the main code generator element. It aggregates other
/// [`CodeGen`] elements. The resulting file will contain the
/// code generated by every element added to the code generator.
pub struct RustCodeGen {
    file: File,
    generators: Vec<Box<dyn CodeGen>>,
}

impl RustCodeGen {
    /// Creates a new Rust code generator
    /// # Arguments:
    /// * `filename` - The file name
    /// # Returns:
    /// This method returns a new [`RustCodeGen`] instance if no errors
    /// occurs when the file is created
    pub fn new<P>(filename: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            file: File::create(filename)?,
            generators: Vec::new(),
        })
    }

    /// Adds an element to the code generator. Each element must implement
    /// [`CodeGen`] trait.
    /// # Arguments:
    /// * `gen` - The code generator element
    pub fn add(&mut self, gen: Box<dyn CodeGen>) {
        self.generators.push(gen);
    }

    /// Write the code into the file created on construction. In case of
    /// error, the file generated could stay in an incomplete or
    /// inconsistent state.
    pub fn generate_code(&mut self) -> Result<(), Error> {
        file_writer::generate_file_header(&mut self.file)?;
        let it = self.generators.iter_mut();
        for gen in it {
            gen.generate_code(&mut self.file)?;
        }
        Ok(())
    }
}

/// Trait implemented by all elements which are able to generate code.
pub trait CodeGen {
    /// Writes the Rust code itself.
    /// # Arguments:
    /// * `file` - The output file
    fn generate_code(&mut self, file: &mut File) -> Result<(), Error>;
}