use precis_core::profile::PrecisFastInvocation;
use precis_core::{CodepointInfo, DerivedPropertyValue, Error};
use precis_profiles::Nickname;
use std::borrow::Cow;

#[test]
fn prepare() {
    let res = Nickname::prepare("");
    assert_eq!(res, Err(Error::Invalid));

    let res = Nickname::prepare("Foo");
    assert_eq!(res, Ok(Cow::from("Foo")));

    let res = Nickname::prepare("foo");
    assert_eq!(res, Ok(Cow::from("foo")));

    let res = Nickname::prepare("Foo Bar");
    assert_eq!(res, Ok(Cow::from("Foo Bar")));

    let res = Nickname::prepare("  Foo     Bar     ");
    assert_eq!(res, Ok(Cow::from("  Foo     Bar     ")));

    let res = Nickname::prepare("Σ");
    assert_eq!(res, Ok(Cow::from("Σ")));

    let res = Nickname::prepare("σ");
    assert_eq!(res, Ok(Cow::from("σ")));

    let res = Nickname::prepare("ς");
    assert_eq!(res, Ok(Cow::from("ς")));

    let res = Nickname::prepare("ϔ");
    assert_eq!(res, Ok(Cow::from("ϔ")));

    let res = Nickname::prepare("∞");
    assert_eq!(res, Ok(Cow::from("∞")));

    let res = Nickname::prepare("Richard \u{2163}");
    assert_eq!(res, Ok(Cow::from("Richard \u{2163}")));

    // Control characters like TAB `U+0009` are disallowed
    let res = Nickname::prepare("simple;\u{0009} test");
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
    let res = Nickname::enforce("");
    assert_eq!(res, Err(Error::Invalid));

    let res = Nickname::enforce("Foo");
    assert_eq!(res, Ok(Cow::from("Foo")));

    let res = Nickname::enforce("foo");
    assert_eq!(res, Ok(Cow::from("foo")));

    let res = Nickname::enforce("Foo Bar");
    assert_eq!(res, Ok(Cow::from("Foo Bar")));

    let res = Nickname::enforce("  Foo     Bar     ");
    assert_eq!(res, Ok(Cow::from("Foo Bar")));

    let res = Nickname::enforce("Σ");
    assert_eq!(res, Ok(Cow::from("Σ")));

    let res = Nickname::enforce("σ");
    assert_eq!(res, Ok(Cow::from("σ")));

    let res = Nickname::enforce("ς");
    assert_eq!(res, Ok(Cow::from("ς")));

    let res = Nickname::enforce("ϔ");
    assert_eq!(res, Ok(Cow::from("Ϋ")));

    let res = Nickname::enforce("∞");
    assert_eq!(res, Ok(Cow::from("∞")));

    let res = Nickname::enforce("Richard \u{2163}");
    assert_eq!(res, Ok(Cow::from("Richard IV")));

    // Control characters like TAB `U+0009` are disallowed
    let res = Nickname::enforce("simple;\u{0009} test");
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
    let res = Nickname::compare("", "");
    assert_eq!(res, Err(Error::Invalid));

    let res = Nickname::compare("Foo", "");
    assert_eq!(res, Err(Error::Invalid));

    let res = Nickname::compare("", "foo");
    assert_eq!(res, Err(Error::Invalid));

    let res = Nickname::compare("Foo", "foo");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("foo", "foo");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("Foo Bar", "foo bar");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("  Foo     Bar     ", "foo bar");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("Σ", "σ");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("σ", "σ");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("ς", "ς");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("ϔ", "ϋ");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("∞", "∞");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("Richard \u{2163}", "richard iv");
    assert_eq!(res, Ok(true));

    // Control characters like TAB `U+0009` are disallowed
    let res = Nickname::compare("simple;\u{0009} test", "simple;\u{0009} test");
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
fn test_multibyte_utf8_characters() {
    // Test with Spanish characters (accented)
    let res = Nickname::enforce("José García");
    assert_eq!(res, Ok(Cow::from("José García")));

    let res = Nickname::enforce("  José   García  ");
    assert_eq!(res, Ok(Cow::from("José García")));

    // Test with French characters
    let res = Nickname::enforce("François Müller");
    assert_eq!(res, Ok(Cow::from("François Müller")));

    let res = Nickname::enforce("  Naïve   Test  ");
    assert_eq!(res, Ok(Cow::from("Naïve Test")));

    // Test with German characters
    let res = Nickname::enforce("  Björk   Güdröndóttir  ");
    assert_eq!(res, Ok(Cow::from("Björk Güdröndóttir")));

    // Test with café example (the original bug case)
    let res = Nickname::enforce("café test");
    assert_eq!(res, Ok(Cow::from("café test")));

    let res = Nickname::enforce("  café   test  ");
    assert_eq!(res, Ok(Cow::from("café test")));

    // Test comparison with multibyte
    let res = Nickname::compare("  José   García  ", "josé garcía");
    assert_eq!(res, Ok(true));

    let res = Nickname::compare("  café   test  ", "café test");
    assert_eq!(res, Ok(true));
}

#[test]
fn test_cjk_characters() {
    // Test with Chinese characters
    let res = Nickname::enforce("李明");
    assert_eq!(res, Ok(Cow::from("李明")));

    let res = Nickname::enforce("  李明  ");
    assert_eq!(res, Ok(Cow::from("李明")));

    let res = Nickname::enforce("  张三   李四  ");
    assert_eq!(res, Ok(Cow::from("张三 李四")));

    // Test with Japanese characters
    let res = Nickname::enforce("田中太郎");
    assert_eq!(res, Ok(Cow::from("田中太郎")));

    let res = Nickname::enforce("  佐藤   花子  ");
    assert_eq!(res, Ok(Cow::from("佐藤 花子")));

    // Test with Korean characters
    let res = Nickname::enforce("김철수");
    assert_eq!(res, Ok(Cow::from("김철수")));

    let res = Nickname::enforce("  박영희   이민수  ");
    assert_eq!(res, Ok(Cow::from("박영희 이민수")));
}

#[test]
fn test_arabic_characters() {
    // Test with Arabic characters
    let res = Nickname::enforce("محمد");
    assert_eq!(res, Ok(Cow::from("محمد")));

    let res = Nickname::enforce("  محمد   علي  ");
    assert_eq!(res, Ok(Cow::from("محمد علي")));

    // Test with mixed Arabic and Latin
    let res = Nickname::enforce("  User   محمد  ");
    assert_eq!(res, Ok(Cow::from("User محمد")));
}

#[test]
fn test_emoji_with_spaces() {
    // Test with emoji characters
    let res = Nickname::enforce("User 🎮");
    assert_eq!(res, Ok(Cow::from("User 🎮")));

    let res = Nickname::enforce("  Player   🎯  ");
    assert_eq!(res, Ok(Cow::from("Player 🎯")));

    let res = Nickname::enforce("  Test   ⚡   User  ");
    assert_eq!(res, Ok(Cow::from("Test ⚡ User")));
}

#[test]
fn test_utf8_bug_double_space_after_multibyte() {
    // This test specifically targets the UTF-8 bug:
    // "café  test" has a 2-byte character 'é' followed by double space
    // If chars().enumerate() is used instead of char_indices(),
    // it will return character index 4 for the first space,
    // but the byte index at character 4 is actually 5 (because 'é' is 2 bytes).
    // Using character index as byte index in slicing will cause panic.
    let res = Nickname::enforce("café  test");
    assert_eq!(res, Ok(Cow::from("café test")));

    // Another case: leading double space after multibyte at start
    let res = Nickname::enforce("é  test");
    assert_eq!(res, Ok(Cow::from("é test")));

    // Case with emoji (4-byte UTF-8) followed by double space
    let res = Nickname::enforce("hi🎮  test");
    assert_eq!(res, Ok(Cow::from("hi🎮 test")));
}

#[test]
fn test_single_character_inputs() {
    // Single ASCII character
    let res = Nickname::prepare("a");
    assert_eq!(res, Ok(Cow::from("a")));

    let res = Nickname::enforce("a");
    assert_eq!(res, Ok(Cow::from("a")));

    // Single uppercase ASCII
    let res = Nickname::enforce("A");
    assert_eq!(res, Ok(Cow::from("A")));

    // Single digit
    let res = Nickname::prepare("5");
    assert_eq!(res, Ok(Cow::from("5")));

    // Single Unicode character
    let res = Nickname::prepare("é");
    assert_eq!(res, Ok(Cow::from("é")));

    // Single emoji
    let res = Nickname::prepare("😀");
    assert_eq!(res, Ok(Cow::from("😀")));

    // Single CJK character
    let res = Nickname::prepare("文");
    assert_eq!(res, Ok(Cow::from("文")));

    // Single space should fail after trimming (becomes empty)
    let res = Nickname::enforce(" ");
    assert_eq!(res, Err(Error::Invalid));
}

#[test]
fn test_single_character_with_context_rules() {
    // Single ZWNJ - context rule should fail (needs proper context)
    let res = Nickname::prepare("\u{200C}");
    // ZWNJ alone typically fails context rules
    assert!(res.is_err());

    // Single ZWJ - context rule should fail
    let res = Nickname::prepare("\u{200D}");
    assert!(res.is_err());

    // Middle dot alone should fail (needs l·l context per RFC 5892 A.3)
    let res = Nickname::prepare("\u{00B7}");
    assert!(
        res.is_err(),
        "Middle dot alone should fail - requires l·l context"
    );
}
