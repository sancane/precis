//! Property-based tests using proptest
//!
//! These tests verify invariants that should hold for all valid inputs,
//! such as idempotence, commutativity, and consistency of the PRECIS framework.

use precis_core::profile::PrecisFastInvocation;
use precis_profiles::{Nickname, OpaqueString, UsernameCaseMapped, UsernameCasePreserved};
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;

// Strategy to generate valid Unicode strings
fn unicode_string() -> impl Strategy<Value = String> {
    "[\\PC]{0,100}"
}

// Strategy to generate ASCII strings (more likely to be valid)
fn ascii_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9_-]{1,50}"
}

// Strategy to generate strings with spaces
fn string_with_spaces() -> impl Strategy<Value = String> {
    "[ a-zA-Z0-9_-]{1,50}"
}

// Strategy to generate international strings
fn international_string() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-zA-Z0-9]{1,50}",           // ASCII
        "[α-ωΑ-Ω]{1,30}",              // Greek
        "[а-яА-Я]{1,30}",              // Cyrillic
        "[一-龯]{1,20}",                // CJK
        "[ぁ-ん]{1,30}",                // Hiragana
    ]
}

#[cfg(test)]
mod nickname_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 1000,
            .. ProptestConfig::default()
        })]

        /// Property: enforce is idempotent
        /// enforce(enforce(s)) == enforce(s)
        #[test]
        fn enforce_is_idempotent(s in unicode_string()) {
            if let Ok(enforced1) = Nickname::enforce(&s) {
                let enforced2 = Nickname::enforce(enforced1.as_ref())?;
                prop_assert_eq!(enforced1.as_ref(), enforced2.as_ref());
            }
        }

        /// Property: compare is symmetric
        /// compare(a, b) == compare(b, a)
        #[test]
        fn compare_is_symmetric(a in ascii_string(), b in ascii_string()) {
            let result_ab = Nickname::compare(&a, &b);
            let result_ba = Nickname::compare(&b, &a);
            prop_assert_eq!(result_ab, result_ba);
        }

        /// Property: compare is reflexive
        /// compare(a, a) always returns Ok(true) for valid strings
        #[test]
        fn compare_is_reflexive(s in ascii_string()) {
            if Nickname::enforce(&s).is_ok() {
                let result = Nickname::compare(&s, &s);
                prop_assert_eq!(result, Ok(true));
            }
        }

        /// Property: enforce implies prepare would succeed
        /// If enforce(s) is Ok, then prepare(s) must be Ok
        /// (The reverse is not always true because enforce also trims spaces)
        #[test]
        fn enforce_ok_implies_prepare_ok(s in unicode_string()) {
            if Nickname::enforce(&s).is_ok() {
                prop_assert!(Nickname::prepare(&s).is_ok());
            }
        }

        /// Property: enforce result has no leading/trailing spaces
        /// Any string that passes enforce should have no surrounding spaces
        #[test]
        fn enforce_removes_spaces(s in string_with_spaces()) {
            if let Ok(enforced) = Nickname::enforce(&s) {
                let result = enforced.as_ref();
                prop_assert_eq!(result, result.trim());
            }
        }

        /// Property: international strings that are valid remain consistent
        #[test]
        fn international_consistency(s in international_string()) {
            let result1 = Nickname::enforce(&s);
            let result2 = Nickname::enforce(&s);

            // Both should have the same success/failure status
            prop_assert_eq!(result1.is_ok(), result2.is_ok());

            // If successful, results should be identical
            if let (Ok(r1), Ok(r2)) = (result1, result2) {
                prop_assert_eq!(r1, r2);
            }
        }
    }
}

#[cfg(test)]
mod username_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 1000,
            .. ProptestConfig::default()
        })]

        /// Property: UsernameCaseMapped enforce is idempotent
        #[test]
        fn username_casemapped_idempotent(s in ascii_string()) {
            if let Ok(enforced1) = UsernameCaseMapped::enforce(&s) {
                let enforced2 = UsernameCaseMapped::enforce(enforced1.as_ref())?;
                prop_assert_eq!(enforced1.as_ref(), enforced2.as_ref());
            }
        }

        /// Property: UsernameCaseMapped compare is case-insensitive
        #[test]
        fn username_casemapped_case_insensitive(s in ascii_string()) {
            if UsernameCaseMapped::enforce(&s).is_ok() {
                let upper = s.to_uppercase();
                let lower = s.to_lowercase();

                if UsernameCaseMapped::enforce(&upper).is_ok()
                    && UsernameCaseMapped::enforce(&lower).is_ok() {
                    let result = UsernameCaseMapped::compare(&upper, &lower);
                    prop_assert_eq!(result, Ok(true));
                }
            }
        }

        /// Property: UsernameCasePreserved compare is case-sensitive
        #[test]
        fn username_casepreserved_case_sensitive(s in "[a-zA-Z]{5,20}") {
            if UsernameCasePreserved::enforce(&s).is_ok() {
                let upper = s.to_uppercase();
                let lower = s.to_lowercase();

                if upper != lower
                    && UsernameCasePreserved::enforce(&upper).is_ok()
                    && UsernameCasePreserved::enforce(&lower).is_ok() {
                    let result = UsernameCasePreserved::compare(&upper, &lower);
                    // Should be false because case is different
                    prop_assert_eq!(result, Ok(false));
                }
            }
        }
    }
}

