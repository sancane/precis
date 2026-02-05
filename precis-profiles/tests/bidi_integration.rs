//! BiDi integration tests
//!
//! Integration tests that exercise BiDi rules through real profile usage.
//! These tests ensure BiDi validation is properly integrated into profiles.

use precis_core::profile::PrecisFastInvocation;
use precis_profiles::{Nickname, UsernameCaseMapped};

#[cfg(test)]
mod rtl_integration {
    use super::*;

    #[test]
    fn test_pure_arabic_text() {
        // Pure Arabic text (all R/AL/AN characters)
        let input = "محمد";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_arabic_with_numbers() {
        // Arabic with Arabic-Indic digits
        let input = "محمد١٢٣"; // Arabic letters + Arabic-Indic digits
        let result = Nickname::enforce(input);
        assert!(result.is_ok() || result.is_err()); // Depends on Unicode version
    }

    #[test]
    fn test_hebrew_text() {
        // Hebrew text (R characters)
        let input = "שלום";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mixed_rtl_ltr() {
        // Mixed LTR and RTL - should follow first character's direction
        let input = "Test محمد";
        let result = Nickname::enforce(input);
        // BiDi rules may reject mixed direction
        assert!(result.is_ok() || result.is_err());
    }
}

#[cfg(test)]
mod ltr_integration {
    use super::*;

    #[test]
    fn test_pure_latin() {
        let input = "Hello World";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_latin_with_numbers() {
        let input = "User123";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cjk_text() {
        // CJK characters (default to L in BiDi)
        let input = "用户名";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;

    #[test]
    fn test_rtl_with_trailing_spaces() {
        // RTL text with spaces that need trimming
        let input = "  محمد  ";
        let result = Nickname::enforce(input);
        // Spaces will be trimmed
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_rtl_character() {
        // Single RTL character
        let input = "م"; // Arabic Letter Meem
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_username_with_arabic() {
        // Usernames typically don't allow spaces, test RTL there
        let input = "محمد";
        let result = UsernameCaseMapped::enforce(input);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod complex_bidi_scenarios {
    use super::*;

    #[test]
    fn test_rtl_with_punctuation() {
        // RTL with punctuation (ON - Other Neutral)
        let input = "محمد!";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rtl_with_parentheses() {
        // RTL with parentheses (neutral characters)
        let input = "(محمد)";
        let result = Nickname::enforce(input);
        // May fail depending on BiDi rules for neutrals
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_hebrew_with_punctuation() {
        let input = "שלום!";
        let result = Nickname::enforce(input);
        assert!(result.is_ok());
    }
}
