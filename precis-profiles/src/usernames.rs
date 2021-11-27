include!(concat!(env!("OUT_DIR"), "/width_mapping.rs"));

use crate::bidi;
use crate::common;
use lazy_static::lazy_static;
use precis_core::profile::{PrecisFastInvocation, Profile, Rules};
use precis_core::Codepoints;
use precis_core::Error;
use precis_core::{IdentifierClass, StringClass};
use std::borrow::Cow;

fn get_decomposition_mapping(cp: u32) -> Option<u32> {
    WIDE_NARROW_MAPPING
        .binary_search_by(|cps| cps.0.partial_cmp(&cp).unwrap())
        .map(|x| WIDE_NARROW_MAPPING[x].1)
        .ok()
}

fn has_width_mapping(c: char) -> bool {
    get_decomposition_mapping(c as u32).is_some()
}

fn width_mapping_rule<'a, T>(s: T) -> Result<Cow<'a, str>, Error>
where
    T: Into<Cow<'a, str>>,
{
    let s = s.into();
    match s.find(has_width_mapping) {
        None => Ok(s),
        Some(pos) => {
            let mut res = String::from(&s[..pos]);
            res.reserve(s.len() - res.len());
            for c in s[pos..].chars() {
                res.push(match get_decomposition_mapping(c as u32) {
                    Some(d) => char::from_u32(d).ok_or(Error::Unexpected)?,
                    None => c,
                });
            }
            Ok(res.into())
        }
    }
}

fn directionality_rule<'a, T>(s: T) -> Result<Cow<'a, str>, Error>
where
    T: Into<Cow<'a, str>>,
{
    let s = s.into();
    if bidi::has_rtl(&s) {
        bidi::satisfy_bidi_rule(&s)
            .then(|| s)
            .ok_or(Error::Disallowed)
    } else {
        Ok(s)
    }
}

/// [`UsernameCaseMapped`](https://datatracker.ietf.org/doc/html/rfc8265#section-3.3).
/// Profile designed to deal with usernames in security and application protocols.
/// It replaces the `SASLprep` profile of `Stringprep`. Look at the
/// [`IANA` Considerations](https://datatracker.ietf.org/doc/html/rfc8265#section-7.1)
/// section for more details.
/// # Example
/// ```rust
/// use precis_core::Error;
/// use precis_core::profile::Profile;
/// use precis_profiles::UsernameCaseMapped;
/// use std::borrow::Cow;
///
/// // create UsernameCaseMapped profile
/// let profile = UsernameCaseMapped::new();
///
/// // prepare string
/// assert_eq!(profile.prepare("Guybrush"), Ok(Cow::from("Guybrush")));
///
/// // UsernameCaseMapped does not accept spaces
/// assert_eq!(profile.prepare("Guybrush Threepwood"),
///    Err(Error::Disallowed));
///
/// // enforce string
/// assert_eq!(profile.enforce("Guybrush"), Ok(Cow::from("guybrush")));
///
/// // compare strings
/// assert_eq!(profile.compare("Guybrush", "guybrush"), Ok(true));
/// ```
pub struct UsernameCaseMapped {
    class: IdentifierClass,
}

impl UsernameCaseMapped {
    /// Creates a [`UsernameCaseMapped`] profile.
    pub fn new() -> Self {
        Self {
            class: IdentifierClass {},
        }
    }
}

impl Default for UsernameCaseMapped {
    fn default() -> Self {
        UsernameCaseMapped::new()
    }
}

impl Profile for UsernameCaseMapped {
    fn prepare<'a>(&self, s: &'a str) -> Result<Cow<'a, str>, Error> {
        let s = self.width_mapping_rule(s)?;
        let s = (!s.is_empty()).then(|| s).ok_or(Error::Disallowed)?;
        self.class.allows(&s).then(|| s).ok_or(Error::Disallowed)
    }

    fn enforce<'a>(&self, s: &'a str) -> Result<Cow<'a, str>, Error> {
        let s = self.prepare(s)?;
        let s = self.case_mapping_rule(s)?;
        let s = self.normalization_rule(s)?;
        let s = (!s.is_empty()).then(|| s).ok_or(Error::Disallowed)?;
        directionality_rule(s)
    }

    fn compare(&self, s1: &str, s2: &str) -> Result<bool, Error> {
        Ok(self.enforce(s1)? == self.enforce(s2)?)
    }
}

impl Rules for UsernameCaseMapped {
    fn width_mapping_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        width_mapping_rule(s)
    }

    fn case_mapping_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        common::case_mapping_rule(s)
    }

    fn normalization_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        common::normalization_form_nfc(s)
    }

    fn directionality_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        directionality_rule(s)
    }
}

