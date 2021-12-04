use precis_core::profile::Profile;
use precis_core::{CodepointInfo, DerivedPropertyValue, Error};
use precis_profiles::Nickname;
use std::borrow::Cow;

#[test]
fn prepare() {
    let profile = Nickname::new();

    let res = profile.prepare("");
    assert_eq!(res, Err(Error::Invalid));

    let res = profile.prepare("Foo");
    assert_eq!(res, Ok(Cow::from("Foo")));

    let res = profile.prepare("foo");
    assert_eq!(res, Ok(Cow::from("foo")));

    let res = profile.prepare("Foo Bar");
    assert_eq!(res, Ok(Cow::from("Foo Bar")));

    let res = profile.prepare("  Foo     Bar     ");
    assert_eq!(res, Ok(Cow::from("  Foo     Bar     ")));

    let res = profile.prepare("Σ");
    assert_eq!(res, Ok(Cow::from("Σ")));

    let res = profile.prepare("σ");
    assert_eq!(res, Ok(Cow::from("σ")));

    let res = profile.prepare("ς");
    assert_eq!(res, Ok(Cow::from("ς")));

    let res = profile.prepare("ϔ");
    assert_eq!(res, Ok(Cow::from("ϔ")));

    let res = profile.prepare("∞");
    assert_eq!(res, Ok(Cow::from("∞")));

    let res = profile.prepare("Richard \u{2163}");
    assert_eq!(res, Ok(Cow::from("Richard \u{2163}")));

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
    let profile = Nickname::new();

    let res = profile.enforce("");
    assert_eq!(res, Err(Error::Invalid));

    let res = profile.enforce("Foo");
    assert_eq!(res, Ok(Cow::from("Foo")));

    let res = profile.enforce("foo");
    assert_eq!(res, Ok(Cow::from("foo")));

    let res = profile.enforce("Foo Bar");
    assert_eq!(res, Ok(Cow::from("Foo Bar")));

    let res = profile.enforce("  Foo     Bar     ");
    assert_eq!(res, Ok(Cow::from("Foo Bar")));

    let res = profile.enforce("Σ");
    assert_eq!(res, Ok(Cow::from("Σ")));

    let res = profile.enforce("σ");
    assert_eq!(res, Ok(Cow::from("σ")));

    let res = profile.enforce("ς");
    assert_eq!(res, Ok(Cow::from("ς")));

    let res = profile.enforce("ϔ");
    assert_eq!(res, Ok(Cow::from("Ϋ")));

    let res = profile.enforce("∞");
    assert_eq!(res, Ok(Cow::from("∞")));

    let res = profile.enforce("Richard \u{2163}");
    assert_eq!(res, Ok(Cow::from("Richard IV")));

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
    let profile = Nickname::new();

    let res = profile.compare("", "");
    assert_eq!(res, Err(Error::Invalid));

    let res = profile.compare("Foo", "");
    assert_eq!(res, Err(Error::Invalid));

    let res = profile.compare("", "foo");
    assert_eq!(res, Err(Error::Invalid));

    let res = profile.compare("Foo", "foo");
    assert_eq!(res, Ok(true));

    let res = profile.compare("foo", "foo");
    assert_eq!(res, Ok(true));

    let res = profile.compare("Foo Bar", "foo bar");
    assert_eq!(res, Ok(true));

    let res = profile.compare("  Foo     Bar     ", "foo bar");
    assert_eq!(res, Ok(true));

    let res = profile.compare("Σ", "σ");
    assert_eq!(res, Ok(true));

    let res = profile.compare("σ", "σ");
    assert_eq!(res, Ok(true));

    let res = profile.compare("ς", "ς");
    assert_eq!(res, Ok(true));

    let res = profile.compare("ϔ", "ϋ");
    assert_eq!(res, Ok(true));

    let res = profile.compare("∞", "∞");
    assert_eq!(res, Ok(true));

    let res = profile.compare("Richard \u{2163}", "richard iv");
    assert_eq!(res, Ok(true));

    // Control characters like TAB `U+0009` are disallowed
    let res = profile.compare("simple;\u{0009} test", "simple;\u{0009} test");
    assert_eq!(
        res,
        Err(Error::BadCodepoint(CodepointInfo::new(
            0x0009,
            7,
            DerivedPropertyValue::Disallowed
        )))
    );
}
