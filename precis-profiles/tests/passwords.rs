use precis_core::profile::Profile;
use precis_core::{CodepointInfo, DerivedPropertyValue, Error};
use precis_profiles::OpaqueString;
use std::borrow::Cow;

#[test]
fn prepare() {
    let profile = OpaqueString::new();

    // SPACE `U+0020` is allowed
    let res = profile.prepare("correct horse battery staple");
    assert_eq!(res, Ok(Cow::from("correct horse battery staple")));

    // Differs by case from previous example
    let res = profile.prepare("Correct Horse Battery Staple");
    assert_eq!(res, Ok(Cow::from("Correct Horse Battery Staple")));

    // Non-ASCII letters are OK (e.g., GREEK SMALL LETTER
    // PI `U+03C0`)
    let res = profile.prepare("πßå");
    assert_eq!(res, Ok(Cow::from("πßå")));

    // Symbols are OK (e.g., BLACK DIAMOND SUIT `U+2666`)
    let res = profile.prepare("Jack of ♦s");
    assert_eq!(res, Ok(Cow::from("Jack of ♦s")));

    // Zero-length passwords are disallowed
    let res = profile.prepare("");
    assert_eq!(res, Err(Error::Invalid));

    // Control characters like TAB `U+0009` are disallowed
    let res = profile.prepare("simple;\u{0009} test");
    assert_eq!(
        res,
        Err(Error::BadCodepoint(CodepointInfo::new(
            0x0009,
            7,
            DerivedPropertyValue::Disallowed
        )))
    );
}

#[test]
fn enforce() {
    let profile = OpaqueString::new();

    // SPACE `U+0020` is allowed
    let res = profile.enforce("correct horse battery staple");
    assert_eq!(res, Ok(Cow::from("correct horse battery staple")));

    // Differs by case from previous example
    let res = profile.enforce("Correct Horse Battery Staple");
    assert_eq!(res, Ok(Cow::from("Correct Horse Battery Staple")));

    // Non-ASCII letters are OK (e.g., GREEK SMALL LETTER
    // PI `U+03C0`)
    let res = profile.enforce("πßå");
    assert_eq!(res, Ok(Cow::from("πßå")));

    // Symbols are OK (e.g., BLACK DIAMOND SUIT `U+2666`)
    let res = profile.enforce("Jack of ♦s");
    assert_eq!(res, Ok(Cow::from("Jack of ♦s")));

    // `OGHAM` SPACE MARK `U+1680` is mapped to SPACE `U+0020`;
    // thus, the full string is mapped to <foo bar>
    let res = profile.enforce("foo bar");
    assert_eq!(res, Ok(Cow::from("foo bar")));

    // Zero-length passwords are disallowed
    let res = profile.enforce("");
    assert_eq!(res, Err(Error::Invalid));

    // Control characters like TAB `U+0009` are disallowed
    let res = profile.enforce("simple;\u{0009} test");
    assert_eq!(
        res,
        Err(Error::BadCodepoint(CodepointInfo::new(
            0x0009,
            7,
            DerivedPropertyValue::Disallowed
        )))
    );
}

#[test]
fn compare() {
    let profile = OpaqueString::new();

    let res = profile.compare("𝄞💝♦💣東💯 Secret", "");
    assert_eq!(res, Err(Error::Invalid));

    let res = profile.compare("", "𝄞💝♦💣東💯 Secret");
    assert_eq!(res, Err(Error::Invalid));

    // Same string. `OGHAM` SPACE MARK `U+1680` is mapped to SPACE `U+0020`
    let res = profile.compare("𝄞💝♦💣東💯 Secret", "𝄞💝♦💣東💯 Secret");
    assert_eq!(res, Ok(true));

    // Differs by case
    let res = profile.compare("Secret", "secret");
    assert_eq!(res, Ok(false));
}