fn get_username_case_mapped_profile() -> &'static UsernameCaseMapped {
    lazy_static! {
        static ref USERNAME_CASE_MAPPED: UsernameCaseMapped = UsernameCaseMapped::new();
    }
    &USERNAME_CASE_MAPPED
}

impl PrecisFastInvocation for UsernameCaseMapped {
    fn prepare(s: &str) -> Result<Cow<'_, str>, Error> {
        get_username_case_mapped_profile().prepare(s)
    }

    fn enforce(s: &str) -> Result<Cow<'_, str>, Error> {
        get_username_case_mapped_profile().enforce(s)
    }

    fn compare(s1: &str, s2: &str) -> Result<bool, Error> {
        get_username_case_mapped_profile().compare(s1, s2)
    }
}

/// [`UsernameCasePreserved`](https://datatracker.ietf.org/doc/html/rfc8265#section-3.4).
/// Profile designed to deal with usernames in security and application protocols.
/// It replaces the `SASLprep` profile of `Stringprep`. Look at the
/// [`IANA` Considerations](https://datatracker.ietf.org/doc/html/rfc8265#section-7.2)
/// section for more details.
/// # Example
/// ```rust
/// use precis_core::Error;
/// use precis_core::profile::Profile;
/// use precis_profiles::UsernameCasePreserved;
/// use std::borrow::Cow;
///
/// // create UsernameCasePreserved profile
/// let profile = UsernameCasePreserved::new();
///
/// // prepare string
/// assert_eq!(profile.prepare("Guybrush"), Ok(Cow::from("Guybrush")));
///
/// // UsernameCaseMapped does not accept spaces
/// assert_eq!(profile.prepare("Guybrush Threepwood"),
///    Err(Error::Disallowed));
///
/// // enforce string
/// assert_eq!(profile.enforce("Guybrush"), Ok(Cow::from("Guybrush")));
///
/// // compare strings
/// assert_eq!(profile.compare("Guybrush", "Guybrush"), Ok(true));
/// ```
pub struct UsernameCasePreserved {
    class: IdentifierClass,
}

impl UsernameCasePreserved {
    /// Creates a [`UsernameCasePreserved`] profile.
    pub fn new() -> Self {
        Self {
            class: IdentifierClass {},
        }
    }
}

impl Default for UsernameCasePreserved {
    fn default() -> Self {
        UsernameCasePreserved::new()
    }
}

impl Profile for UsernameCasePreserved {
    fn prepare<'a>(&self, s: &'a str) -> Result<Cow<'a, str>, Error> {
        let s = self.width_mapping_rule(s)?;
        let s = (!s.is_empty()).then(|| s).ok_or(Error::Disallowed)?;
        self.class.allows(&s).then(|| s).ok_or(Error::Disallowed)
    }

    fn enforce<'a>(&self, s: &'a str) -> Result<Cow<'a, str>, Error> {
        let s = self.prepare(s)?;
        let s = self.normalization_rule(s)?;
        let s = (!s.is_empty()).then(|| s).ok_or(Error::Disallowed)?;
        self.directionality_rule(s)
    }

    fn compare(&self, s1: &str, s2: &str) -> Result<bool, Error> {
        Ok(self.enforce(s1)? == self.enforce(s2)?)
    }
}

impl Rules for UsernameCasePreserved {
    fn width_mapping_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        width_mapping_rule(s)
    }

    fn normalization_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        common::normalization_form_nfc(s)
    }

    fn directionality_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        directionality_rule(s)
    }
}

fn get_username_case_preserved_profile() -> &'static UsernameCasePreserved {
    lazy_static! {
        static ref USERNAME_CASE_PRESERVED: UsernameCasePreserved = UsernameCasePreserved::new();
    }
    &USERNAME_CASE_PRESERVED
}

impl PrecisFastInvocation for UsernameCasePreserved {
    fn prepare(s: &str) -> Result<Cow<'_, str>, Error> {
        get_username_case_preserved_profile().prepare(s)
    }

    fn enforce(s: &str) -> Result<Cow<'_, str>, Error> {
        get_username_case_preserved_profile().enforce(s)
    }

    fn compare(s1: &str, s2: &str) -> Result<bool, Error> {
        get_username_case_preserved_profile().compare(s1, s2)
    }
}

#[cfg(test)]
mod profile_rules {
    use crate::usernames::*;

    #[test]
    fn test_width_mapping_rule() {
        let res = width_mapping_rule("");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "");

