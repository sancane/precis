use precis_core::profile::PrecisFastInvocation;
use precis_core::{CodepointInfo, DerivedPropertyValue, Error};
use precis_profiles::OpaqueString;
use std::borrow::Cow;

#[test]
fn prepare() {
    // SPACE `U+0020` is allowed
    let res = OpaqueString::prepare("correct horse battery staple");
    assert_eq!(res, Ok(Cow::from("correct horse battery staple")));

    // Differs by case from previous example
    let res = OpaqueString::prepare("Correct Horse Battery Staple");
    assert_eq!(res, Ok(Cow::from("Correct Horse Battery Staple")));

    // Non-ASCII letters are OK (e.g., GREEK SMALL LETTER
    // PI `U+03C0`)
    let res = OpaqueString::prepare("πßå");
    assert_eq!(res, Ok(Cow::from("πßå")));

    // Symbols are OK (e.g., BLACK DIAMOND SUIT `U+2666`)
    let res = OpaqueString::prepare("Jack of ♦s");
    assert_eq!(res, Ok(Cow::from("Jack of ♦s")));

    // Zero-length passwords are disallowed
    let res = OpaqueString::prepare("");
    assert_eq!(res, Err(Error::Invalid));

    // Control characters like TAB `U+0009` are disallowed
    let res = OpaqueString::prepare("simple;\u{0009} test");
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
    // SPACE `U+0020` is allowed
    let res = OpaqueString::enforce("correct horse battery staple");
    assert_eq!(res, Ok(Cow::from("correct horse battery staple")));

    // Differs by case from previous example
    let res = OpaqueString::enforce("Correct Horse Battery Staple");
    assert_eq!(res, Ok(Cow::from("Correct Horse Battery Staple")));

    // Non-ASCII letters are OK (e.g., GREEK SMALL LETTER
    // PI `U+03C0`)
    let res = OpaqueString::enforce("πßå");
    assert_eq!(res, Ok(Cow::from("πßå")));

    // Symbols are OK (e.g., BLACK DIAMOND SUIT `U+2666`)
    let res = OpaqueString::enforce("Jack of ♦s");
    assert_eq!(res, Ok(Cow::from("Jack of ♦s")));

    // `OGHAM` SPACE MARK `U+1680` is mapped to SPACE `U+0020`;
    // thus, the full string is mapped to <foo bar>
    let res = OpaqueString::enforce("foo bar");
    assert_eq!(res, Ok(Cow::from("foo bar")));

    // Zero-length passwords are disallowed
    let res = OpaqueString::enforce("");
    assert_eq!(res, Err(Error::Invalid));

    // Control characters like TAB `U+0009` are disallowed
    let res = OpaqueString::enforce("simple;\u{0009} test");
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
    let res = OpaqueString::compare("𝄞💝♦💣東💯 Secret", "");
    assert_eq!(res, Err(Error::Invalid));

    let res = OpaqueString::compare("", "𝄞💝♦💣東💯 Secret");
    assert_eq!(res, Err(Error::Invalid));

    // Same string. `OGHAM` SPACE MARK `U+1680` is mapped to SPACE `U+0020`
    let res = OpaqueString::compare("𝄞💝♦💣東💯 Secret", "𝄞💝♦💣東💯 Secret");
    assert_eq!(res, Ok(true));

    // Differs by case
    let res = OpaqueString::compare("Secret", "secret");
    assert_eq!(res, Ok(false));
}

#[test]
fn test_single_character_passwords() {
    // Single ASCII character password
    let res = OpaqueString::prepare("a");
    assert_eq!(res, Ok(Cow::from("a")));

    let res = OpaqueString::enforce("a");
    assert_eq!(res, Ok(Cow::from("a")));

    // Single uppercase ASCII
    let res = OpaqueString::enforce("A");
    assert_eq!(res, Ok(Cow::from("A")));

    // Single digit
    let res = OpaqueString::prepare("5");
    assert_eq!(res, Ok(Cow::from("5")));

    // Single Unicode character
    let res = OpaqueString::prepare("π");
    assert_eq!(res, Ok(Cow::from("π")));

    // Single emoji
    let res = OpaqueString::prepare("😀");
    assert_eq!(res, Ok(Cow::from("😀")));

    // Single CJK character
    let res = OpaqueString::prepare("密");
    assert_eq!(res, Ok(Cow::from("密")));

    // Single symbol
    let res = OpaqueString::prepare("♦");
    assert_eq!(res, Ok(Cow::from("♦")));
}

#[test]
fn test_multibyte_passwords() {
    // UTF-8 multibyte sequences
    let res = OpaqueString::prepare("café");
    assert_eq!(res, Ok(Cow::from("café")));

    // Mix of multibyte characters
    let res = OpaqueString::enforce("πßå123");
    assert_eq!(res, Ok(Cow::from("πßå123")));

    // Full multibyte password
    let res = OpaqueString::enforce("密碼😀");
    assert_eq!(res, Ok(Cow::from("密碼😀")));

    // Arabic password
    let res = OpaqueString::prepare("كلمةالسر");
    assert_eq!(res, Ok(Cow::from("كلمةالسر")));
}

#[test]
fn test_contexto_chars_with_valid_context_are_idempotent() {
    // U+0387 (Greek Ano Teleia) normalizes to U+00B7 (Middle Dot)
    // which requires 'l' characters before and after (per RFC 5892 A.3).
    // When proper context is provided, idempotence should hold.

    // U+0387 with valid context: l + U+0387 + l
    let input_0387 = "l\u{0387}l";
    let enforced1 = OpaqueString::enforce(input_0387).unwrap();
    // Should normalize to l·l (U+00B7)
    assert_eq!(enforced1.as_ref(), "l\u{00B7}l");

    // Now enforce the result - should be idempotent because context is valid
    let enforced2 = OpaqueString::enforce(enforced1.as_ref()).unwrap();
    assert_eq!(enforced1.as_ref(), enforced2.as_ref());

    // Direct input with U+00B7 and valid context should also work
    let input_00b7 = "l\u{00B7}l";
    let enforced3 = OpaqueString::enforce(input_00b7).unwrap();
    assert_eq!(enforced3.as_ref(), "l\u{00B7}l");
}

#[test]
fn test_contexto_chars_without_context_behavior() {
    // U+00B7 (Middle Dot) alone requires 'l' context (per RFC 5892 A.3).
    // U+0387 (Greek Ano Teleia) is PVALID but normalizes to U+00B7.
    // Per RFC 8265, enforce output is conforming if prepare passed,
    // even if normalization produces context-dependent characters.

    // U+00B7 alone should fail in prepare (no context)
    let result_00b7 = OpaqueString::enforce("\u{00B7}");
    assert!(result_00b7.is_err());

    // U+0387 alone is PVALID and passes prepare
    // Normalizes to U+00B7, output is conforming per RFC 8265
    let result_0387 = OpaqueString::enforce("\u{0387}");
    assert_eq!(result_0387, Ok(Cow::from("·")));

    // However, the output is not idempotent (by design per RFC):
    // enforce(U+0387) → Ok(U+00B7), but enforce(U+00B7) → Err
    // This is expected behavior: not all enforce outputs are valid inputs
}
