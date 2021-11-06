use crate::error::Error;
use crate::file_writer;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct UnicodeVersionGen {}

impl UnicodeVersionGen {
    pub fn generate_code(out_dir: &Path, version: &str, file_name: &str) -> Result<(), Error> {
        let dest_path = out_dir.join(file_name);
        let mut file = File::create(dest_path).unwrap();

        UnicodeVersionGen::generate_unicode_version(&mut file, version)
    }

    fn get_version(version: &str) -> Result<(u64, u64, u64), Error> {
        lazy_static! {
            static ref VERSION_RX: Regex = Regex::new(r"([0-9]+).([0-9]+).([0-9]+)").unwrap();
        }

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

    fn generate_unicode_version(file: &mut File, version: &str) -> Result<(), Error> {
        let (major, minor, patch) = UnicodeVersionGen::get_version(version)?;
        file_writer::generate_file_header(file)?;

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
