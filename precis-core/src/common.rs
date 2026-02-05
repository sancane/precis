include!(concat!(env!("OUT_DIR"), "/precis_tables.rs"));
include!(concat!(env!("OUT_DIR"), "/context_tables.rs"));

use crate::Codepoints;
use crate::DerivedPropertyValue;
use std::char;
use unicode_normalization::UnicodeNormalization;

/// Helper function to perform binary search on a table of (Codepoints, T) tuples
/// and return the associated value if found.
#[inline]
fn binary_search_codepoints_table<T>(table: &[(Codepoints, T)], cp: u32) -> Option<&T> {
    table
        .binary_search_by(|(cps, _)| {
            // Use PartialOrd<u32> which correctly handles range containment
            // This always returns Some(_) for valid codepoints
            cps.partial_cmp(&cp)
                .expect("Codepoints::partial_cmp should always return Some")
        })
        .ok()
        .map(|idx| &table[idx].1)
}

#[inline]
pub(crate) fn get_exception_val(cp: u32) -> Option<&'static DerivedPropertyValue> {
    binary_search_codepoints_table(&EXCEPTIONS, cp)
}

#[inline]
pub(crate) fn get_backward_compatible_val(cp: u32) -> Option<&'static DerivedPropertyValue> {
    binary_search_codepoints_table(&BACKWARD_COMPATIBLE, cp)
}

#[inline]
fn is_in_table(cp: u32, table: &[Codepoints]) -> bool {
    table
        .binary_search_by(|cps| {
            // Use PartialOrd<u32> which correctly handles range containment
            // This always returns Some(_) for valid codepoints
            cps.partial_cmp(&cp)
                .expect("Codepoints::partial_cmp should always return Some")
        })
        .is_ok()
}

#[inline]
pub(crate) fn is_letter_digit(cp: u32) -> bool {
    is_in_table(cp, &LOWERCASE_LETTER)
        || is_in_table(cp, &UPPERCASE_LETTER)
        || is_in_table(cp, &OTHER_LETTER)
        || is_in_table(cp, &DECIMAL_NUMBER)
        || is_in_table(cp, &MODIFIER_LETTER)
        || is_in_table(cp, &NONSPACING_MARK)
        || is_in_table(cp, &SPACING_MARK)
}

#[inline]
pub(crate) fn is_join_control(cp: u32) -> bool {
    is_in_table(cp, &JOIN_CONTROL)
}

#[inline]
pub(crate) fn is_old_hangul_jamo(cp: u32) -> bool {
    is_in_table(cp, &LEADING_JAMO)
        || is_in_table(cp, &VOWEL_JAMO)
        || is_in_table(cp, &TRAILING_JAMO)
}

#[inline]
pub(crate) fn is_unassigned(cp: u32) -> bool {
    !is_in_table(cp, &NONCHARACTER_CODE_POINT) && is_in_table(cp, &UNASSIGNED)
}

#[inline]
pub(crate) fn is_ascii7(cp: u32) -> bool {
    is_in_table(cp, &ASCII7)
}

#[inline]
pub(crate) fn is_control(cp: u32) -> bool {
    is_in_table(cp, &CONTROL)
}

#[inline]
pub(crate) fn is_precis_ignorable_property(cp: u32) -> bool {
    is_in_table(cp, &DEFAULT_IGNORABLE_CODE_POINT) || is_in_table(cp, &NONCHARACTER_CODE_POINT)
}

#[inline]
pub(crate) fn is_space(cp: u32) -> bool {
    is_in_table(cp, &SPACE_SEPARATOR)
}

#[inline]
pub(crate) fn is_symbol(cp: u32) -> bool {
    is_in_table(cp, &MATH_SYMBOL)
        || is_in_table(cp, &CURRENCY_SYMBOL)
        || is_in_table(cp, &MODIFIER_SYMBOL)
        || is_in_table(cp, &OTHER_SYMBOL)
}

#[inline]
pub(crate) fn is_punctuation(cp: u32) -> bool {
    is_in_table(cp, &CONNECTOR_PUNCTUATION)
        || is_in_table(cp, &DASH_PUNCTUATION)
        || is_in_table(cp, &OPEN_PUNCTUATION)
        || is_in_table(cp, &CLOSE_PUNCTUATION)
        || is_in_table(cp, &INITIAL_PUNCTUATION)
        || is_in_table(cp, &FINAL_PUNCTUATION)
        || is_in_table(cp, &OTHER_PUNCTUATION)
}

