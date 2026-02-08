include!(concat!(env!("OUT_DIR"), "/bidi_class.rs"));

use precis_core::Codepoints;

#[inline]
fn bidi_class_cp(cp: u32) -> BidiClass {
    match BIDI_CLASS_TABLE.binary_search_by(|(cps, _)| cps.partial_cmp(&cp).unwrap()) {
        Ok(idx) => BIDI_CLASS_TABLE[idx].1,
        // `UCD/extracted/DerivedBidiClass.txt`: "All code points not explicitly listed
        // for `Bidi_Class` have the value `Left_To_Right` (L)."
        Err(_) => BidiClass::L,
    }
}

#[inline]
fn bidi_class(c: char) -> BidiClass {
    bidi_class_cp(c as u32)
}

/// From `rfc5893` Right-to-Left Scripts for Internationalized Domain Names for Applications (`IDNA`)
/// An `RTL` label is a label that contains at least one character of type R, AL, or AN.
#[inline]
pub(crate) fn has_rtl(label: &str) -> bool {
    label
        .find(|c| matches!(bidi_class(c), BidiClass::R | BidiClass::AL | BidiClass::AN))
        .is_some()
}

// From `rfc5893` Right-to-Left Scripts for Internationalized Domain Names for Applications (`IDNA`)
// Section 2. The `Bidi` rule
// The following rule, consisting of six conditions, applies to labels
// in `Bidi` domain names.  The requirements that this rule satisfies are
// described in Section 3.  All the conditions must be satisfied for
// the rule to be satisfied.
//
// 1.  The first character must be a character with `Bidi` property `L`, `R`,
//     or `AL`.  If it has the `R` or `AL` property, it is an `RTL` label; if it
//     has the `L` property, it is an `LTR` label.
//
// 2.  In an `RTL` label, only characters with the `Bidi` properties `R`, `AL`,
//     `AN`, `EN`, `ES`, `CS`, `ET`, `ON`, `BN`, or `NSM` are allowed.
//
// 3.  In an `RTL` label, the end of the label must be a character with
//     `Bidi` property `R`, `AL`, `EN`, or `AN`, followed by zero or more
//     characters with `Bidi` property `NSM`.
//
// 4.  In an `RTL` label, if an `EN` is present, no `AN` may be present, and
//     vice versa.
//
// 5.  In an `LTR` label, only characters with the `Bidi` properties `L`, `EN`,
//     `ES`, `CS`, `ET`, `ON`, `BN`, or `NSM` are allowed.
//
// 6.  In an `LTR` label, the end of the label must be a character with
//     `Bidi` property `L` or `EN`, followed by zero or more characters with
//     `Bidi` property `NSM`.
pub(crate) fn satisfy_bidi_rule(label: &str) -> bool {
    let mut it = label.chars();

    if let Some(c) = it.next() {
        let first = bidi_class(c);
        // rule 1. First character can only be `L`, `R` or `AL`
        if matches!(first, BidiClass::R | BidiClass::AL) {
            // this is a `RTL` label
            is_valid_rtl_label(it, first)
        } else if first == BidiClass::L {
            // this is a `LTR` label
            is_valid_ltr_label(it, first)
        } else {
            // char no in [`L`, `R` or `AL`]
            false
        }
    } else {
        // empty label
        true
    }
}

fn is_valid_rtl_label<I>(it: I, prev: BidiClass) -> bool
where
    I: IntoIterator<Item = char>,
{
    let mut prev = prev;
    let mut nsm = false;
    let mut en = false;
    let mut an = false;

    for c in it {
        let class = bidi_class(c);
        // rule 2.
        // In an `RTL` label, only characters with the `Bidi` properties `R`, `AL`,
        // `AN`, `EN`, `ES`, `CS`, `ET`, `ON`, `BN`, or `NSM` are allowed.
        match class {
            BidiClass::R
            | BidiClass::AL
            | BidiClass::ES
            | BidiClass::CS
            | BidiClass::ET
            | BidiClass::ON
            | BidiClass::BN => {}
            BidiClass::AN => {
                if en {
                    // rule 4.
                    // if an `EN` is present, no `AN` may be present
                    return false;
                }
                an = true;
            }
            BidiClass::EN => {
                if an {
                    // rule 4.
                    // if an `AN` is present, no `EN` may be present
                    return false;
                }
                en = true;
            }
            BidiClass::NSM => {
                // rule 3.
                // In an `RTL` label, the end of the label must be a character with
                // `Bidi` property `R`, `AL`, `EN`, or `AN`, followed by zero or more
                // characters with `Bidi` property `NSM`.
                if !matches!(
                    prev,
                    BidiClass::R | BidiClass::AL | BidiClass::EN | BidiClass::AN
                ) {
                    // char not in [`R`, `AL`, `EN`, or `AN`]
                    return false;
                }
                nsm = true;
                continue;
            }
            // char not in [`R`, `AL`, `AN`, `EN`, `ES`, `CS`, `ET`, `ON`, `BN`, or `NSM`]
            _ => return false,
        }

        if nsm {
            // rule 3
            // If we got a character with `Bidi` property `NSM`,
            // only characters with `Bidi` property `NSM` are allowed
            return false;
        } else {
            prev = class;
        }
    }

    nsm || matches!(
        prev,
        BidiClass::R | BidiClass::AL | BidiClass::EN | BidiClass::AN
    )
}

