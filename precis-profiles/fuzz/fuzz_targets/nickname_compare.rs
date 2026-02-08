#![no_main]

use libfuzzer_sys::fuzz_target;
use precis_core::profile::PrecisFastInvocation;
use precis_profiles::Nickname;

fuzz_target!(|data: &[u8]| {
    // Split input into two strings to test compare
    if data.len() < 2 {
        return;
    }

    let split_point = data.len() / 2;
    let s1 = &data[..split_point];
    let s2 = &data[split_point..];

    // Convert both parts to strings
    if let (Ok(str1), Ok(str2)) = (std::str::from_utf8(s1), std::str::from_utf8(s2)) {
        // Try to compare nicknames
        // This should never panic, only return Ok or Err
        let _ = Nickname::compare(str1, str2);
    }
});