#[inline]
pub(crate) fn is_other_letter_digit(cp: u32) -> bool {
    is_in_table(cp, &TITLECASE_LETTER)
        || is_in_table(cp, &LETTER_NUMBER)
        || is_in_table(cp, &OTHER_NUMBER)
        || is_in_table(cp, &ENCLOSING_MARK)
}

pub(crate) fn has_compat(cp: u32) -> bool {
    let c: char = match char::from_u32(cp) {
        Some(c) => c,
        None => return false,
    };

    // This PRECIS-specific category is used to group any code point that is
    // decomposed and recomposed into something other than itself under
    // Unicode Normalization Form KC.
    // Typically, this category is true of code points that are
    // "compatibility `decomposable` characters" as defined in the Unicode
    // Standard.

    // Optimize: Check if NFKC normalization changes the character
    // without allocating strings. If the normalized form produces
    // exactly one character equal to the original, it has no compat mapping.
    // We check by consuming the first two elements of the iterator:
    // - If first char equals original and second is None => no compat
    // - Otherwise => has compat
    let mut normalized = c.nfkc();
    !matches!((normalized.next(), normalized.next()), (Some(nc), None) if nc == c)
}

#[inline]
pub(crate) fn is_virama(cp: u32) -> bool {
    is_in_table(cp, &VIRAMA)
}

#[inline]
pub(crate) fn is_greek(cp: u32) -> bool {
    is_in_table(cp, &GREEK)
}

#[inline]
pub(crate) fn is_hebrew(cp: u32) -> bool {
    is_in_table(cp, &HEBREW)
}

#[inline]
pub(crate) fn is_hiragana(cp: u32) -> bool {
    is_in_table(cp, &HIRAGANA)
}

#[inline]
pub(crate) fn is_katakana(cp: u32) -> bool {
    is_in_table(cp, &KATAKANA)
}

#[inline]
pub(crate) fn is_han(cp: u32) -> bool {
    is_in_table(cp, &HAN)
}

#[inline]
pub(crate) fn is_dual_joining(cp: u32) -> bool {
    is_in_table(cp, &DUAL_JOINING)
}

#[inline]
pub(crate) fn is_left_joining(cp: u32) -> bool {
    is_in_table(cp, &LEFT_JOINING)
}

#[inline]
pub(crate) fn is_right_joining(cp: u32) -> bool {
    is_in_table(cp, &RIGHT_JOINING)
}

#[inline]
pub(crate) fn is_transparent(cp: u32) -> bool {
    is_in_table(cp, &TRANSPARENT)
}

#[cfg(test)]
mod tests {
    use crate::common::*;

    #[test]
    fn find_codepoint() {
        let empty_table: [Codepoints; 0] = [];
        assert!(!is_in_table(34, &empty_table));

        let test_table: [Codepoints; 1] = [Codepoints::Single(11)];
        assert!(is_in_table(11, &test_table));
        assert!(!is_in_table(12, &test_table));

        let test_table: [Codepoints; 2] = [Codepoints::Single(1), Codepoints::Single(10)];
        assert!(is_in_table(1, &test_table));
        assert!(is_in_table(10, &test_table));
        assert!(!is_in_table(5, &test_table));

        let test_table: [Codepoints; 3] = [
            Codepoints::Single(1),
            Codepoints::Single(5),
            Codepoints::Single(10),
        ];
        assert!(is_in_table(1, &test_table));
        assert!(is_in_table(5, &test_table));
        assert!(is_in_table(10, &test_table));
        assert!(!is_in_table(7, &test_table));

        let test_table: [Codepoints; 1] =
            [Codepoints::Range(std::ops::RangeInclusive::new(10, 20))];
        for i in 10..=20 {
            assert!(is_in_table(i, &test_table));
        }
        assert!(!is_in_table(9, &test_table));
        assert!(!is_in_table(21, &test_table));

        let test_table: [Codepoints; 4] = [
            Codepoints::Range(std::ops::RangeInclusive::new(2, 4)),
            Codepoints::Single(7),
            Codepoints::Range(std::ops::RangeInclusive::new(10, 20)),
            Codepoints::Single(25),
        ];
        for i in 2..=4 {
            assert!(is_in_table(i, &test_table));
        }
        for i in 10..=20 {
            assert!(is_in_table(i, &test_table));
        }
        assert!(is_in_table(7, &test_table));
        assert!(is_in_table(25, &test_table));
        assert!(!is_in_table(1, &test_table));
        assert!(!is_in_table(8, &test_table));
        assert!(!is_in_table(26, &test_table));

        let test_table: [Codepoints; 2] = [
            Codepoints::Range(std::ops::RangeInclusive::new(2, 4)),
            Codepoints::Range(std::ops::RangeInclusive::new(10, 20)),
        ];
        for i in 2..=4 {
            assert!(is_in_table(i, &test_table));
        }
        for i in 10..=20 {
            assert!(is_in_table(i, &test_table));
        }
        assert!(!is_in_table(1, &test_table));
        assert!(!is_in_table(21, &test_table));
    }