        // Valid username with no modifications
        let res = width_mapping_rule("TestName");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "TestName");

        // Mapping code point `U+FF03` (`＃`) to `U+0023` (`#`)
        let res = width_mapping_rule("\u{ff03}");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "\u{0023}");

        let res = width_mapping_rule("a\u{ff03}");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "a\u{0023}");

        let res = width_mapping_rule("\u{ff03}a");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "\u{0023}a");

        let res = width_mapping_rule("\u{ff03}\u{ff03}\u{ff03}");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "\u{0023}\u{0023}\u{0023}");
    }

    #[test]
    fn test_directionality_rule() {
        let res = directionality_rule("");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "");

        // No `RTL` label
        let res = directionality_rule("Hello");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Hello");

        // `RTL` label
        let res = directionality_rule("\u{05be}");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "\u{05be}");

        // `LTR` label
        let res = directionality_rule("\u{00aa}");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "\u{00aa}");

        // Invalid label
        let res = directionality_rule("\u{05be}Hello");
        assert_eq!(res.is_ok(), false);
    }
}

#[cfg(test)]
mod username_case_mapped {
    use crate::usernames::*;

    #[test]
    fn prepare() {
        let profile = UsernameCaseMapped::new();

        // Preparation does no apply case mapping rule
        let res = profile.prepare("XxXxX");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "XxXxX");

        // Preparation applies width mapping rule
        let res = profile.prepare("Xx\u{ff01}xX");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Xx\u{0021}xX");

        // The "at" sign ("@") is allowed in the PRECIS `IdentifierClass`
        let res = profile.prepare("juliet@example.com");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "juliet@example.com");

        let res = profile.prepare("fussaball");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "fussaball");

        // The third character is LATIN SMALL LETTER SHARP S `U+00DF`
        let res = profile.prepare("fußball");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "fußball");

        // A `userpart` of GREEK SMALL LETTER PI `U+03C0`
        let res = profile.prepare("π");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "π");

        // A `userpart` of GREEK CAPITAL LETTER SIGMA `U+03A3`
        let res = profile.prepare("Σ");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Σ");

        // A `userpart` of GREEK SMALL LETTER SIGMA `U+03C3`
        let res = profile.prepare("σ");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "σ");

        // A `userpart` of GREEK SMALL LETTER FINAL SIGMA `U+03C2`
        let res = profile.prepare("ς");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "ς");

        // SPACE `U+0020` is disallowed in the `userpart`
        let res = profile.prepare("foo bar");
        assert_eq!(res.is_err(), true);

        // Zero-length `userpart`
        let res = profile.prepare("");
        assert_eq!(res.is_err(), true);

        // The sixth character is ROMAN NUMERAL FOUR `U+2163`
        let res = profile.prepare("henry\u{2163}");
        assert_eq!(res.is_err(), true);

        // A `userpart` of INFINITY `U+221E`
        let res = profile.prepare("∞");
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn enforce() {
        let profile = UsernameCaseMapped::new();

        // Enforcement applies case mapping rule
        let res = profile.enforce("XxXxX");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "xxxxx");

        // Enforcement applies width mapping rule
        let res = profile.enforce("Xx\u{ff01}xX");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "xx!xx");

        // The "at" sign ("@") is allowed in the PRECIS `IdentifierClass`
        let res = profile.enforce("juliet@example.com");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "juliet@example.com");

        let res = profile.enforce("fussaball");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "fussaball");

        // The third character is LATIN SMALL LETTER SHARP S `U+00DF`
        let res = profile.enforce("fußball");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "fußball");

        // A `userpart` of GREEK SMALL LETTER PI `U+03C0`
        let res = profile.enforce("π");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "π");

        // A `userpart` of GREEK CAPITAL LETTER SIGMA `U+03A3`
        let res = profile.enforce("Σ");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "σ");

        // A `userpart` of GREEK SMALL LETTER SIGMA `U+03C3`
        let res = profile.prepare("σ");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "σ");

        // A `userpart` of GREEK SMALL LETTER FINAL SIGMA `U+03C2`
        let res = profile.prepare("ς");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "ς");

        // SPACE `U+0020` is disallowed in the `userpart`
        let res = profile.prepare("foo bar");
        assert_eq!(res.is_err(), true);

        // Zero-length `userpart`
        let res = profile.prepare("");
        assert_eq!(res.is_err(), true);

        // The sixth character is ROMAN NUMERAL FOUR `U+2163`
        let res = profile.prepare("henry\u{2163}");
        assert_eq!(res.is_err(), true);

        // A `userpart` of INFINITY `U+221E`
        let res = profile.prepare("∞");
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn compare() {
        let profile = UsernameCaseMapped::new();

        let res = profile.compare("", "Hello");
        assert_eq!(res.is_err(), true);

        let res = profile.compare("Hello", "");
        assert_eq!(res.is_err(), true);

        let res = profile.compare("heLLo", "Hello");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), true);

        let res = profile.compare("heLLo!", "Hello\u{ff01}");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), true);

        let res = profile.compare("heLLo", "Bello");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), false);

        let res = profile.compare("heLL∞", "Hell");
        assert_eq!(res.is_err(), true);
    }
}

