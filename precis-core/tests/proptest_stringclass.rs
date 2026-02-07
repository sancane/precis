//! Property-based tests for StringClass using proptest
//!
//! These tests verify fundamental properties of Unicode character classification
//! according to RFC 8264 PRECIS Framework.

use precis_core::{DerivedPropertyValue, FreeformClass, IdentifierClass, StringClass};
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;

// Strategy to generate Unicode codepoints
fn unicode_codepoint() -> impl Strategy<Value = u32> {
    0x0000u32..=0x10FFFFu32
}

// Strategy to generate valid Unicode strings
fn unicode_string() -> impl Strategy<Value = String> {
    "[\\PC]{0,100}"
}

// Strategy to generate ASCII strings
fn ascii_string() -> impl Strategy<Value = String> {
    "[\\x00-\\x7F]{1,50}"
}

#[cfg(test)]
mod stringclass_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 5000,
            .. ProptestConfig::default()
        })]

        /// Property: get_value_from_char and get_value_from_codepoint are consistent
        /// For any valid Unicode character, both methods should return the same value
        #[test]
        fn char_codepoint_consistency(cp in 0x0000u32..=0xD7FFu32) {
            // Skip surrogate range (0xD800-0xDFFF)
            if let Some(c) = char::from_u32(cp) {
                let id_class = IdentifierClass::default();
                let from_char = id_class.get_value_from_char(c);
                let from_codepoint = id_class.get_value_from_codepoint(cp);
                prop_assert_eq!(from_char, from_codepoint);
            }
        }

        /// Property: FreeformClass accepts everything IdentifierClass accepts
        /// IdentifierClass is more restrictive than FreeformClass
        #[test]
        fn freeform_more_permissive(s in ascii_string()) {
            let id_class = IdentifierClass::default();
            let ff_class = FreeformClass::default();

            let id_result = id_class.allows(&s);
            let ff_result = ff_class.allows(&s);

            // If IdentifierClass accepts it, FreeformClass must too
            if id_result.is_ok() {
                prop_assert!(ff_result.is_ok());
            }
        }

        /// Property: Classification is deterministic
        /// Same codepoint always produces same DerivedPropertyValue
        #[test]
        fn classification_deterministic(cp in unicode_codepoint()) {
            let id_class = IdentifierClass::default();

            let result1 = id_class.get_value_from_codepoint(cp);
            let result2 = id_class.get_value_from_codepoint(cp);
            let result3 = id_class.get_value_from_codepoint(cp);

            prop_assert_eq!(result1, result2);
            prop_assert_eq!(result2, result3);
        }

        /// Property: PValid characters are always accepted
        #[test]
        fn pvalid_always_accepted(cp in 0x0061u32..=0x007Au32) {
            // ASCII lowercase letters are always PValid
            if let Some(c) = char::from_u32(cp) {
                let id_class = IdentifierClass::default();
                let value = id_class.get_value_from_char(c);

                prop_assert_eq!(value, DerivedPropertyValue::PValid);

                let s = c.to_string();
                prop_assert!(id_class.allows(&s).is_ok());
            }
        }

        /// Property: Disallowed codepoints are rejected
        #[test]
        fn disallowed_rejected(cp in 0x0000u32..=0x001Fu32) {
            // Control characters are disallowed
            if let Some(c) = char::from_u32(cp) {
                let id_class = IdentifierClass::default();
                let s = c.to_string();
                let result = id_class.allows(&s);

                prop_assert!(result.is_err());
            }
        }

        /// Property: allows() is consistent with repeated calls
        #[test]
        fn allows_consistent(s in unicode_string()) {
            let id_class = IdentifierClass::default();

            let result1 = id_class.allows(&s);
            let result2 = id_class.allows(&s);

            prop_assert_eq!(result1.is_ok(), result2.is_ok());
        }
    }
}

#[cfg(test)]
mod derived_property_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 1000,
            .. ProptestConfig::default()
        })]

        /// Property: DerivedPropertyValue enum is exhaustive
        /// Every codepoint must have one of the defined property values
        #[test]
        fn all_codepoints_classified(cp in unicode_codepoint()) {
            let id_class = IdentifierClass::default();
            let value = id_class.get_value_from_codepoint(cp);

            // Must be one of the defined values
            prop_assert!(matches!(value,
                DerivedPropertyValue::PValid |
                DerivedPropertyValue::SpecClassPval |
                DerivedPropertyValue::SpecClassDis |
                DerivedPropertyValue::Disallowed |
                DerivedPropertyValue::ContextJ |
                DerivedPropertyValue::ContextO |
                DerivedPropertyValue::Unassigned
            ));
        }

        /// Property: IdentifierClass and FreeformClass differ on spaces
        #[test]
        fn classes_differ_on_spaces(_s in Just(())) {
            let id_class = IdentifierClass::default();
            let ff_class = FreeformClass::default();

            let space_cp = 0x0020u32; // ASCII space

            let id_value = id_class.get_value_from_codepoint(space_cp);
            let ff_value = ff_class.get_value_from_codepoint(space_cp);

            // IdentifierClass: space is SpecClassDis
            prop_assert_eq!(id_value, DerivedPropertyValue::SpecClassDis);

            // FreeformClass: space is SpecClassPval
            prop_assert_eq!(ff_value, DerivedPropertyValue::SpecClassPval);
        }

        /// Property: ASCII letters are PValid in both classes
        #[test]
        fn ascii_letters_pvalid(cp in prop_oneof![
            0x0041u32..=0x005Au32,  // A-Z
            0x0061u32..=0x007Au32,  // a-z
        ]) {
            let id_class = IdentifierClass::default();
            let ff_class = FreeformClass::default();

            let id_value = id_class.get_value_from_codepoint(cp);
            let ff_value = ff_class.get_value_from_codepoint(cp);

            prop_assert_eq!(id_value, DerivedPropertyValue::PValid);
            prop_assert_eq!(ff_value, DerivedPropertyValue::PValid);
        }

        /// Property: ASCII digits are PValid in both classes
        #[test]
        fn ascii_digits_pvalid(cp in 0x0030u32..=0x0039u32) {
            // 0-9
            let id_class = IdentifierClass::default();
            let ff_class = FreeformClass::default();

            let id_value = id_class.get_value_from_codepoint(cp);
            let ff_value = ff_class.get_value_from_codepoint(cp);

            prop_assert_eq!(id_value, DerivedPropertyValue::PValid);
            prop_assert_eq!(ff_value, DerivedPropertyValue::PValid);
        }
    }
}

