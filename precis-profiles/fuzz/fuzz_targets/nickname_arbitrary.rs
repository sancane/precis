#![no_main]

use libfuzzer_sys::fuzz_target;
use precis_core::profile::PrecisFastInvocation;
use precis_profiles::Nickname;

fuzz_target!(|data: &[u8]| {
    // Allow arbitrary bytes (including invalid UTF-8)
    // Test both valid and invalid UTF-8 sequences
    let s = String::from_utf8_lossy(data);

    // Try enforce - should handle invalid UTF-8 gracefully
    let _ = Nickname::enforce(&s);

    // If enforce succeeds, try prepare as well
    if let Ok(enforced) = Nickname::enforce(&s) {
        let _ = Nickname::prepare(enforced.as_ref());
    }
});
