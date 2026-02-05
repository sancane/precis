//! StringClass validation edge case tests
//!
//! Tests for StringClass validation behavior with various edge cases.
//! Note: These tests focus on StringClass.allows() validation, not full
//! Profile stabilization (which includes normalization). For full profile
//! stabilization tests, see the integration tests in precis-profiles.
//!
//! Covers edge cases including:
//! - Empty strings
//! - Single characters (ASCII and Unicode)
//! - Combining character sequences
//! - Multiple passes through validation
//! - Idempotency of validation

use precis_core::profile::Profile;
use precis_core::{Error, FreeformClass, IdentifierClass, StringClass};
use std::borrow::Cow;

/// Simple test profile that validates strings using IdentifierClass
/// without applying normalization transformations.
///
/// This is a minimal Profile implementation used for testing StringClass
/// validation behavior in isolation.
struct ValidationTestProfile {
    class: IdentifierClass,
}

impl ValidationTestProfile {
    fn new() -> Self {
        Self {
            class: IdentifierClass::default(),
        }
    }
}

impl Profile for ValidationTestProfile {
    fn prepare<'a, S>(&self, s: S) -> Result<Cow<'a, str>, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        let s = s.into();
        // Validate using the IdentifierClass
        self.class.allows(&s)?;
        Ok(s)
    }

    fn enforce<'a, S>(&self, s: S) -> Result<Cow<'a, str>, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        let s = s.into();
        // Use IdentifierClass for validation
        self.class.allows(&s)?;
        Ok(s)
    }

    fn compare<A, B>(&self, s1: A, s2: B) -> Result<bool, Error>
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let p1 = self.enforce(s1.as_ref())?;
        let p2 = self.enforce(s2.as_ref())?;
        Ok(p1 == p2)
    }
}

#[test]
fn test_validation_ascii_lowercase() {
    // ASCII lowercase string with underscore (all valid in IdentifierClass)
    let profile = ValidationTestProfile::new();
    let input = "already_stable";
    let result = profile.enforce(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), input);
}

#[test]
fn test_validation_alphanumeric() {
    // Simple alphanumeric string (all valid in IdentifierClass)
    let profile = ValidationTestProfile::new();
    let input = "simple123";
    let result = profile.enforce(input);
    assert!(result.is_ok());
}

#[test]
fn test_validation_empty_string() {
    // Empty string passes StringClass validation (no disallowed codepoints)
    // but may be rejected at the Profile level depending on requirements
    let profile = ValidationTestProfile::new();
    let input = "";
    let result = profile.enforce(input);
    // StringClass.allows("") returns Ok, so this test expects Ok
    assert!(
        result.is_ok(),
        "Empty string should pass StringClass validation"
    );
}

#[test]
fn test_validation_single_ascii_char() {
    // Single ASCII lowercase character (valid in IdentifierClass)
    let profile = ValidationTestProfile::new();
    let input = "a";
    let result = profile.enforce(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "a");
}

#[test]
fn test_validation_single_unicode_char() {
    // Single Unicode character (ñ) - valid in IdentifierClass
    let profile = ValidationTestProfile::new();
    let input = "ñ";
    let result = profile.enforce(input);
    assert!(result.is_ok());
}

#[test]
fn test_validation_combining_diacritics() {
    // Base character with multiple combining diacritical marks
    // All are valid in IdentifierClass (category Mn - Mark, nonspacing)
    let profile = ValidationTestProfile::new();
    // e + combining acute + combining circumflex
    let input = "e\u{0301}\u{0302}";
    let result = profile.enforce(input);
    assert!(result.is_ok());
}

#[test]
fn test_validation_multiple_combining_marks() {
    // Multiple combining marks in sequence - all valid for IdentifierClass
    let profile = ValidationTestProfile::new();
    // a + grave + acute + circumflex
    let input = "a\u{0300}\u{0301}\u{0302}";
    let result = profile.enforce(input);
    assert!(result.is_ok());
}

#[test]
fn test_validation_complex_combining_sequence() {
    // Complex combining sequences with diaeresis and macron
    let profile = ValidationTestProfile::new();
    // o + diaeresis + macron (ṏ)
    let input = "o\u{0308}\u{0304}";
    let result = profile.enforce(input);
    assert!(result.is_ok());
}

#[test]
fn test_validation_composed_unicode() {
    // Pre-composed Unicode characters (café uses composed é: U+00E9)
    let profile = ValidationTestProfile::new();
    let input = "café";
    let result = profile.enforce(input);
    assert!(result.is_ok());
}

#[test]
fn test_validation_idempotent() {
    // Validation is idempotent - applying twice gives same result
    // (Note: this would be more meaningful with normalization)
    let profile = ValidationTestProfile::new();
    let input = "test\u{0301}";
    let first = profile.enforce(input).unwrap();
    let second = profile.enforce(first.as_ref()).unwrap();
    assert_eq!(first, second);
}

#[cfg(test)]
mod stringclass_validation {
    use super::*;

    #[test]
    fn test_identifier_class_empty() {
        let class = IdentifierClass::default();
        // Empty string is valid at StringClass level (no disallowed codepoints)
        // Profile-level rules may reject it for semantic reasons
        let result = class.allows("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_identifier_class_simple() {
        let class = IdentifierClass::default();
        // Simple ASCII lowercase string (all PVALID in IdentifierClass)
        let result = class.allows("simple");
        assert!(result.is_ok());
    }

    #[test]
    fn test_freeform_class_empty() {
        let class = FreeformClass::default();
        // Empty string is valid at StringClass level
        let result = class.allows("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_freeform_class_with_spaces() {
        let class = FreeformClass::default();
        // FreeformClass allows spaces (category Zs - SpecClassPval)
        let result = class.allows("hello world");
        assert!(result.is_ok());
    }
}
