use crate::error::Error;
use crate::generators::CodeGen;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::sync::LazyLock;

/// Generates the UNICODE version variable used to generate
/// the library.
pub struct UnicodeVersionGen {
    version: String,
}

impl UnicodeVersionGen {
    /// Creates a new generator for the Unicode version to generate tables
    pub fn new(version: &str) -> Self {
        Self {
            version: String::from(version),
        }
    }
}

fn get_version(version: &str) -> Result<(u64, u64, u64), Error> {
    static VERSION_RX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"([0-9]+).([0-9]+).([0-9]+)").unwrap());

    let caps = match VERSION_RX.captures(version) {
        Some(c) => c,
        None => return err!("Failed to find version in '{}'", version),
    };

    let capture_to_num = |n| {
        caps.get(n)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .map_err(|_e| Error {
                mesg: format!("Failed to parse version from '{:?}'", version),
                line: Some(0),
                path: None,
            })
    };
    let major = capture_to_num(1)?;
    let minor = capture_to_num(2)?;
    let patch = capture_to_num(3)?;

    Ok((major, minor, patch))
}

impl CodeGen for UnicodeVersionGen {
    fn generate_code(&mut self, file: &mut File) -> Result<(), Error> {
        let (major, minor, patch) = get_version(&self.version)?;
        writeln!(
            file,
            "/// The [Unicode version](http://www.unicode.org/versions) of data"
        )?;
        writeln!(
            file,
            "pub const UNICODE_VERSION: (u8, u8, u8) = ({}, {}, {});",
            major, minor, patch
        )?;
        Ok(writeln!(file)?)
    }
}
