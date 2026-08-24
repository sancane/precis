//! UTF-8 boundary edge case tests
//!
//! Tests for proper handling of multibyte UTF-8 characters at string boundaries.
//! Covers:
//! - Emoji (4-byte sequences) at start/end
//! - Combining characters after multibyte characters
//! - Non-ASCII spaces at boundaries
//! - Context rule characters (ZWNJ/ZWJ) with multibyte
//! - BiDi markers with CJK characters
//! - Trimming operations around multibyte characters

use precis_core::profile::PrecisFastInvocation;
use precis_profiles::{Nickname, OpaqueString, UsernameCasePreserved};

#[cfg(test)]
mod emoji_boundaries {
    use super::*;

    #[test]
    fn test_emoji_at_start() {
        // 4-byte emoji at the start
        let input = "😀hello";
        let result = Nickname::prepare(input);
        // Emojis should be allowed in Nickname
        assert!(result.is_ok());
    }

    #[test]
    fn test_emoji_at_end() {
        // 4-byte emoji at the end
        let input = "hello😀";
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_emoji_only() {
        // String with only emoji
        let input = "😀";
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_emoji_sequence() {
        // Multiple 4-byte emojis
        let input = "😀😁😂";
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_emoji_with_combining() {
        // Emoji with combining character
        let input = "😀\u{0301}";
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod combining_after_multibyte {
    use super::*;

    #[test]
    fn test_combining_after_2byte() {
        // 2-byte character (ñ = U+00F1) with additional combining
        let input = "ñ\u{0301}"; // ñ + combining acute
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_combining_after_3byte() {
        // 3-byte character (CJK) with combining mark
        let input = "文\u{0300}"; // CJK + combining grave
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_combining_after_4byte() {
        // 4-byte character with combining
        let input = "𝕳\u{0301}"; // Mathematical bold H + combining
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_combining_after_multibyte() {
        // Multiple combining marks after multibyte
        let input = "é\u{0300}\u{0302}"; // é + grave + circumflex
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod nonascii_spaces_boundaries {
    use super::*;

    #[test]
    fn test_nbsp_at_start() {
        // Non-breaking space (U+00A0) at start - should be trimmed by enforce
        let input = "\u{00A0}hello";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_nbsp_at_end() {
        // Non-breaking space at end - should be trimmed by enforce
        let input = "hello\u{00A0}";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_ideographic_space_at_start() {
        // Ideographic space (U+3000) at start
        let input = "\u{3000}hello";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_ideographic_space_at_end() {
        // Ideographic space at end
        let input = "hello\u{3000}";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_en_quad_at_boundaries() {
        // En quad (U+2000) at both boundaries
        let input = "\u{2000}hello\u{2000}";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_mixed_spaces_at_start() {
        // Mix of ASCII and non-ASCII spaces at start
        let input = " \u{00A0}\u{2000}hello";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }
}

#[cfg(test)]
mod context_chars_with_multibyte {
    use super::*;

    #[test]
    fn test_zwnj_after_emoji() {
        // ZWNJ (U+200C) after emoji
        // Per RFC 5892 Appendix A.1, ZWNJ requires specific joining context
        // Emoji doesn't provide the required context, so this should fail
        let input = "😀\u{200C}test";
        let result = Nickname::prepare(input);
        assert!(
            result.is_err(),
            "ZWNJ after emoji should fail context rules"
        );
    }

    #[test]
    fn test_zwj_after_emoji() {
        // ZWJ (U+200D) after emoji - common in modern emoji sequences
        // Per RFC 5892 Appendix A.2, ZWJ requires specific joining context
        // Emoji doesn't satisfy the joiner context requirements
        let input = "😀\u{200D}test";
        let result = Nickname::prepare(input);
        assert!(result.is_err(), "ZWJ after emoji should fail context rules");
    }

    #[test]
    fn test_zwnj_between_multibyte() {
        // ZWNJ requires specific joining context per RFC 5892 A.1:
        // Before: Joining_Type L or D
        // After: Joining_Type R or D
        // Beh (U+0628) is dual-joining (D), Teh (U+062A) is dual-joining (D)
        // D satisfies both L/D requirement before and R/D requirement after
        let input = "ب\u{200C}ت"; // Beh + ZWNJ + Teh (both dual-joining)
        let result = Nickname::prepare(input);
        assert!(
            result.is_ok(),
            "ZWNJ between dual-joining Arabic letters should succeed"
        );
    }

    #[test]
    fn test_middle_dot_with_catalan() {
        // Middle dot (U+00B7) with Catalan l·l
        let input = "l\u{00B7}l";
        let result = UsernameCasePreserved::prepare(input);
        // Middle dot has specific context rules
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod bidi_with_multibyte {
    use super::*;

    #[test]
    fn test_rtl_marker_with_cjk() {
        // RLM (U+200F - Right-to-Left Mark) with CJK characters
        // RLM is a format control character (category Cf) which is DISALLOWED
        // in PRECIS FreeformClass per RFC 8264
        let input = "\u{200F}文字";
        let result = Nickname::prepare(input);
        assert!(result.is_err(), "RLM (format control) should be disallowed");
    }

    #[test]
    fn test_ltr_marker_with_arabic() {
        // LRM (U+200E - Left-to-Right Mark) with Arabic
        // LRM is a format control character (category Cf) which is DISALLOWED
        // in PRECIS FreeformClass per RFC 8264
        let input = "\u{200E}العربية";
        let result = Nickname::prepare(input);
        assert!(result.is_err(), "LRM (format control) should be disallowed");
    }

    #[test]
    fn test_arabic_with_emoji() {
        // Arabic text with emoji
        let input = "مرحبا😀";
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_hebrew_with_emoji() {
        // Hebrew text with emoji
        let input = "שלום😀";
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod trim_around_multibyte {
    use super::*;

    #[test]
    fn test_trim_spaces_around_emoji() {
        // Spaces around emoji should be trimmed by enforce
        let input = "  😀  ";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "😀");
    }

    #[test]
    fn test_trim_tabs_around_cjk() {
        // Tabs are control characters and will be rejected
        let input = "\t文字\t";
        let result = Nickname::prepare(input);
        // Tabs (U+0009) are disallowed
        assert!(result.is_err());
    }

    #[test]
    fn test_only_multibyte_after_trim() {
        // After trimming, only multibyte chars remain
        let input = "  こんにちは  ";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "こんにちは");
    }

    #[test]
    fn test_emoji_sequence_with_spaces() {
        // Emoji sequence with internal spaces
        let input = "😀 😁 😂";
        let result = Nickname::prepare(input);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod space_mapping_with_multibyte {
    use super::*;

    #[test]
    fn test_double_space_after_multibyte() {
        // The mapping rule has to start at a character boundary, whatever the
        // width of the characters preceding the spaces it collapses.
        assert_eq!(Nickname::enforce("ÜÜ  X").unwrap(), "ÜÜ X");
        assert_eq!(Nickname::enforce("ΑΒΓΔ  Ε").unwrap(), "ΑΒΓΔ Ε");
        assert_eq!(Nickname::enforce("文字  A").unwrap(), "文字 A");
        assert_eq!(Nickname::enforce("😀  A").unwrap(), "😀 A");
        assert_eq!(Nickname::enforce("  ÜÜ  X  ").unwrap(), "ÜÜ X");
    }

    #[test]
    fn test_interior_non_ascii_space_is_mapped() {
        // RFC 8266, section 2.1, sub-rule a maps a non-ASCII space to SPACE.
        // An interior one is mapped, not removed.
        assert_eq!(Nickname::enforce("Ab\u{A0}Cd").unwrap(), "Ab Cd");
        assert_eq!(Nickname::enforce("\u{A0}Ab\u{A0}Cd").unwrap(), "Ab Cd");
        assert_eq!(Nickname::enforce("文字\u{3000}A").unwrap(), "文字 A");
        assert_eq!(Nickname::enforce("Ab\u{A0}\u{2000}Cd").unwrap(), "Ab Cd");
    }

    #[test]
    fn test_space_introduced_by_normalization() {
        // The profile applies its rules until the string is stable, and NFKC
        // decomposes U+00A8 into SPACE followed by U+0308. The mapping rule
        // therefore sees spaces that the input did not contain, behind
        // characters of any width.
        assert_eq!(Nickname::enforce("ÜÜ \u{A8}").unwrap(), "ÜÜ \u{308}");
        assert_eq!(
            Nickname::enforce("ÜÜ\u{A8} \u{A8}").unwrap(),
            "ÜÜ \u{308} \u{308}"
        );
        assert_eq!(Nickname::enforce("\u{A8}").unwrap(), "\u{308}");
    }
}

#[cfg(test)]
mod password_multibyte {
    use super::*;

    #[test]
    fn test_password_emoji_boundaries() {
        // Passwords should preserve emoji at boundaries
        let input = "😀pass😀";
        let result = OpaqueString::prepare(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_multibyte_only() {
        // Password with only multibyte characters
        let input = "密碼";
        let result = OpaqueString::prepare(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_mixed_multibyte() {
        // Mix of ASCII and multibyte in password
        let input = "pass密碼😀";
        let result = OpaqueString::prepare(input);
        assert!(result.is_ok());
    }
}
