//! Edge case tests for profile compare() functions
//!
//! Tests for comparison edge cases including:
//! - Empty string handling
//! - Identical strings
//! - Normalization equivalence
//! - Error propagation

use precis_core::profile::PrecisFastInvocation;
use precis_profiles::{Nickname, OpaqueString, UsernameCaseMapped, UsernameCasePreserved};

#[cfg(test)]
mod nickname_compare_tests {
    use super::*;

    #[test]
    fn test_both_empty() {
        // Both empty should error
        let result = Nickname::compare("", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_first_empty() {
        // First empty should error
        let result = Nickname::compare("", "hello");
        assert!(result.is_err());
    }

    #[test]
    fn test_second_empty() {
        // Second empty should error
        let result = Nickname::compare("hello", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_identical_strings() {
        // Identical ASCII strings
        let result = Nickname::compare("hello", "hello");
        assert_eq!(result, Ok(true));

        // Identical with Unicode
        let result = Nickname::compare("café", "café");
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_normalize_to_same() {
        // Different case, same after normalization
        let result = Nickname::compare("Hello", "hello");
        assert_eq!(result, Ok(true));

        // Different spacing, same after enforcement
        let result = Nickname::compare("  hello  world  ", "hello world");
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_different_strings() {
        let result = Nickname::compare("hello", "world");
        assert_eq!(result, Ok(false));

        let result = Nickname::compare("café", "cafe");
        assert_eq!(result, Ok(false));
    }
}

#[cfg(test)]
mod password_compare_tests {
    use super::*;

    #[test]
    fn test_both_empty() {
        // Both empty should error
        let result = OpaqueString::compare("", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_first_empty() {
        let result = OpaqueString::compare("", "password");
        assert!(result.is_err());
    }

    #[test]
    fn test_second_empty() {
        let result = OpaqueString::compare("password", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_identical_strings() {
        // Identical strings
        let result = OpaqueString::compare("password", "password");
        assert_eq!(result, Ok(true));

        // With Unicode
        let result = OpaqueString::compare("密碼", "密碼");
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_case_sensitive() {
        // Passwords are case-sensitive
        let result = OpaqueString::compare("Password", "password");
        assert_eq!(result, Ok(false));

        let result = OpaqueString::compare("PASS", "pass");
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_different_strings() {
        let result = OpaqueString::compare("password1", "password2");
        assert_eq!(result, Ok(false));
    }
}

#[cfg(test)]
mod username_mapped_compare_tests {
    use super::*;

    #[test]
    fn test_both_empty() {
        let result = UsernameCaseMapped::compare("", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_first_empty() {
        let result = UsernameCaseMapped::compare("", "user");
        assert!(result.is_err());
    }

    #[test]
    fn test_second_empty() {
        let result = UsernameCaseMapped::compare("user", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_identical_strings() {
        let result = UsernameCaseMapped::compare("user", "user");
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_case_insensitive() {
        // Case mapping makes comparison case-insensitive
        let result = UsernameCaseMapped::compare("User", "user");
        assert_eq!(result, Ok(true));

        let result = UsernameCaseMapped::compare("USER", "user");
        assert_eq!(result, Ok(true));

        let result = UsernameCaseMapped::compare("UsEr", "uSeR");
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_different_strings() {
        let result = UsernameCaseMapped::compare("user1", "user2");
        assert_eq!(result, Ok(false));
    }
}

#[cfg(test)]
mod username_preserved_compare_tests {
    use super::*;

    #[test]
    fn test_both_empty() {
        let result = UsernameCasePreserved::compare("", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_first_empty() {
        let result = UsernameCasePreserved::compare("", "user");
        assert!(result.is_err());
    }

    #[test]
    fn test_second_empty() {
        let result = UsernameCasePreserved::compare("user", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_identical_strings() {
        let result = UsernameCasePreserved::compare("user", "user");
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_case_sensitive() {
        // Case preserved means case-sensitive comparison
        let result = UsernameCasePreserved::compare("User", "user");
        assert_eq!(result, Ok(false));

        let result = UsernameCasePreserved::compare("USER", "user");
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_different_strings() {
        let result = UsernameCasePreserved::compare("user1", "user2");
        assert_eq!(result, Ok(false));
    }
}

#[cfg(test)]
mod normalization_compare_tests {
    use super::*;

    #[test]
    fn test_composed_vs_decomposed() {
        // é (U+00E9) vs e + combining acute (U+0065 U+0301)
        let composed = "café";
        let decomposed = "cafe\u{0301}";

        // After NFKC normalization, should be equal
        let result = Nickname::compare(composed, decomposed);
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_fullwidth_vs_normal() {
        // Fullwidth A (U+FF21) vs normal A (U+0041)
        let result = UsernameCaseMapped::compare("\u{FF21}", "a");
        assert_eq!(result, Ok(true)); // Both map to lowercase 'a'

        let result = UsernameCasePreserved::compare("\u{FF21}", "A");
        assert_eq!(result, Ok(true)); // Both map to 'A'
    }
}
