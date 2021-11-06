include!(concat!(env!("OUT_DIR"), "/precis_tables.rs"));

use crate::stringclasses::DerivedPropertyValue;
use crate::Codepoints;
use phf::phf_map;
use std::char;
use std::cmp::Ordering;
use unicode_normalization::UnicodeNormalization;

// 9.6.  Exceptions (F)
// This category explicitly lists code points for which the category
// cannot be assigned using only the core property values that exist in
// the Unicode standard.  The values are according to the table below:
static EXCEPTIONS: phf::Map<u32, DerivedPropertyValue> = phf_map! {
    // PVALID -- Would otherwise have been DISALLOWED
    0x00DFu32 => DerivedPropertyValue::PValid,     // LATIN SMALL LETTER SHARP S
    0x03C2u32 => DerivedPropertyValue::PValid,     // GREEK SMALL LETTER FINAL SIGMA
    0x06FDu32 => DerivedPropertyValue::PValid,     // ARABIC SIGN SINDHI AMPERSAND
    0x06FEu32 => DerivedPropertyValue::PValid,     // ARABIC SIGN SINDHI POSTPOSITION MEN
    0x0F0Bu32 => DerivedPropertyValue::PValid,     // TIBETAN MARK INTERSYLLABIC TSHEG
    0x3007u32 => DerivedPropertyValue::PValid,     // IDEOGRAPHIC NUMBER ZERO

    // CONTEXTO -- Would otherwise have been DISALLOWED
    0x00B7u32 => DerivedPropertyValue::ContextO,   // MIDDLE DOT
    0x0375u32 => DerivedPropertyValue::ContextO,   // GREEK LOWER NUMERAL SIGN (KERAIA)
    0x05F3u32 => DerivedPropertyValue::ContextO,   // HEBREW PUNCTUATION GERESH
    0x05F4u32 => DerivedPropertyValue::ContextO,   // HEBREW PUNCTUATION GERSHAYIM
    0x30FBu32 => DerivedPropertyValue::ContextO,   // KATAKANA MIDDLE DOT

    // CONTEXTO -- Would otherwise have been PVALID
    0x0660u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT ZERO
    0x0661u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT ONE
    0x0662u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT TWO
    0x0663u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT THREE
    0x0664u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT FOUR
    0x0665u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT FIVE
    0x0666u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT SIX
    0x0667u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT SEVEN
    0x0668u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT EIGHT
    0x0669u32 => DerivedPropertyValue::ContextO,   // ARABIC-INDIC DIGIT NINE
    0x06F0u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT ZERO
    0x06F1u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT ONE
    0x06F2u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT TWO
    0x06F3u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT THREE
    0x06F4u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT FOUR
    0x06F5u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT FIVE
    0x06F6u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT SIX
    0x06F7u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT SEVEN
    0x06F8u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT EIGHT
    0x06F9u32 => DerivedPropertyValue::ContextO,   // EXTENDED ARABIC-INDIC DIGIT NINE

    // DISALLOWED -- Would otherwise have been PVALID
    0x0640u32 => DerivedPropertyValue::Disallowed, // ARABIC TATWEEL
    0x07FAu32 => DerivedPropertyValue::Disallowed, // NKO LAJANYALAN
    0x302Eu32 => DerivedPropertyValue::Disallowed, // HANGUL SINGLE DOT TONE MARK
    0x302Fu32 => DerivedPropertyValue::Disallowed, // HANGUL DOUBLE DOT TONE MARK
    0x3031u32 => DerivedPropertyValue::Disallowed, // VERTICAL KANA REPEAT MARK
    0x3032u32 => DerivedPropertyValue::Disallowed, // VERTICAL KANA REPEAT WITH VOICED SOUND MARK
    0x3033u32 => DerivedPropertyValue::Disallowed, // VERTICAL KANA REPEAT MARK UPPER HALF
    0x3034u32 => DerivedPropertyValue::Disallowed, // VERTICAL KANA REPEAT WITH VOICED SOUND MARK UPPER HA
    0x3035u32 => DerivedPropertyValue::Disallowed, // VERTICAL KANA REPEAT MARK LOWER HALF
    0x303Bu32 => DerivedPropertyValue::Disallowed, // VERTICAL IDEOGRAPHIC ITERATION MARK
};

