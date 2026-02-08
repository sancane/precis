#![no_main]

use libfuzzer_sys::fuzz_target;
use precis_core::{IdentifierClass, StringClass};

fuzz_target!(|data: &[u8]| {
    // Interpret data as array of u32 codepoints
    if data.len() < 4 {
        return;
    }

    let identifier = IdentifierClass::default();

    // Process data in chunks of 4 bytes (u32)
    for chunk in data.chunks(4) {
        if chunk.len() == 4 {
            // Convert 4 bytes to u32 (little endian)
            let codepoint = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);

            // Test get_value_from_codepoint
            // Should handle all codepoint values gracefully (including invalid ones)
            let _ = identifier.get_value_from_codepoint(codepoint);
        }
    }
});