    #[test]
    fn compat() {
        let cp = 0x1e9a;
        assert!(has_compat(cp));

        let cp = 0x006a;
        assert!(!has_compat(cp));
    }

    // Character classification tests
    #[test]
    fn test_letter_digit_classification() {
        // Lowercase letters
        assert!(is_letter_digit('a' as u32));
        assert!(is_letter_digit('z' as u32));

        // Uppercase letters
        assert!(is_letter_digit('A' as u32));
        assert!(is_letter_digit('Z' as u32));

        // Decimal digits
        assert!(is_letter_digit('0' as u32));
        assert!(is_letter_digit('9' as u32));

        // Not symbols or punctuation
        assert!(!is_letter_digit('+' as u32));
        assert!(!is_letter_digit('.' as u32));
    }

    #[test]
    fn test_symbol_classification() {
        // Math symbols
        assert!(is_symbol('+' as u32));
        assert!(is_symbol('=' as u32));

        // Currency symbols
        assert!(is_symbol('$' as u32));
        assert!(is_symbol('€' as u32));

        // Not letters
        assert!(!is_symbol('a' as u32));
        assert!(!is_symbol('5' as u32));
    }

    #[test]
    fn test_punctuation_classification() {
        // Common punctuation
        assert!(is_punctuation('.' as u32));
        assert!(is_punctuation(',' as u32));
        assert!(is_punctuation('!' as u32));
        assert!(is_punctuation('?' as u32));

        // Brackets
        assert!(is_punctuation('(' as u32));
        assert!(is_punctuation(')' as u32));

        // Not letters
        assert!(!is_punctuation('a' as u32));
        assert!(!is_punctuation('5' as u32));
    }

    #[test]
    fn test_special_categories() {
        // Join control
        assert!(is_join_control(0x200C)); // ZWNJ
        assert!(is_join_control(0x200D)); // ZWJ

        // ASCII7
        assert!(is_ascii7('a' as u32));
        assert!(is_ascii7('Z' as u32));
        assert!(!is_ascii7('ñ' as u32));

        // Control characters
        assert!(is_control(0x0009)); // TAB
        assert!(is_control(0x000A)); // LF
        assert!(!is_control('a' as u32));
    }

    #[test]
    fn test_script_classification() {
        // Greek
        assert!(is_greek('α' as u32));
        assert!(is_greek('Σ' as u32));
        assert!(!is_greek('a' as u32));

        // Hebrew
        assert!(is_hebrew('א' as u32));
        assert!(is_hebrew('ב' as u32));
        assert!(!is_hebrew('a' as u32));

        // Han (CJK)
        assert!(is_han('文' as u32));
        assert!(is_han('字' as u32));
        assert!(!is_han('a' as u32));
    }

    #[test]
    fn test_space_classification() {
        // Space separator
        assert!(is_space(' ' as u32));
        assert!(is_space(0x00A0)); // NO-BREAK SPACE
        assert!(is_space(0x3000)); // IDEOGRAPHIC SPACE

        // Not spaces
        assert!(!is_space('\t' as u32)); // TAB
        assert!(!is_space('a' as u32));
    }
}
