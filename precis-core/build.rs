// build.rs
use precis_tools::{CodeGenerator, UnicodeVersionGen};
use std::env;
use std::path::Path;

const UNICODE_VERSION: &str = "6.3.0";

fn generate_code(ucd: &Path, out: &Path) {
    let gen = CodeGenerator::new(ucd);
    gen.generate_definitions(out, "precis_defs.rs");
    gen.generate_code(out, "precis_tables.rs");

    UnicodeVersionGen::generate_code(out, UNICODE_VERSION, "unicode_version.rs").unwrap();
}

#[cfg(feature = "networking")]
mod networking {

    use crate::*;
    use std::fs;

    fn create_dir(path: &Path) {
        if !path.is_dir() {
            fs::create_dir(&path).unwrap();
        }
    }

    pub fn download_files(out: &Path) {
        let ucd_path = Path::new(&out).join("ucd");

        create_dir(&ucd_path);

        let csv_path = Path::new(&out).join("csv");
        create_dir(&csv_path);

        precis_tools::download::get_ucd_file(UNICODE_VERSION, &ucd_path, "UnicodeData.txt")
            .unwrap();

        // JoinControl (H)
        // Noncharacter_Code_Point
        precis_tools::download::get_ucd_file(UNICODE_VERSION, &ucd_path, "PropList.txt").unwrap();
        // 9.9.  OldHangulJamo (I)
        precis_tools::download::get_ucd_file(UNICODE_VERSION, &ucd_path, "HangulSyllableType.txt")
            .unwrap();

        // Default_Ignorable_Code_Point
        precis_tools::download::get_ucd_file(
            UNICODE_VERSION,
            &ucd_path,
            "DerivedCoreProperties.txt",
        )
        .unwrap();

        // for long value aliases for General_Category values
        // Used to generate function names
        precis_tools::download::get_ucd_file(
            UNICODE_VERSION,
            &ucd_path,
            "PropertyValueAliases.txt",
        )
        .unwrap();

        // Required for context rules
        precis_tools::download::get_ucd_file(UNICODE_VERSION, &ucd_path, "Scripts.txt").unwrap();

        let extracted_path = ucd_path.join("extracted");
        create_dir(&extracted_path);
        precis_tools::download::get_ucd_file(
            UNICODE_VERSION,
            &ucd_path,
            "extracted/DerivedJoiningType.txt",
        )
        .unwrap();

        precis_tools::download::get_csv_file(UNICODE_VERSION, &csv_path).unwrap();
    }
}

#[cfg(feature = "networking")]
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let ucd_path = Path::new(&out_path).join("ucd");

    networking::download_files(&out_path);
    generate_code(&ucd_path, &out_path);

    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(not(feature = "networking"))]
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    let base_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let ucd_path = Path::new(&base_dir).join("resources/ucd");

    generate_code(&ucd_path, out_path);

    println!("cargo:rerun-if-changed=build.rs");
}
