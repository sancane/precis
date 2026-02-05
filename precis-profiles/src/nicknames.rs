use crate::common;
use precis_core::profile::stabilize;
use precis_core::profile::{PrecisFastInvocation, Profile, Rules};
use precis_core::Error;
use precis_core::{FreeformClass, StringClass};
use std::borrow::Cow;
use std::sync::LazyLock;

// Additional Mapping Rule: The additional mapping rule consists of
// the following sub-rules.
//  a. Map any instances of non-ASCII space to SPACE (`U+0020`); a
//     non-ASCII space is any Unicode code point having a general
//     category of "Zs", naturally with the exception of SPACE
//     (`U+0020`).  (The inclusion of only ASCII space prevents
//     confusion with various non-ASCII space code points, many of
//     which are difficult to reproduce across different input
//     methods.)
//
//  b. Remove any instances of the ASCII space character at the
//     beginning or end of a nickname.
//
//  c. Map interior sequences of more than one ASCII space character
//     to a single ASCII space character.
fn trim_spaces<'a, T>(s: T) -> Result<Cow<'a, str>, Error>
where
    T: Into<Cow<'a, str>>,
{
    let s = s.into();

    // First pass: check if transformation is needed to avoid allocation
    let needs_transform = {
        let mut begin = true;
        let mut pending_space = false;

        s.chars().any(|c| {
            if !common::is_space_separator(c) {
                pending_space = false;
                begin = false;
                false
            } else {
                // Need transform if: space at beginning, consecutive spaces, or non-ASCII space
                let needs = begin || pending_space || c != common::SPACE;
                if !begin {
                    pending_space = true;
                }
                begin = false;
                needs
            }
        }) || pending_space // Also need transform if string ends with space
    };

    if !needs_transform {
        return Ok(s);
    }

    // Second pass: transform the string
    let mut res = String::new();
    res.reserve(s.len());
    let mut begin = true;
    let mut pending_space = false;

    for c in s.chars() {
        if !common::is_space_separator(c) {
            // Add pending space before this character if there was one
            if pending_space {
                res.push(common::SPACE);
                pending_space = false;
            }
            res.push(c);
            begin = false;
        } else if !begin {
            // Mark that we have a space, but don't add it yet
            // (only add it when we see a non-space character)
            pending_space = true;
        }
    }

    // No need to remove trailing space - it was never added
    Ok(res.into())
}

/// [`Nickname`](https://datatracker.ietf.org/doc/html/rfc8266#section-2).
/// Nicknames or display names in messaging and text conferencing technologies;
/// pet names for devices, accounts, and people; and other uses of nicknames,
/// display names, or pet names. Look at the
/// [`IANA` Considerations](https://datatracker.ietf.org/doc/html/rfc8266#section-5)
/// section for more details.
/// # Example
/// ```rust
/// # use precis_core::profile::Profile;
/// # use precis_profiles::Nickname;
/// # use std::borrow::Cow;
/// // create Nickname profile
/// let profile = Nickname::new();
///
/// // prepare string
/// assert_eq!(profile.prepare("Guybrush Threepwood"),
///     Ok(Cow::from("Guybrush Threepwood")));
///
/// // enforce string
/// assert_eq!(profile.enforce("   Guybrush     Threepwood  "),
///     Ok(Cow::from("Guybrush Threepwood")));
///
/// // compare strings
/// assert_eq!(profile.compare("Guybrush   Threepwood  ",
///     "guybrush threepwood"), Ok(true));
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Nickname(FreeformClass);

impl Nickname {
    /// Creates a [`Nickname`] profile.
    pub fn new() -> Self {
        Self(FreeformClass::default())
    }

    fn apply_prepare_rules<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        let s = common::ensure_not_empty(s)?;
        self.0.allows(&s)?;
        Ok(s)
    }

    fn apply_enforce_rules<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        let s = self.apply_prepare_rules(s)?;
        let s = self.additional_mapping_rule(s)?;
        let s = self.normalization_rule(s)?;
        common::ensure_not_empty(s)
    }

    fn apply_compare_rules<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        let s = self.apply_prepare_rules(s)?;
        let s = self.additional_mapping_rule(s)?;
        let s = self.case_mapping_rule(s)?;
        self.normalization_rule(s)
    }
}

impl Profile for Nickname {
    fn prepare<'a, S>(&self, s: S) -> Result<Cow<'a, str>, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        self.apply_prepare_rules(s)
    }

    fn enforce<'a, S>(&self, s: S) -> Result<Cow<'a, str>, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        stabilize(s, |s| self.apply_enforce_rules(s))
    }

    fn compare<A, B>(&self, s1: A, s2: B) -> Result<bool, Error>
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        Ok(stabilize(s1.as_ref(), |s| self.apply_compare_rules(s))?
            == stabilize(s2.as_ref(), |s| self.apply_compare_rules(s))?)
    }
}

impl Rules for Nickname {
    fn additional_mapping_rule<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        trim_spaces(s)
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
        common::normalization_form_nfkc(s)
    }
}

fn get_nickname_profile() -> &'static Nickname {
    static NICKNAME: LazyLock<Nickname> = LazyLock::new(Nickname::default);
    &NICKNAME
}

impl PrecisFastInvocation for Nickname {
    fn prepare<'a, S>(s: S) -> Result<Cow<'a, str>, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        get_nickname_profile().prepare(s)
    }

    fn enforce<'a, S>(s: S) -> Result<Cow<'a, str>, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        get_nickname_profile().enforce(s)
    }

    fn compare<A, B>(s1: A, s2: B) -> Result<bool, Error>
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        get_nickname_profile().compare(s1, s2)
    }
}

#[cfg(test)]
mod test_nicknames {
    use crate::nicknames::*;

    #[test]
    fn test_trim_spaces() {
        // Check ASCII spaces
        assert_eq!(trim_spaces("  "), Ok(Cow::from("")));
        assert_eq!(trim_spaces(" test"), Ok(Cow::from("test")));
        assert_eq!(trim_spaces("test "), Ok(Cow::from("test")));

        assert_eq!(trim_spaces("hello  world"), Ok(Cow::from("hello world")));

        assert_eq!(trim_spaces(""), Ok(Cow::from("")));
        assert_eq!(trim_spaces(" test"), Ok(Cow::from("test")));
        assert_eq!(trim_spaces("test "), Ok(Cow::from("test")));
        assert_eq!(
            trim_spaces("   hello  world   "),
            Ok(Cow::from("hello world"))
        );

        // Check non-ASCII spaces
        assert_eq!(trim_spaces("\u{205f}test\u{205f}"), Ok(Cow::from("test")));
        assert_eq!(
            trim_spaces("\u{205f}\u{205f}hello\u{205f}\u{205f}world\u{205f}\u{205f}"),
            Ok(Cow::from("hello world"))
        );

        // Mix ASCII and non-ASCII spaces
        assert_eq!(trim_spaces(" \u{205f}test\u{205f} "), Ok(Cow::from("test")));
        assert_eq!(
            trim_spaces("\u{205f} hello \u{205f} world \u{205f} "),
            Ok(Cow::from("hello world"))
        );
    }

    #[test]
    fn nick_name_profile() {
        let profile = Nickname::new();

        let res = profile.prepare("Foo Bar");
        assert_eq!(res, Ok(Cow::from("Foo Bar")));

        let res = profile.enforce("Foo Bar");
        assert_eq!(res, Ok(Cow::from("Foo Bar")));

        let res = profile.compare("Foo Bar", "foo bar");
        assert_eq!(res, Ok(true));
    }
}