#[cfg(test)]
mod username_case_preserved {
    use crate::usernames::*;

    #[test]
    fn prepare() {
        let profile = UsernameCasePreserved::new();

        // Preparation does no apply case mapping rule
        let res = profile.prepare("XxXxX");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "XxXxX");

        // Preparation applies width mapping rule
        let res = profile.prepare("Xx\u{ff01}xX");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Xx\u{0021}xX");

        // The "at" sign (`@`) is allowed in the PRECIS `IdentifierClass`
        let res = profile.prepare("juliet@example.com");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "juliet@example.com");

        let res = profile.prepare("fussaball");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "fussaball");

        // The third character is LATIN SMALL LETTER SHARP S `U+00DF`
        let res = profile.prepare("fußball");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "fußball");

        // A `userpart` of GREEK SMALL LETTER PI `U+03C0`
        let res = profile.prepare("π");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "π");

        // A `userpart` of GREEK CAPITAL LETTER SIGMA `U+03A3`
        let res = profile.prepare("Σ");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Σ");

        // A `userpart` of GREEK SMALL LETTER SIGMA `U+03C3`
        let res = profile.prepare("σ");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "σ");

        // A `userpart` of GREEK SMALL LETTER FINAL SIGMA `U+03C2`
        let res = profile.prepare("ς");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "ς");

        // SPACE `U+0020` is disallowed in the `userpart`
        let res = profile.prepare("foo bar");
        assert_eq!(res.is_err(), true);

        // Zero-length `userpart`
        let res = profile.prepare("");
        assert_eq!(res.is_err(), true);

        // The sixth character is ROMAN NUMERAL FOUR `U+2163`
        let res = profile.prepare("henry\u{2163}");
        assert_eq!(res.is_err(), true);

        // A `userpart` of INFINITY `U+221E`
        let res = profile.prepare("∞");
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn enforce() {
        let profile = UsernameCasePreserved::new();

        // Enforcement does not apply case mapping rule
        let res = profile.enforce("XxXxX");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "XxXxX");

        // Enforcement applies width mapping rule
        let res = profile.enforce("Xx\u{ff01}xX");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Xx!xX");

        // The "at" sign (`@`) is allowed in the PRECIS `IdentifierClass`
        let res = profile.enforce("juliet@example.com");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "juliet@example.com");

        let res = profile.enforce("fussaball");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "fussaball");

        // The third character is LATIN SMALL LETTER SHARP S `U+00DF`
        let res = profile.enforce("fußball");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "fußball");

        // A `userpart` of GREEK SMALL LETTER PI `U+03C0`
        let res = profile.enforce("π");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "π");

        // A `userpart` of GREEK CAPITAL LETTER SIGMA `U+03A3`
        let res = profile.enforce("Σ");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Σ");

        // A `userpart` of GREEK SMALL LETTER SIGMA `U+03C3`
        let res = profile.prepare("σ");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "σ");

        // A `userpart` of GREEK SMALL LETTER FINAL SIGMA `U+03C2`
        let res = profile.prepare("ς");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "ς");

        // SPACE `U+0020` is disallowed in the `userpart`
        let res = profile.prepare("foo bar");
        assert_eq!(res.is_err(), true);

        // Zero-length `userpart`
        let res = profile.prepare("");
        assert_eq!(res.is_err(), true);

        // The sixth character is ROMAN NUMERAL FOUR `U+2163`
        let res = profile.prepare("henry\u{2163}");
        assert_eq!(res.is_err(), true);

        // A `userpart` of INFINITY `U+221E`
        let res = profile.prepare("∞");
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn compare() {
        let profile = UsernameCasePreserved::new();

        let res = profile.compare("", "Hello");
        assert_eq!(res.is_err(), true);

        let res = profile.compare("Hello", "");
        assert_eq!(res.is_err(), true);

        let res = profile.compare("heLLo", "Hello");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), false);

        let res = profile.compare("Hello!", "Hello\u{ff01}");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), true);

        let res = profile.compare("heLLo", "Bello");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), false);

        let res = profile.compare("heLL∞", "Hell");
        assert_eq!(res.is_err(), true);
    }
}