pub fn get_exception_val(cp: u32) -> Option<&'static DerivedPropertyValue> {
    EXCEPTIONS.get(&cp)
}

// 9.7.  BackwardCompatible (G)
// This category includes the code points that property values in
// versions of Unicode after 5.2 have changed in such a way that the
// derived property value would no longer be PVALID or DISALLOWED.  If
// changes are made to future versions of Unicode so that code points
// might change the property value from PVALID or DISALLOWED, then this
// table can be updated and keep special exception values so that the
// property values for code points stay stable.
static BACKWARD_COMPATIBLE: phf::Map<u32, DerivedPropertyValue> = phf_map! {};

pub fn get_backward_compatible_val(cp: u32) -> Option<&'static DerivedPropertyValue> {
    BACKWARD_COMPATIBLE.get(&cp)
}

fn is_in_table(cp: u32, table: &[Codepoints]) -> bool {
    table
        .binary_search_by(|cps| match cps {
            Codepoints::Single(c) => c.cmp(&cp),
            Codepoints::Range(r) => {
                if r.contains(&cp) {
                    Ordering::Equal
                } else if cp < *r.start() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        })
        .is_ok()
}

pub fn is_letter_digit(cp: u32) -> bool {
    is_in_table(cp, &LOWERCASE_LETTER)
        || is_in_table(cp, &UPPERCASE_LETTER)
        || is_in_table(cp, &OTHER_LETTER)
        || is_in_table(cp, &DECIMAL_NUMBER)
        || is_in_table(cp, &MODIFIER_LETTER)
        || is_in_table(cp, &NONSPACING_MARK)
        || is_in_table(cp, &SPACING_MARK)
}

pub fn is_join_control(cp: u32) -> bool {
    is_in_table(cp, &JOIN_CONTROL)
}

pub fn is_old_hangul_jamo(cp: u32) -> bool {
    is_in_table(cp, &LEADING_JAMO)
        || is_in_table(cp, &VOWEL_JAMO)
        || is_in_table(cp, &TRAILING_JAMO)
}

pub fn is_unassigned(cp: u32) -> bool {
    !is_in_table(cp, &NONCHARACTER_CODE_POINT) && is_in_table(cp, &UNASSIGNED)
}

pub fn is_ascii7(cp: u32) -> bool {
    is_in_table(cp, &ASCII7)
}

pub fn is_control(cp: u32) -> bool {
    is_in_table(cp, &CONTROL)
}

pub fn is_precis_ignorable_property(cp: u32) -> bool {
    is_in_table(cp, &DEFAULT_IGNORABLE_CODE_POINT) || is_in_table(cp, &NONCHARACTER_CODE_POINT)
}

pub fn is_space(cp: u32) -> bool {
    is_in_table(cp, &SPACE_SEPARATOR)
}

pub fn is_symbol(cp: u32) -> bool {
    is_in_table(cp, &MATH_SYMBOL)
        || is_in_table(cp, &CURRENCY_SYMBOL)
        || is_in_table(cp, &MODIFIER_SYMBOL)
        || is_in_table(cp, &OTHER_SYMBOL)
}

pub fn is_punctuation(cp: u32) -> bool {
    is_in_table(cp, &CONNECTOR_PUNCTUATION)
        || is_in_table(cp, &DASH_PUNCTUATION)
        || is_in_table(cp, &OPEN_PUNCTUATION)
        || is_in_table(cp, &CLOSE_PUNCTUATION)
        || is_in_table(cp, &INITIAL_PUNCTUATION)
        || is_in_table(cp, &FINAL_PUNCTUATION)
        || is_in_table(cp, &OTHER_PUNCTUATION)
}

pub fn is_other_letter_digit(cp: u32) -> bool {
    is_in_table(cp, &TITLECASE_LETTER)
        || is_in_table(cp, &LETTER_NUMBER)
        || is_in_table(cp, &OTHER_NUMBER)
        || is_in_table(cp, &ENCLOSING_MARK)
}

pub fn has_compat(cp: u32) -> bool {
    let c: char = match char::from_u32(cp) {
        Some(c) => c,
        None => return false,
    };

    // This PRECIS-specific category is used to group any code point that is
    // decomposed and recomposed into something other than itself under
    // Unicode Normalization Form KC.
    // Typically, this category is true of code points that are
    // "compatibility decomposable characters" as defined in the Unicode
    // Standard.

    let cs = c.to_string();
    cs != cs.nfkc().collect::<String>()
}

pub fn is_virama(cp: u32) -> bool {
    is_in_table(cp, &VIRAMA)
}

pub fn is_greek(cp: u32) -> bool {
    is_in_table(cp, &GREEK)
}

pub fn is_hebrew(cp: u32) -> bool {
    is_in_table(cp, &HEBREW)
}

pub fn is_hiragana(cp: u32) -> bool {
    is_in_table(cp, &HIRAGANA)
}

pub fn is_katakana(cp: u32) -> bool {
    is_in_table(cp, &KATAKANA)
}
pub fn is_han(cp: u32) -> bool {
    is_in_table(cp, &HAN)
}

pub fn is_dual_joining(cp: u32) -> bool {
    is_in_table(cp, &DUAL_JOINING)
}

pub fn is_left_joining(cp: u32) -> bool {
    is_in_table(cp, &LEFT_JOINING)
}

pub fn is_right_joining(cp: u32) -> bool {
    is_in_table(cp, &RIGHT_JOINING)
}

pub fn is_transparent(cp: u32) -> bool {
    is_in_table(cp, &TRANSPARENT)
}

#[cfg(test)]
mod tests {
    use crate::common::*;

    #[test]
    fn find_codepoint() {
        let empty_table: [Codepoints; 0] = [];
        assert_eq!(is_in_table(34, &empty_table), false);

        let test_table: [Codepoints; 1] = [Codepoints::Single(11)];
        assert_eq!(is_in_table(11, &test_table), true);
        assert_eq!(is_in_table(12, &test_table), false);

        let test_table: [Codepoints; 2] = [Codepoints::Single(1), Codepoints::Single(10)];
        assert_eq!(is_in_table(1, &test_table), true);
        assert_eq!(is_in_table(10, &test_table), true);
        assert_eq!(is_in_table(5, &test_table), false);

        let test_table: [Codepoints; 3] = [
            Codepoints::Single(1),
            Codepoints::Single(5),
            Codepoints::Single(10),
        ];
        assert_eq!(is_in_table(1, &test_table), true);
        assert_eq!(is_in_table(5, &test_table), true);
        assert_eq!(is_in_table(10, &test_table), true);
        assert_eq!(is_in_table(7, &test_table), false);

        let test_table: [Codepoints; 1] =
            [Codepoints::Range(std::ops::RangeInclusive::new(10, 20))];
        for i in 10..=20 {
            assert_eq!(is_in_table(i, &test_table), true);
        }
        assert_eq!(is_in_table(9, &test_table), false);
        assert_eq!(is_in_table(21, &test_table), false);

        let test_table: [Codepoints; 4] = [
            Codepoints::Range(std::ops::RangeInclusive::new(2, 4)),
            Codepoints::Single(7),
            Codepoints::Range(std::ops::RangeInclusive::new(10, 20)),
            Codepoints::Single(25),
        ];
        for i in 2..=4 {
            assert_eq!(is_in_table(i, &test_table), true);
        }
        for i in 10..=20 {
            assert_eq!(is_in_table(i, &test_table), true);
        }
        assert_eq!(is_in_table(7, &test_table), true);
        assert_eq!(is_in_table(25, &test_table), true);
        assert_eq!(is_in_table(1, &test_table), false);
        assert_eq!(is_in_table(8, &test_table), false);
        assert_eq!(is_in_table(26, &test_table), false);

        let test_table: [Codepoints; 2] = [
            Codepoints::Range(std::ops::RangeInclusive::new(2, 4)),
            Codepoints::Range(std::ops::RangeInclusive::new(10, 20)),
        ];
        for i in 2..=4 {
            assert_eq!(is_in_table(i, &test_table), true);
        }
        for i in 10..=20 {
            assert_eq!(is_in_table(i, &test_table), true);
        }
        assert_eq!(is_in_table(1, &test_table), false);
        assert_eq!(is_in_table(21, &test_table), false);
    }

    #[test]
    fn compat() {
        let cp = 0x1e9a;
        assert_eq!(has_compat(cp), true);

        let cp = 0x006a;
        assert_eq!(has_compat(cp), false);
    }
}
