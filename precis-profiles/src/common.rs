include!(concat!(env!("OUT_DIR"), "/space_separator.rs"));

use precis_core::Codepoints;
use precis_core::Error;
use std::borrow::Cow;
use unicode_normalization::UnicodeNormalization;

pub(crate) const SPACE: char = '\u{0020}';

pub(crate) fn is_space_separator(c: char) -> bool {
    let cp = c as u32;
    SPACE_SEPARATOR
        .binary_search_by(|cps| cps.partial_cmp(&cp).unwrap())
        .is_ok()
}

pub(crate) fn is_non_ascii_space(c: char) -> bool {
    c != SPACE && is_space_separator(c)
}

/// Helper function to transform a string starting from the first position where
/// a predicate matches, avoiding allocation if no transformation is needed.
///
/// # Arguments
/// * `s` - Input string
/// * `predicate` - Function to find the first position to transform
/// * `transform` - Function to transform each character from that position
///
/// # Returns
/// Original string if predicate never matches, or transformed string otherwise
pub(crate) fn transform_from_first_match<'a, T, P, F>(
    s: T,
    predicate: P,
    mut transform: F,
) -> Result<Cow<'a, str>, Error>
where
    T: Into<Cow<'a, str>>,
    P: Fn(char) -> bool,
    F: FnMut(char, &mut String),
{
    let s = s.into();
    match s.find(predicate) {
        None => Ok(s),
        Some(pos) => {
            let mut res = String::from(&s[..pos]);
            res.reserve(s.len() - res.len());
            for c in s[pos..].chars() {
                transform(c, &mut res);
            }
            Ok(res.into())
        }
    }
}

/// Ensures a string is not empty, returning an error if it is.
///
/// # Arguments
/// * `s` - Input string
///
/// # Returns
/// The input string if not empty, or Error::Invalid if empty
pub(crate) fn ensure_not_empty<'a, T>(s: T) -> Result<Cow<'a, str>, Error>
where
    T: Into<Cow<'a, str>>,
{
    let s = s.into();
    (!s.is_empty()).then_some(s).ok_or(Error::Invalid)
}

pub(crate) fn normalization_form_nfkc<'a, T>(s: T) -> Result<Cow<'a, str>, Error>
where
    T: Into<Cow<'a, str>>,
{
    // Checking if this string is normalized can help us to save memory
    // allocations but this will make this function a bit less efficient
    // due to the extra iteration over the string
    let s = s.into();
    if unicode_normalization::is_nfkc(&s) {
        Ok(s)
    } else {
        Ok(s.nfkc().collect::<String>().into())
    }
}

pub(crate) fn normalization_form_nfc<'a, T>(s: T) -> Result<Cow<'a, str>, Error>
where
    T: Into<Cow<'a, str>>,
{
    // Checking if this string is normalized can help us to save memory
    // allocations but this will make this function a bit less efficient
    // due to the extra iteration over the string
    let s = s.into();
    if unicode_normalization::is_nfc(&s) {
        Ok(s)
    } else {
        Ok(s.nfc().collect::<String>().into())
    }
}

pub(crate) fn case_mapping_rule<'a, T>(s: T) -> Result<Cow<'a, str>, Error>
where
    T: Into<Cow<'a, str>>,
{
    transform_from_first_match(s, char::is_uppercase, |c, res| {
        if c.is_lowercase() {
            res.push(c);
        } else {
            c.to_lowercase().for_each(|x| res.push(x));
        }
    })
}

#[cfg(test)]
mod profile_rules {
    use crate::common::*;

    #[test]
    fn test_normalization_rule() {
        let res = normalization_form_nfc("");
        assert_eq!(res, Ok(Cow::from("")));

        let res = normalization_form_nfc("\u{212b}");
        assert_eq!(res, Ok(Cow::from("\u{00c5}")));

        let res = normalization_form_nfc("a\u{212b}");
        assert_eq!(res, Ok(Cow::from("a\u{00c5}")));

        let res = normalization_form_nfc("\u{212b}a");
        assert_eq!(res, Ok(Cow::from("\u{00c5}a")));

        let res = normalization_form_nfc("\u{212b}\u{2126}\u{1e0b}\u{0323}");
        assert_eq!(res, Ok(Cow::from("\u{00c5}\u{03a9}\u{1e0d}\u{0307}")));
    }

    #[test]
    fn test_case_mapping_rule() {
        let res = case_mapping_rule("");
        assert_eq!(res, Ok(Cow::from("")));

        let res = case_mapping_rule("T");
        assert_eq!(res, Ok(Cow::from("t")));

        let res = case_mapping_rule("aT");
        assert_eq!(res, Ok(Cow::from("at")));

        let res = case_mapping_rule("Ta");
        assert_eq!(res, Ok(Cow::from("ta")));

        let res = case_mapping_rule("TestUserName");
        assert_eq!(res, Ok(Cow::from("testusername")));
    }

    #[test]
    fn test_is_non_ascii_space() {
        // Check ASCII space
        assert!(!is_non_ascii_space(SPACE));

        // Check character not in `Space_Separator` general category
        assert!(!is_non_ascii_space('a'));

        // Check character in `Space_Separator` that is not `0x0020`
        assert!(is_non_ascii_space('\u{205f}'));
    }
}