#[cfg(test)]
mod password_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 1000,
            .. ProptestConfig::default()
        })]

        /// Property: OpaqueString (password) enforce is idempotent
        #[test]
        fn password_idempotent(s in unicode_string()) {
            if let Ok(enforced1) = OpaqueString::enforce(&s) {
                let enforced2 = OpaqueString::enforce(enforced1.as_ref())?;
                prop_assert_eq!(enforced1.as_ref(), enforced2.as_ref());
            }
        }

        /// Property: OpaqueString compare is reflexive
        #[test]
        fn password_compare_reflexive(s in unicode_string()) {
            if OpaqueString::enforce(&s).is_ok() {
                let result = OpaqueString::compare(&s, &s);
                prop_assert_eq!(result, Ok(true));
            }
        }

        /// Property: Empty strings are invalid for all profiles
        #[test]
        fn empty_string_invalid(_s in Just(())) {
            prop_assert!(Nickname::enforce("").is_err());
            prop_assert!(UsernameCaseMapped::enforce("").is_err());
            prop_assert!(OpaqueString::enforce("").is_err());
        }
    }
}

#[cfg(test)]
mod cross_profile_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 500,
            .. ProptestConfig::default()
        })]

        /// Property: Transitivity of compare
        /// If compare(a,b) and compare(b,c) are both Ok(true), then compare(a,c) should be Ok(true)
        #[test]
        fn compare_transitivity(a in ascii_string(), b in ascii_string(), c in ascii_string()) {
            // Test with UsernameCaseMapped (case-insensitive)
            if UsernameCaseMapped::enforce(&a).is_ok()
                && UsernameCaseMapped::enforce(&b).is_ok()
                && UsernameCaseMapped::enforce(&c).is_ok() {

                let ab = UsernameCaseMapped::compare(&a, &b);
                let bc = UsernameCaseMapped::compare(&b, &c);
                let ac = UsernameCaseMapped::compare(&a, &c);

                if ab == Ok(true) && bc == Ok(true) {
                    prop_assert_eq!(ac, Ok(true));
                }
            }
        }

        /// Property: Consistency across profiles
        /// If a string is valid ASCII, it should be valid in all profiles
        #[test]
        fn ascii_valid_everywhere(s in "[a-z0-9]{1,20}") {
            let nickname_ok = Nickname::enforce(&s).is_ok();
            let username_ok = UsernameCaseMapped::enforce(&s).is_ok();
            let password_ok = OpaqueString::enforce(&s).is_ok();

            // Simple ASCII should work in all profiles
            prop_assert!(nickname_ok);
            prop_assert!(username_ok);
            prop_assert!(password_ok);
        }

        /// Property: Determinism - same input always produces same output
        #[test]
        fn deterministic_behavior(s in unicode_string()) {
            let result1 = Nickname::enforce(&s);
            let result2 = Nickname::enforce(&s);
            let result3 = Nickname::enforce(&s);

            prop_assert_eq!(result1.is_ok(), result2.is_ok());
            prop_assert_eq!(result2.is_ok(), result3.is_ok());

            if let (Ok(r1), Ok(r2), Ok(r3)) = (result1, result2, result3) {
                prop_assert_eq!(r1.as_ref(), r2.as_ref());
                prop_assert_eq!(r2.as_ref(), r3.as_ref());
            }
        }
    }
}

#[cfg(test)]
mod normalization_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 500,
            .. ProptestConfig::default()
        })]

        /// Property: Normalization produces stable output
        /// Normalized strings should not change when normalized again
        #[test]
        fn normalization_stable(s in unicode_string()) {
            // Test with Nickname which applies normalization
            if let Ok(result1) = Nickname::prepare(&s) {
                if let Ok(result2) = Nickname::prepare(result1.as_ref()) {
                    prop_assert_eq!(result1.as_ref(), result2.as_ref());
                }
            }
        }

        /// Property: Length relationship
        /// After enforcement, string length should be <= original (due to trimming)
        #[test]
        fn enforce_reduces_or_maintains_length(s in string_with_spaces()) {
            if let Ok(result) = Nickname::enforce(&s) {
                prop_assert!(result.len() <= s.len());
            }
        }

        /// Property: Character set preservation
        /// Enforce should not introduce characters that weren't in the input
        /// (except through normalization which is expected)
        #[test]
        fn no_unexpected_characters(s in "[a-zA-Z ]{1,20}") {
            if let Ok(result) = Nickname::enforce(&s) {
                // All characters in result should be ASCII letters or spaces
                // (spaces only if internal)
                for (i, c) in result.chars().enumerate() {
                    let is_first = i == 0;
                    let is_last = i == result.chars().count() - 1;

                    if is_first || is_last {
                        prop_assert!(c.is_ascii_alphabetic(), "unexpected char at boundary: {:?}", c);
                    } else {
                        prop_assert!(c.is_ascii_alphabetic() || c == ' ', "unexpected char: {:?}", c);
                    }
                }
            }
        }
    }
}