#[cfg(test)]
mod error_handling_properties {
    use super::*;
    use precis_core::Error;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 500,
            .. ProptestConfig::default()
        })]

        /// Property: Error contains useful information
        /// When allows() fails, the error should contain the problematic codepoint
        #[test]
        fn error_contains_codepoint_info(cp in 0x0000u32..=0x001Fu32) {
            // Control characters should fail
            if let Some(c) = char::from_u32(cp) {
                let id_class = IdentifierClass::default();
                let s = c.to_string();

                if let Err(Error::BadCodepoint(info)) = id_class.allows(&s) {
                    prop_assert_eq!(info.cp, cp);
                    prop_assert_eq!(info.position, 0);
                }
            }
        }

        /// Property: Position in error is accurate
        /// When a string has an invalid character, the position should be correct
        #[test]
        fn error_position_accurate(prefix_len in 0usize..10) {
            let mut s = "a".repeat(prefix_len);
            s.push('\u{0000}'); // Add control character

            let id_class = IdentifierClass::default();

            if let Err(Error::BadCodepoint(info)) = id_class.allows(&s) {
                prop_assert_eq!(info.position, prefix_len);
            }
        }

        /// Property: Valid prefix followed by invalid char is caught
        #[test]
        fn catches_invalid_after_valid(valid_len in 1usize..20) {
            let valid = "a".repeat(valid_len);
            let invalid = format!("{}\u{0000}", valid);

            let id_class = IdentifierClass::default();

            prop_assert!(id_class.allows(&valid).is_ok());
            prop_assert!(id_class.allows(&invalid).is_err());
        }
    }
}

#[cfg(test)]
mod unicode_ranges_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 1000,
            .. ProptestConfig::default()
        })]

        /// Property: Basic Multilingual Plane characters are classified
        /// BMP (U+0000 to U+FFFF) should all have classifications
        #[test]
        fn bmp_all_classified(cp in 0x0000u32..=0xD7FFu32) {
            // Skip surrogate range
            let id_class = IdentifierClass::default();
            let _value = id_class.get_value_from_codepoint(cp);
            // Should not panic
        }

        /// Property: Supplementary planes are handled
        /// Characters beyond BMP (U+10000 to U+10FFFF)
        #[test]
        fn supplementary_planes_handled(cp in 0x10000u32..=0x10FFFFu32) {
            let id_class = IdentifierClass::default();
            let _value = id_class.get_value_from_codepoint(cp);
            // Should not panic
        }

        /// Property: Surrogate codepoints (0xD800-0xDFFF) cannot form valid chars
        #[test]
        fn surrogate_range_invalid(cp in 0xD800u32..=0xDFFFu32) {
            // These should not be valid Unicode characters
            prop_assert!(char::from_u32(cp).is_none());
        }

        /// Property: Private Use Area characters are handled
        #[test]
        fn private_use_handled(cp in prop_oneof![
            0xE000u32..=0xF8FFu32,      // BMP Private Use
            0xF0000u32..=0xFFFFFu32,    // Supplementary Private Use A
            0x100000u32..=0x10FFFFu32,  // Supplementary Private Use B
        ]) {
            let id_class = IdentifierClass::default();
            let value = id_class.get_value_from_codepoint(cp);

            // Private use should be unassigned or disallowed
            prop_assert!(matches!(value,
                DerivedPropertyValue::Unassigned |
                DerivedPropertyValue::Disallowed
            ));
        }
    }
}

#[cfg(test)]
mod consistency_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
            cases: 500,
            .. ProptestConfig::default()
        })]

        /// Property: Empty strings are handled consistently
        #[test]
        fn empty_string_handled(_s in Just(())) {
            let id_class = IdentifierClass::default();
            let ff_class = FreeformClass::default();

            // Empty string should be accepted (contains no invalid codepoints)
            prop_assert!(id_class.allows("").is_ok());
            prop_assert!(ff_class.allows("").is_ok());
        }

        /// Property: Single character strings work correctly
        #[test]
        fn single_char_works(c in "[a-z]") {
            let id_class = IdentifierClass::default();

            let result = id_class.allows(&c);
            prop_assert!(result.is_ok());
        }

        /// Property: Concatenation of valid strings
        /// If allows(a) and allows(b) are Ok, then allows(a+b) should be Ok
        #[test]
        fn concatenation_preserves_validity(a in "[a-z]{1,10}", b in "[a-z]{1,10}") {
            let id_class = IdentifierClass::default();

            let a_ok = id_class.allows(&a).is_ok();
            let b_ok = id_class.allows(&b).is_ok();
            let ab = format!("{}{}", a, b);
            let ab_ok = id_class.allows(&ab).is_ok();

            if a_ok && b_ok {
                prop_assert!(ab_ok);
            }
        }
    }
}