fn is_valid_ltr_label<I>(it: I, prev: BidiClass) -> bool
where
    I: IntoIterator<Item = char>,
{
    let mut prev = prev;
    let mut nsm = false;

    for c in it {
        let class = bidi_class(c);
        // rule 5
        // In an `LTR` label, only characters with the `Bidi` properties `L`, `EN`,
        // `ES`, `CS`, `ET`, `ON`, `BN`, or `NSM` are allowed.
        match class {
            BidiClass::L
            | BidiClass::EN
            | BidiClass::ES
            | BidiClass::CS
            | BidiClass::ET
            | BidiClass::ON
            | BidiClass::BN => {
                if nsm {
                    // rule 6
                    // If we got a character with `Bidi` property `NSM`,
                    // only characters with `Bidi` property `NSM` are allowed
                    return false;
                }
                prev = class;
            }
            BidiClass::NSM => {
                // rule 6
                // In an `LTR` label, the end of the label must be a character with
                // `Bidi` property `L` or `EN`, followed by zero or more characters with
                // `Bidi` property `NSM`.
                if !matches!(prev, BidiClass::L | BidiClass::EN) {
                    // char not in L or EN
                    return false;
                }
                nsm = true;
            }
            // char not in [`L`, `EN`, `ES`, `CS`, `ET`, `ON`, `BN`, or `NSM`]
            _ => return false,
        };
    }

    nsm || matches!(prev, BidiClass::L | BidiClass::EN)
}

#[cfg(test)]
mod bidi_tests {
    use crate::bidi::*;

    const L: char = '\u{00aa}';
    const R: char = '\u{05be}';
    const AL: char = '\u{0608}';
    const AN: char = '\u{06dd}';
    const EN: char = '\u{00b9}';
    const ES: char = '\u{002b}';
    const CS: char = '\u{002c}';
    const ET: char = '\u{058f}';
    const ON: char = '\u{037e}';
    const BN: char = '\u{00ad}';
    const NSM: char = '\u{1e2ae}';
    const WS: char = '\u{0020}';

    macro_rules! str_chars {
    ($($args:expr),*) => {{
		let mut result = String::from("");
		$(
			result.push($args);
		)*
		result
		}}
	}

    #[test]
    fn test_bidi_class() {
        assert_eq!(bidi_class(L), BidiClass::L);
        assert_eq!(bidi_class(R), BidiClass::R);
        assert_eq!(bidi_class(AL), BidiClass::AL);
        assert_eq!(bidi_class(AN), BidiClass::AN);
        assert_eq!(bidi_class(EN), BidiClass::EN);
        assert_eq!(bidi_class(ES), BidiClass::ES);
        assert_eq!(bidi_class(CS), BidiClass::CS);
        assert_eq!(bidi_class(ET), BidiClass::ET);
        assert_eq!(bidi_class(ON), BidiClass::ON);
        assert_eq!(bidi_class(BN), BidiClass::BN);
        assert_eq!(bidi_class(NSM), BidiClass::NSM);
        assert_eq!(bidi_class(WS), BidiClass::WS);

        // All code points not explicitly listed `Bidi_Class`
        // have the value `Left_To_Right` (L).
        assert_eq!(bidi_class('\u{e0080}'), BidiClass::L);
    }

    #[test]
    fn test_has_rtl() {
        assert!(!has_rtl(""));
        assert!(!has_rtl("Hi"));

        // check R character
        assert!(has_rtl(&str_chars!(R)));
        assert!(has_rtl(&str_chars!(R, 'A')));
        assert!(has_rtl(&str_chars!('A', R)));

        // check AL character
        assert!(has_rtl(&str_chars!(AL)));
        assert!(has_rtl(&str_chars!(AL, 'A')));
        assert!(has_rtl(&str_chars!('A', AL)));

        // check AN character
        assert!(has_rtl(&str_chars!(AN)));
        assert!(has_rtl(&str_chars!(AN, 'A')));
        assert!(has_rtl(&str_chars!('A', AN)));
    }

