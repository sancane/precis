// build.rs
use precis_tools::{BidiClassGen, MappingTablesGen, SpaceSeparatorGen, UnicodeVersionGen};
use std::env;
use std::path::Path;

const UNICODE_VERSION: &str = "14.0.0";

fn generate_code(ucd: &Path, out: &Path) {
    MappingTablesGen::generate_tables(ucd, out, "profile_tables.rs").unwrap();
    BidiClassGen::generate_file(ucd, out, "bidi_class.rs").unwrap();
    SpaceSeparatorGen::generate_tables(ucd, out, "space_separator.rs").unwrap();
    UnicodeVersionGen::generate_code(out, UNICODE_VERSION, "unicode_version.rs").unwrap();
}

#[cfg(feature = "networking")]
mod download_ucd {

    use crate::*;
    use std::fs;

    pub fn create_dir(path: &Path) {
        if !path.is_dir() {
            fs::create_dir(&path).unwrap();
        }
    }
}

#[cfg(feature = "networking")]
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let ucd_path = Path::new(&out_dir).join("ucd");

    download_ucd::create_dir(&ucd_path);

    precis_tools::download::get_ucd_file(UNICODE_VERSION, &ucd_path, "UnicodeData.txt").unwrap();

    generate_code(&ucd_path, &out_path);

    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(not(feature = "networking"))]
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    let base_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let ucd_path = Path::new(&base_dir).join("resources/ucd");

    generate_code(&ucd_path, &out_path);

    println!("cargo:rerun-if-changed=build.rs");
}
