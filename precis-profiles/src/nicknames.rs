use crate::common;
use lazy_static::lazy_static;
use precis_core::profile::stabilize;
use precis_core::profile::{PrecisFastInvocation, Profile, Rules};
use precis_core::Error;
use precis_core::{FreeformClass, StringClass};
use std::borrow::Cow;

fn find_disallowed_space(label: &str) -> Option<usize> {
    let mut begin = true;
    let mut prev_space = false;
    let mut last_c: Option<char> = None;
    let mut offset = 0;

    for (index, c) in label.chars().enumerate() {
        offset = index;
        if !common::is_space_separator(c) {
            last_c = Some(c);
            prev_space = false;
            begin = false;
            continue;
        }

        if begin {
            // Starts with space
            return Some(index);
        }

        if prev_space {
            // More than one separator
            return Some(index);
        }

        if c == common::SPACE {
            prev_space = true;
            last_c = Some(c);
        } else {
            // non-ASCII space
            return Some(index);
        }
    }

    // last character is a space
    if let Some(common::SPACE) = last_c {
        return Some(offset - 1);
    }

    None
}

fn trim_spaces<'a, T>(s: T) -> Result<Cow<'a, str>, Error>
where
    T: Into<Cow<'a, str>>,
{
    let s = s.into();
    match find_disallowed_space(&s) {
        None => Ok(s),
        Some(pos) => {
            let mut res = String::from(&s[..pos]);
            res.reserve(s.len() - res.len());
            let mut begin = true;
            let mut prev_space = false;
            for c in s[pos..].chars() {
                if !common::is_space_separator(c) {
                    res.push(c);
                    prev_space = false;
                    begin = false;
                    continue;
                }

                if begin {
                    // skip spaces at the beginning
                    continue;
                }

                if !prev_space {
                    res.push(common::SPACE);
                }

                prev_space = true;
            }
            // Skip last space character
            if let Some(c) = res.pop() {
                if c != common::SPACE {
                    res.push(c);
                }
            }
            Ok(res.into())
        }
    }
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
pub struct Nickname {
    class: FreeformClass,
}

impl Nickname {
    /// Creates a [`Nickname`] profile.
    pub fn new() -> Self {
        Self {
            class: FreeformClass {},
        }
    }

    fn apply_prepare_rules<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        let s = s.into();
        let s = (!s.is_empty()).then(|| s).ok_or(Error::Invalid)?;
        self.class.allows(&s)?;
        Ok(s)
    }

    fn apply_enforce_rules<'a, T>(&self, s: T) -> Result<Cow<'a, str>, Error>
    where
        T: Into<Cow<'a, str>>,
    {
        let s = self.apply_prepare_rules(s)?;
        let s = self.additional_mapping_rule(s)?;
        let s = self.normalization_rule(s)?;
        (!s.is_empty()).then(|| s).ok_or(Error::Invalid)
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

impl Default for Nickname {
    fn default() -> Self {
        Nickname::new()
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

    fn compare<S>(&self, s1: S, s2: S) -> Result<bool, Error>
    where
        S: AsRef<str>,
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
    lazy_static! {
        static ref NICKNAME: Nickname = Nickname::new();
    }
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

    fn compare<S>(s1: S, s2: S) -> Result<bool, Error>
    where
        S: AsRef<str>,
    {
        get_nickname_profile().compare(s1, s2)
    }
}

#[cfg(test)]
mod nickname {
    use crate::nicknames::*;

    #[test]
    fn test_find_disallowed_space() {
        assert_eq!(find_disallowed_space(""), None);
        assert_eq!(find_disallowed_space("test"), None);

        // Two ASCII spaces in a row
        assert_eq!(find_disallowed_space("  test"), Some(0));
        assert_eq!(find_disallowed_space("t  est"), Some(2));

        // Starts with ASCII space
        assert_eq!(find_disallowed_space(" test"), Some(0));

        // Non ASCII separator
        assert_eq!(find_disallowed_space("te\u{00a0}st"), Some(2));
        assert_eq!(find_disallowed_space("test\u{00a0}"), Some(4));
    }

    #[test]
    fn test_trim_spaces() {
        // Check ASCII spaces
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
}