    #[test]
    fn test_bidi_rule() {
        // Check empty label
        assert!(satisfy_bidi_rule(""));

        // Check rule 1
        // First character is L
        assert!(satisfy_bidi_rule(&str_chars!(L)));

        // First character is R
        assert!(satisfy_bidi_rule(&str_chars!(R)));
        // First character is AL
        assert!(satisfy_bidi_rule(&str_chars!(AL)));

        // First character is ES (not [`L`, `R` or `AL`])
        assert!(!satisfy_bidi_rule(&str_chars!(ES)));
        // First character is `WS`
        assert!(!satisfy_bidi_rule(&str_chars!(WS)));
    }

    #[test]
    fn test_rtl_label() {
        // Check rule 2
        // In an `RTL` label, only characters with the `Bidi` properties `R`, `AL`,
        // `AN`, `EN`, `ES`, `CS`, `ET`, `ON`, `BN`, or `NSM` are allowed:
        assert!(satisfy_bidi_rule(&str_chars!(
            R, AL, ES, CS, ET, ON, BN, AN
        )));
        assert!(satisfy_bidi_rule(&str_chars!(
            R, AL, ES, CS, ET, ON, BN, EN
        )));
        assert!(satisfy_bidi_rule(&str_chars!(
            R, AL, ES, CS, ET, ON, BN, EN, NSM
        )));
        // Add a character with `Bidi` property `WS` which is not in
        // [`R`, `AL`, `AN`, `EN`, `ES`, `CS`, `ET`, `ON`, `BN`, or `NSM`]
        assert!(!satisfy_bidi_rule(&str_chars!(
            R, AL, ES, CS, WS, ON, BN, EN, NSM
        )));

        // Check rule 3
        // In an `RTL` label, the end of the label must be a character with
        // `Bidi` property `R`, `AL`, `EN`, or `AN`, followed by zero or more
        // characters with `Bidi` property `NSM`
        assert!(satisfy_bidi_rule(&str_chars!(R, AL, EN, NSM, NSM)));
        assert!(satisfy_bidi_rule(&str_chars!(R, NSM, NSM, NSM, NSM)));
        // Next tests check that last character is not in [`R`, `AL`, `EN`, or `AN`]
        assert!(!satisfy_bidi_rule(&str_chars!(R, CS)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, ET, NSM)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, BN, NSM, NSM)));

        // After a character with `Bidi` property `NSM`, only character with the
        // same `Bidi` property are allowed
        assert!(!satisfy_bidi_rule(&str_chars!(R, NSM, AN)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, BN, NSM, NSM, AN)));

        // Check rule 4
        // In an `RTL` label, if an `EN` is present, no `AN` may be present, and
        // vice versa.
        assert!(!satisfy_bidi_rule(&str_chars!(R, EN, CS, AN, AL, NSM)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, AN, CS, EN, AL, NSM)));
        // Two characters `AN` are allowed
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, AN, AL)));
        // Two characters `EN` are allowed
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, EN, AL)));
    }

    #[test]
    fn test_ltr_label() {
        // Check rule 5
        // In an `LTR` label, only characters with the `Bidi` properties `L`, `EN`,
        // `ES`, `CS`, `ET`, `ON`, `BN`, or `NSM` are allowed.
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, ES, CS, ET, ON, BN, L)));
        // `LTR` label with a character with `Bidi` property `R` which is
        // not in [`L`, `EN`, `ES`, `CS`, `ET`, `ON`, `BN`, `NSM`] must fail
        assert!(!satisfy_bidi_rule(&str_chars!(L, EN, ES, CS, R, ON, BN, L)));

        // Check rule 6
        // In an `LTR` label, the end of the label must be a character with
        // `Bidi` property `L` or `EN`, followed by zero or more characters with
        // `Bidi` property `NSM`.
        assert!(satisfy_bidi_rule(&str_chars!(L)));
        assert!(satisfy_bidi_rule(&str_chars!(L, EN)));
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, NSM)));
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, NSM, NSM)));
        assert!(satisfy_bidi_rule(&str_chars!(L, NSM)));

        // `LTR` label that not ends with a character with `Bidi` property that
        // is not `L` or `EN` must fail
        assert!(!satisfy_bidi_rule(&str_chars!(L, ES)));
        assert!(!satisfy_bidi_rule(&str_chars!(L, CS, NSM)));

        // After a character with `Bidi` property `NSM` is found, only
        // characters with `Bidi` property `NSM` are allowed
        assert!(!satisfy_bidi_rule(&str_chars!(L, NSM, EN)));
        assert!(!satisfy_bidi_rule(&str_chars!(L, NSM, NSM, L, EN, NSM)));
    }

    #[test]
    fn test_rtl_edge_cases() {
        // Single R character
        assert!(satisfy_bidi_rule(&str_chars!(R)));

        // Single AL character
        assert!(satisfy_bidi_rule(&str_chars!(AL)));

        // R followed by AL
        assert!(satisfy_bidi_rule(&str_chars!(R, AL)));

        // AL followed by R
        assert!(satisfy_bidi_rule(&str_chars!(AL, R)));

        // Long NSM sequence at end
        assert!(satisfy_bidi_rule(&str_chars!(
            R, AL, NSM, NSM, NSM, NSM, NSM
        )));

        // NSM without proper preceding character
        assert!(!satisfy_bidi_rule(&str_chars!(R, ES, NSM)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, CS, NSM)));

        // BN (Boundary Neutral) characters
        assert!(satisfy_bidi_rule(&str_chars!(R, BN, AL)));
        assert!(satisfy_bidi_rule(&str_chars!(R, BN, BN, AL)));

        // CS/ES/ET (Common Separators and European Terminators)
        assert!(satisfy_bidi_rule(&str_chars!(R, ES, AL)));
        assert!(satisfy_bidi_rule(&str_chars!(R, CS, AL)));
        assert!(satisfy_bidi_rule(&str_chars!(R, ET, AL)));

        // ON (Other Neutrals)
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, AL)));
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, ON, AL)));
    }

    #[test]
    fn test_ltr_edge_cases() {
        // Single L character
        assert!(satisfy_bidi_rule(&str_chars!(L)));

        // Single L with NSM
        assert!(satisfy_bidi_rule(&str_chars!(L, NSM)));

        // Long NSM sequence at end
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, NSM, NSM, NSM, NSM)));

        // BN (Boundary Neutral) in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, BN, EN)));
        assert!(satisfy_bidi_rule(&str_chars!(L, BN, BN, L)));

        // ES in LTR ending with L
        assert!(satisfy_bidi_rule(&str_chars!(L, ES, L)));
        assert!(satisfy_bidi_rule(&str_chars!(L, ES, EN)));

        // NSM not after L or EN should fail
        assert!(!satisfy_bidi_rule(&str_chars!(L, ES, NSM)));
        assert!(!satisfy_bidi_rule(&str_chars!(L, BN, NSM)));
    }

    #[test]
    fn test_mixed_numeric_edge_cases() {
        // AN without EN in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, AN, R)));

        // EN without AN in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, EN, R)));

        // Multiple AN at different positions
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, ES, AN, AL)));

        // Multiple EN at different positions
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, ES, EN, AL)));

        // EN in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, EN, EN)));
        assert!(satisfy_bidi_rule(&str_chars!(L, ES, EN)));
    }

    #[test]
    fn test_rtl_all_allowed_classes() {
        // Test each allowed class individually in RTL context
        // This ensures coverage of all match arms in is_valid_rtl_label

        // R class
        assert!(satisfy_bidi_rule(&str_chars!(R, R, R)));

        // AL class
        assert!(satisfy_bidi_rule(&str_chars!(AL, AL, AL)));

        // ES (European Separator) in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, ES, R)));

        // CS (Common Separator) in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, CS, R)));

        // ET (European Terminator) in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, ET, R)));

        // ON (Other Neutral) in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, R)));

        // BN (Boundary Neutral) in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, BN, R)));

        // AN (Arabic Number) in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, AN, R)));

        // EN (European Number) in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, EN, R)));

        // NSM (Non-Spacing Mark) in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, NSM)));
        assert!(satisfy_bidi_rule(&str_chars!(AL, NSM)));
    }

    #[test]
    fn test_ltr_all_allowed_classes() {
        // Test each allowed class individually in LTR context
        // This ensures coverage of all match arms in is_valid_ltr_label

        // L class
        assert!(satisfy_bidi_rule(&str_chars!(L, L, L)));

        // EN (European Number) in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, L)));

        // ES (European Separator) in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, ES, L)));

        // CS (Common Separator) in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, CS, L)));

        // ET (European Terminator) in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, ET, L)));

        // ON (Other Neutral) in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, ON, L)));

        // BN (Boundary Neutral) in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, BN, L)));

        // NSM (Non-Spacing Mark) in LTR after L
        assert!(satisfy_bidi_rule(&str_chars!(L, NSM)));

        // NSM after EN in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, NSM)));
    }

    #[test]
    fn test_rtl_ending_validation() {
        // Test the final validation in is_valid_rtl_label (line 147)
        // Must end with R, AL, EN, or AN (possibly followed by NSM)

        // Valid endings
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, R))); // ends with R
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, AL))); // ends with AL
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, EN))); // ends with EN
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, AN))); // ends with AN

        // With NSM
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, R, NSM)));
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, AL, NSM)));
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, EN, NSM)));
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, AN, NSM)));

        // Invalid endings (should fail)
        assert!(!satisfy_bidi_rule(&str_chars!(R, ON, ES)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, ON, CS)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, ON, ET)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, ON, BN)));
    }

    #[test]
    fn test_invalid_first_char() {
        // Characters that are not L, R, or AL at the start
        assert!(!satisfy_bidi_rule(&str_chars!(EN)));
        assert!(!satisfy_bidi_rule(&str_chars!(AN)));
        assert!(!satisfy_bidi_rule(&str_chars!(NSM)));
        assert!(!satisfy_bidi_rule(&str_chars!(BN)));
        assert!(!satisfy_bidi_rule(&str_chars!(ON)));
    }

    #[test]
    fn test_bidi_class_cp() {
        // Test codepoint-based classification
        assert_eq!(bidi_class_cp(L as u32), BidiClass::L);
        assert_eq!(bidi_class_cp(R as u32), BidiClass::R);
        assert_eq!(bidi_class_cp(AL as u32), BidiClass::AL);

        // Test default case (unassigned codepoints default to L)
        assert_eq!(bidi_class_cp(0xE0080), BidiClass::L);

        // Test all major BiDi classes
        assert_eq!(bidi_class_cp(AN as u32), BidiClass::AN);
        assert_eq!(bidi_class_cp(EN as u32), BidiClass::EN);
        assert_eq!(bidi_class_cp(ES as u32), BidiClass::ES);
        assert_eq!(bidi_class_cp(CS as u32), BidiClass::CS);
        assert_eq!(bidi_class_cp(ET as u32), BidiClass::ET);
        assert_eq!(bidi_class_cp(ON as u32), BidiClass::ON);
        assert_eq!(bidi_class_cp(BN as u32), BidiClass::BN);
        assert_eq!(bidi_class_cp(NSM as u32), BidiClass::NSM);
        assert_eq!(bidi_class_cp(WS as u32), BidiClass::WS);
    }

    #[test]
    fn test_bidi_class_char() {
        // Test char-based classification (wraps bidi_class_cp)
        assert_eq!(bidi_class(L), BidiClass::L);
        assert_eq!(bidi_class(R), BidiClass::R);
        assert_eq!(bidi_class(AL), BidiClass::AL);
    }

    #[test]
    fn test_has_rtl_comprehensive() {
        // Empty string
        assert!(!has_rtl(""));

        // Only LTR characters
        assert!(!has_rtl("Hello World"));
        assert!(!has_rtl("123"));

        // Contains R
        assert!(has_rtl(&str_chars!(L, R, L)));
        assert!(has_rtl(&str_chars!(R)));

        // Contains AL
        assert!(has_rtl(&str_chars!(L, AL, L)));
        assert!(has_rtl(&str_chars!(AL)));

        // Contains AN
        assert!(has_rtl(&str_chars!(L, AN, L)));
        assert!(has_rtl(&str_chars!(AN)));

        // Mixed RTL markers
        assert!(has_rtl(&str_chars!(R, AL, AN)));
    }

    #[test]
    fn test_rtl_label_nsm_edge_cases() {
        // NSM at the very end after valid ending
        assert!(satisfy_bidi_rule(&str_chars!(R, AL, NSM, NSM, NSM)));

        // Multiple NSM sequences
        assert!(satisfy_bidi_rule(&str_chars!(R, NSM, NSM)));

        // NSM after AN (valid ending)
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, NSM)));

        // NSM after EN (valid ending)
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, NSM)));
    }

    #[test]
    fn test_ltr_label_comprehensive() {
        // L with various neutrals
        assert!(satisfy_bidi_rule(&str_chars!(L, ES, ET, CS, ON, BN, L)));

        // Multiple EN in sequence
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, EN, EN, EN)));

        // L with ending EN
        assert!(satisfy_bidi_rule(&str_chars!(L, ES, CS, EN)));

        // Just L
        assert!(satisfy_bidi_rule(&str_chars!(L)));

        // L followed by EN
        assert!(satisfy_bidi_rule(&str_chars!(L, EN)));
    }

    #[test]
    fn test_bidi_rule_boundary_conditions() {
        // String with only neutrals starting with valid L
        assert!(satisfy_bidi_rule(&str_chars!(L, BN, BN, BN, L)));

        // String with alternating valid patterns
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, ES, EN, CS, EN)));

        // RTL with maximum allowed neutral characters
        assert!(satisfy_bidi_rule(&str_chars!(
            R, BN, ES, CS, ET, ON, BN, AL
        )));
    }

    #[test]
    fn test_bidi_invalid_endings() {
        // LTR ending with invalid character
        assert!(!satisfy_bidi_rule(&str_chars!(L, ES, CS)));
        assert!(!satisfy_bidi_rule(&str_chars!(L, BN)));
        assert!(!satisfy_bidi_rule(&str_chars!(L, ON)));

        // RTL ending with invalid character
        assert!(!satisfy_bidi_rule(&str_chars!(R, BN)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, ES)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, ON)));
    }

    #[test]
    fn test_rtl_all_neutral_types() {
        // Test each neutral type explicitly in RTL context
        // This ensures all match arms in is_valid_rtl_label are covered

        // R type
        assert!(satisfy_bidi_rule(&str_chars!(R, R, R)));

        // AL type
        assert!(satisfy_bidi_rule(&str_chars!(R, AL, AL, R)));

        // ES type (European Separator)
        assert!(satisfy_bidi_rule(&str_chars!(R, ES, R)));

        // CS type (Common Separator)
        assert!(satisfy_bidi_rule(&str_chars!(R, CS, R)));

        // ET type (European Terminator)
        assert!(satisfy_bidi_rule(&str_chars!(R, ET, R)));

        // ON type (Other Neutral)
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, R)));

        // BN type (Boundary Neutral)
        assert!(satisfy_bidi_rule(&str_chars!(R, BN, R)));
    }

    #[test]
    fn test_ltr_all_neutral_types() {
        // Test each neutral type explicitly in LTR context

        // Multiple L
        assert!(satisfy_bidi_rule(&str_chars!(L, L, L)));

        // EN type
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, EN, L)));

        // ES type in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, ES, L)));

        // CS type in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, CS, L)));

        // ET type in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, ET, L)));

        // ON type in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, ON, L)));

        // BN type in LTR
        assert!(satisfy_bidi_rule(&str_chars!(L, BN, L)));
    }

    #[test]
    fn test_nsm_after_invalid_base() {
        // NSM after ES in RTL (ES is not R/AL/EN/AN)
        assert!(!satisfy_bidi_rule(&str_chars!(R, ES, NSM)));

        // NSM after CS in RTL
        assert!(!satisfy_bidi_rule(&str_chars!(R, CS, NSM)));

        // NSM after ET in RTL
        assert!(!satisfy_bidi_rule(&str_chars!(R, ET, NSM)));

        // NSM after ON in RTL
        assert!(!satisfy_bidi_rule(&str_chars!(R, ON, NSM)));

        // NSM after BN in RTL
        assert!(!satisfy_bidi_rule(&str_chars!(R, BN, NSM)));
    }

    #[test]
    fn test_nsm_after_invalid_base_ltr() {
        // NSM after ES in LTR (ES is not L/EN)
        assert!(!satisfy_bidi_rule(&str_chars!(L, ES, NSM)));

        // NSM after CS in LTR
        assert!(!satisfy_bidi_rule(&str_chars!(L, CS, NSM)));

        // NSM after ET in LTR
        assert!(!satisfy_bidi_rule(&str_chars!(L, ET, NSM)));

        // NSM after ON in LTR
        assert!(!satisfy_bidi_rule(&str_chars!(L, ON, NSM)));

        // NSM after BN in LTR
        assert!(!satisfy_bidi_rule(&str_chars!(L, BN, NSM)));
    }

    #[test]
    fn test_character_after_nsm_sequence() {
        // In RTL: after NSM sequence starts, only NSM allowed
        assert!(!satisfy_bidi_rule(&str_chars!(R, AL, NSM, R)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, EN, NSM, EN)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, AN, NSM, AN)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, AL, NSM, AL)));

        // In LTR: after NSM sequence starts, only NSM allowed
        assert!(!satisfy_bidi_rule(&str_chars!(L, NSM, L)));
        assert!(!satisfy_bidi_rule(&str_chars!(L, EN, NSM, EN)));
        assert!(!satisfy_bidi_rule(&str_chars!(L, EN, NSM, L)));
    }

    #[test]
    fn test_rtl_each_character_type_individually() {
        // Test that each character type R, AL, ES, CS, ET, ON, BN
        // is individually accepted in RTL labels (lines 93-99)

        // R in middle position
        assert!(satisfy_bidi_rule(&str_chars!(R, R, R)));

        // AL in middle position
        assert!(satisfy_bidi_rule(&str_chars!(R, AL, R)));

        // ES in middle position
        assert!(satisfy_bidi_rule(&str_chars!(R, ES, R)));

        // CS in middle position
        assert!(satisfy_bidi_rule(&str_chars!(R, CS, R)));

        // ET in middle position
        assert!(satisfy_bidi_rule(&str_chars!(R, ET, R)));

        // ON in middle position
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, R)));

        // BN in middle position
        assert!(satisfy_bidi_rule(&str_chars!(R, BN, R)));
    }

    #[test]
    fn test_ltr_each_character_type_individually() {
        // Test that each character type L, EN, ES, CS, ET, ON, BN
        // is individually accepted in LTR labels (lines 164-170)

        // L in middle
        assert!(satisfy_bidi_rule(&str_chars!(L, L, L)));

        // EN in middle
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, L)));

        // ES in middle
        assert!(satisfy_bidi_rule(&str_chars!(L, ES, L)));

        // CS in middle
        assert!(satisfy_bidi_rule(&str_chars!(L, CS, L)));

        // ET in middle
        assert!(satisfy_bidi_rule(&str_chars!(L, ET, L)));

        // ON in middle
        assert!(satisfy_bidi_rule(&str_chars!(L, ON, L)));

        // BN in middle
        assert!(satisfy_bidi_rule(&str_chars!(L, BN, L)));
    }

    #[test]
    fn test_coverage_rtl_individual_branches() {
        // Ensure each individual branch in the RTL match (lines 93-99) is executed

        // Single R after R start (line 93)
        assert!(satisfy_bidi_rule(&str_chars!(R, R)));

        // Single AL after AL start (line 94)
        assert!(satisfy_bidi_rule(&str_chars!(AL, AL)));

        // ES in second position (line 95)
        assert!(satisfy_bidi_rule(&str_chars!(R, ES, R)));

        // CS in second position (line 96)
        assert!(satisfy_bidi_rule(&str_chars!(R, CS, R)));

        // ET in second position (line 97)
        assert!(satisfy_bidi_rule(&str_chars!(R, ET, R)));

        // ON in second position (line 98)
        assert!(satisfy_bidi_rule(&str_chars!(R, ON, R)));

        // BN in second position (line 99)
        assert!(satisfy_bidi_rule(&str_chars!(R, BN, R)));
    }

    #[test]
    fn test_coverage_an_branch() {
        // Cover line 100: BidiClass::AN branch entry
        // AN in RTL label (line 100)
        assert!(satisfy_bidi_rule(&str_chars!(R, AN)));

        // Multiple AN allowed
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, AN)));
    }

    #[test]
    fn test_coverage_en_branch() {
        // Cover line 108: BidiClass::EN branch entry
        // EN in RTL label (line 108)
        assert!(satisfy_bidi_rule(&str_chars!(R, EN)));

        // Multiple EN allowed
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, EN)));
    }

    #[test]
    fn test_coverage_nsm_branch() {
        // Cover line 116: BidiClass::NSM branch entry
        // NSM after valid R (line 116)
        assert!(satisfy_bidi_rule(&str_chars!(R, NSM)));

        // NSM after AL
        assert!(satisfy_bidi_rule(&str_chars!(AL, NSM)));

        // NSM after EN in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, NSM)));

        // NSM after AN in RTL
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, NSM)));
    }

    #[test]
    fn test_coverage_nsm_validation() {
        // Cover line 123: Check if NSM follows valid character
        // Invalid: NSM after ES (line 121-126)
        assert!(!satisfy_bidi_rule(&str_chars!(R, ES, NSM)));

        // Invalid: NSM after CS
        assert!(!satisfy_bidi_rule(&str_chars!(R, CS, NSM)));

        // Invalid: NSM after BN
        assert!(!satisfy_bidi_rule(&str_chars!(R, BN, NSM)));
    }

    #[test]
    fn test_coverage_rtl_ending() {
        // Cover line 147: Final validation in is_valid_rtl_label
        // Ending with R
        assert!(satisfy_bidi_rule(&str_chars!(R)));

        // Ending with AL
        assert!(satisfy_bidi_rule(&str_chars!(AL)));

        // Ending with EN
        assert!(satisfy_bidi_rule(&str_chars!(R, EN)));

        // Ending with AN
        assert!(satisfy_bidi_rule(&str_chars!(R, AN)));

        // Ending with NSM after valid char (line 145 covers nsm flag)
        assert!(satisfy_bidi_rule(&str_chars!(R, NSM)));
    }

    #[test]
    fn test_coverage_ltr_individual_branches() {
        // Cover lines 164-170: Each branch in LTR match

        // L in second position (line 164)
        assert!(satisfy_bidi_rule(&str_chars!(L, L)));

        // EN in second position (line 165)
        assert!(satisfy_bidi_rule(&str_chars!(L, EN)));

        // ES in second position (line 166)
        assert!(satisfy_bidi_rule(&str_chars!(L, ES, L)));

        // CS in second position (line 167)
        assert!(satisfy_bidi_rule(&str_chars!(L, CS, L)));

        // ET in second position (line 168)
        assert!(satisfy_bidi_rule(&str_chars!(L, ET, L)));

        // ON in second position (line 169)
        assert!(satisfy_bidi_rule(&str_chars!(L, ON, L)));

        // BN in second position (line 170)
        assert!(satisfy_bidi_rule(&str_chars!(L, BN, L)));
    }

    #[test]
    fn test_coverage_ltr_nsm_branch() {
        // Cover line 179: NSM branch in LTR
        // NSM after L (line 179)
        assert!(satisfy_bidi_rule(&str_chars!(L, NSM)));

        // NSM after EN in LTR (line 179)
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, NSM)));
    }

    #[test]
    fn test_an_en_conflict_in_rtl() {
        // Test line 100-107: AN when EN is present
        // AN should be accepted when no EN
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, R)));

        // But fail when EN already present (line 101-104)
        assert!(!satisfy_bidi_rule(&str_chars!(R, EN, AN)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, EN, ES, AN)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, EN, CS, AN, R)));
    }

    #[test]
    fn test_en_an_conflict_in_rtl() {
        // Test line 108-115: EN when AN is present
        // EN should be accepted when no AN
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, R)));

        // But fail when AN already present (line 109-112)
        assert!(!satisfy_bidi_rule(&str_chars!(R, AN, EN)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, AN, ES, EN)));
        assert!(!satisfy_bidi_rule(&str_chars!(R, AN, CS, EN, R)));
    }

    #[test]
    fn test_nsm_placement_rules() {
        // Test line 116-129: NSM must follow R/AL/EN/AN

        // NSM after R (valid)
        assert!(satisfy_bidi_rule(&str_chars!(R, NSM)));

        // NSM after AL (valid)
        assert!(satisfy_bidi_rule(&str_chars!(AL, NSM)));

        // NSM after EN (valid)
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, NSM)));

        // NSM after AN (valid)
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, NSM)));

        // NSM after ES (invalid - line 121-126)
        assert!(!satisfy_bidi_rule(&str_chars!(R, ES, NSM)));

        // NSM after CS (invalid)
        assert!(!satisfy_bidi_rule(&str_chars!(R, CS, NSM)));

        // NSM after ET (invalid)
        assert!(!satisfy_bidi_rule(&str_chars!(R, ET, NSM)));

        // NSM after ON (invalid)
        assert!(!satisfy_bidi_rule(&str_chars!(R, ON, NSM)));
    }

    #[test]
    fn test_ltr_nsm_placement() {
        // Test line 179-189: NSM must follow L or EN in LTR

        // NSM after L (valid)
        assert!(satisfy_bidi_rule(&str_chars!(L, NSM)));

        // NSM after EN (valid)
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, NSM)));

        // NSM after ES (invalid - line 184-186)
        assert!(!satisfy_bidi_rule(&str_chars!(L, ES, NSM)));

        // NSM after CS (invalid)
        assert!(!satisfy_bidi_rule(&str_chars!(L, CS, NSM)));

        // NSM after ET (invalid)
        assert!(!satisfy_bidi_rule(&str_chars!(L, ET, NSM)));

        // NSM after ON (invalid)
        assert!(!satisfy_bidi_rule(&str_chars!(L, ON, NSM)));

        // NSM after BN (invalid)
        assert!(!satisfy_bidi_rule(&str_chars!(L, BN, NSM)));
    }

    #[test]
    fn test_ending_with_nsm() {
        // RTL must end with R/AL/EN/AN (line 145-148)
        assert!(satisfy_bidi_rule(&str_chars!(R, NSM))); // ends with R via NSM
        assert!(satisfy_bidi_rule(&str_chars!(AL, NSM))); // ends with AL via NSM
        assert!(satisfy_bidi_rule(&str_chars!(R, EN, NSM))); // ends with EN via NSM
        assert!(satisfy_bidi_rule(&str_chars!(R, AN, NSM))); // ends with AN via NSM

        // LTR must end with L or EN (line 195)
        assert!(satisfy_bidi_rule(&str_chars!(L, NSM))); // ends with L via NSM
        assert!(satisfy_bidi_rule(&str_chars!(L, EN, NSM))); // ends with EN via NSM
    }
}
