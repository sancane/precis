#![no_main]

use libfuzzer_sys::fuzz_target;
use precis_core::profile::PrecisFastInvocation;
use precis_profiles::UsernameCaseMapped;

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string (fuzzer generates arbitrary bytes)
    if let Ok(s) = std::str::from_utf8(data) {
        // Try to enforce the UsernameCaseMapped profile
        // This should never panic, only return Ok or Err
        let _ = UsernameCaseMapped::enforce(s);
    }
});
