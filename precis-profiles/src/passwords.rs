use crate::common;
use lazy_static::lazy_static;
use precis_core::profile::{PrecisFastInvocation, Profile, Rules};
use precis_core::Error;
use precis_core::{FreeformClass, StringClass};
use std::borrow::Cow;

/// [`OpaqueString`](<https://datatracker.ietf.org/doc/html/rfc8265#section-4.2>)
/// Profile designed to deal with passwords and other opaque strings in security
/// and application protocols.
/// Replaces:  The `SASLprep` profile of `Stringprep`. Look at the
/// [`IANA` Considerations](https://datatracker.ietf.org/doc/html/rfc8265#section-7.3)
/// section for more details.
/// # Example
/// ```rust
/// use precis_core::Error;
/// use precis_core::profile::Profile;
/// use precis_profiles::OpaqueString;
/// use std::borrow::Cow;
///
/// // create OpaqueString profile
/// let profile = OpaqueString::new();
///
/// // prepare string
/// assert_eq!(profile.prepare("I'm Guybrush Threepwood, Mighty Pirate ☠"),
///     Ok(Cow::from("I'm Guybrush Threepwood, Mighty Pirate ☠")));
///
/// // enforce string
/// assert_eq!(profile.enforce("Look behind you, a three-headed monkey!🐒"),
///     Ok(Cow::from("Look behind you, a three-headed monkey!🐒")));
///
/// // compare strings
/// assert_eq!(profile.compare("That’s the second biggest 🐵 I’ve ever seen!",
///     "That’s the second biggest 🐵 I’ve ever seen!"), Ok(true));
/// ```
pub struct OpaqueString {
    class: FreeformClass,
}

impl OpaqueString {
    /// Creates a [`OpaqueString`] profile.
    pub fn new() -> Self {
        Self {
            class: FreeformClass {},
        }
    }
}

impl Default for OpaqueString {
    fn default() -> Self {
        OpaqueString::new()
    }
}

impl Profile for OpaqueString {
    fn prepare<'a>(&self, s: &'a str) -> Result<Cow<'a, str>, Error> {
        let s = (!s.is_empty()).then(|| s).ok_or(Error::Disallowed)?;
        self.class
            .allows(s)
            .then(|| s.into())
            .ok_or(Error::Disallowed)
    }

    fn enforce<'a>(&self, s: &'a str) -> Result<Cow<'a, str>, Error> {
        let s = self.prepare(s)?;
        let s = self.additional_mapping_rule(s)?;
        let s = self.normalization_rule(s)?;
        (!s.is_empty()).then(|| s).ok_or(Error::Disallowed)
    }

    fn compare(&self, s1: &str, s2: &str) -> Result<bool, Error> {
        Ok(self.enforce(s1)? == self.enforce(s2)?)
    }
}

impl Rules for OpaqueString {
    fn additional_mapping_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        let s = s.into();
        match s.find(common::is_non_ascii_space) {
            None => Ok(s),
            Some(pos) => {
                let mut res = String::from(&s[..pos]);
                res.reserve(s.len() - res.len());
                for c in s[pos..].chars() {
                    if common::is_non_ascii_space(c) {
                        res.push(common::SPACE);
                    } else {
                        res.push(c);
                    }
                }
                Ok(res.into())
            }
        }
    }

    fn normalization_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        common::normalization_form_nfc(s)
    }
}

fn get_opaque_string_profile() -> &'static OpaqueString {
    lazy_static! {
        static ref OPAQUE_STRING: OpaqueString = OpaqueString::new();
    }
    &OPAQUE_STRING
}

impl PrecisFastInvocation for OpaqueString {
    fn prepare(s: &str) -> Result<Cow<'_, str>, Error> {
        get_opaque_string_profile().prepare(s)
    }

    fn enforce<'a>(s: &str) -> Result<Cow<'_, str>, Error> {
        get_opaque_string_profile().enforce(s)
    }

    fn compare(s1: &str, s2: &str) -> Result<bool, Error> {
        get_opaque_string_profile().compare(s1, s2)
    }
}

#[cfg(test)]
mod opaque_string {
    use crate::passwords::*;

    #[test]
    fn prepare() {
        let profile = OpaqueString::new();

        // SPACE `U+0020` is allowed
        let res = profile.prepare("correct horse battery staple");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "correct horse battery staple");

        // Differs by case from previous example
        let res = profile.prepare("Correct Horse Battery Staple");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Correct Horse Battery Staple");

        // Non-ASCII letters are OK (e.g., GREEK SMALL LETTER
        // PI `U+03C0`)
        let res = profile.prepare("πßå");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "πßå");

        // Symbols are OK (e.g., BLACK DIAMOND SUIT `U+2666`)
        let res = profile.prepare("Jack of ♦s");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Jack of ♦s");

        // Zero-length passwords are disallowed
        let res = profile.prepare("");
        assert_eq!(res.is_err(), true);

        // Control characters like TAB `U+0009` are disallowed
        let res = profile.prepare("simple;\u{0009} test");
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn enforce() {
        let profile = OpaqueString::new();

        // SPACE `U+0020` is allowed
        let res = profile.enforce("correct horse battery staple");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "correct horse battery staple");

        // Differs by case from previous example
        let res = profile.enforce("Correct Horse Battery Staple");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Correct Horse Battery Staple");

        // Non-ASCII letters are OK (e.g., GREEK SMALL LETTER
        // PI `U+03C0`)
        let res = profile.enforce("πßå");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "πßå");

        // Symbols are OK (e.g., BLACK DIAMOND SUIT `U+2666`)
        let res = profile.enforce("Jack of ♦s");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Jack of ♦s");

        // `OGHAM` SPACE MARK `U+1680` is mapped to SPACE `U+0020`;
        // thus, the full string is mapped to <foo bar>
        let res = profile.enforce("foo bar");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "foo bar");

        // Symbols are OK (e.g., BLACK DIAMOND SUIT `U+2666`)
        let res = profile.enforce("Jack of ♦s");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "Jack of ♦s");

        // Zero-length passwords are disallowed
        let res = profile.enforce("");
        assert_eq!(res.is_err(), true);

        // Control characters like TAB `U+0009` are disallowed
        let res = profile.enforce("simple;\u{0009} test");
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn compare() {
        let profile = OpaqueString::new();

        let res = profile.compare("𝄞💝♦💣東💯 Secret", "");
        assert_eq!(res.is_err(), true);

        let res = profile.compare("", "𝄞💝♦💣東💯 Secret");
        assert_eq!(res.is_err(), true);

        // Same string. `OGHAM` SPACE MARK `U+1680` is mapped to SPACE `U+0020`
        let res = profile.compare("𝄞💝♦💣東💯 Secret", "𝄞💝♦💣東💯 Secret");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), true);

        // Differs by case
        let res = profile.compare("Secret", "secret");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), false);
    }
}
