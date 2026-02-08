#![no_main]

use libfuzzer_sys::fuzz_target;
use precis_core::{FreeformClass, StringClass};

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string (fuzzer generates arbitrary bytes)
    if let Ok(s) = std::str::from_utf8(data) {
        // Create FreeformClass instance
        let freeform = FreeformClass::default();

        // Test allows() method
        // This should never panic, only return Ok or Err
        let _ = freeform.allows(s);
    }
});
