#![no_main]

use libfuzzer_sys::fuzz_target;
use precis_core::{FreeformClass, StringClass};

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string (fuzzer generates arbitrary bytes)
    if let Ok(s) = std::str::from_utf8(data) {
        let freeform = FreeformClass::default();

        // Test get_value_from_char for each character in the string
        for c in s.chars() {
            let _ = freeform.get_value_from_char(c);
        }
    }
});
