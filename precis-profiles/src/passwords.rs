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
/// # use precis_core::Error;
/// # use precis_core::profile::Profile;
/// # use precis_profiles::OpaqueString;
/// # use std::borrow::Cow;
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
        let s = (!s.is_empty()).then(|| s).ok_or(Error::Invalid)?;
        self.class.allows(s)?;
        Ok(s.into())
    }

    fn enforce<'a>(&self, s: &'a str) -> Result<Cow<'a, str>, Error> {
        let s = self.prepare(s)?;
        let s = self.additional_mapping_rule(s)?;
        let s = self.normalization_rule(s)?;
        (!s.is_empty()).then(|| s).ok_or(Error::Invalid)
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
