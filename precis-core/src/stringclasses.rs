use crate::common;
use crate::context;
use crate::DerivedPropertyValue;

/// Interface for specific classes to deal with specific Unicode
/// code groups defined in RFC 8264.
/// Next callbacks will be invoked to calculate the derived property
/// value in the order defined in [`get_derived_property_value`]
pub trait SpecificDerivedPropertyValue {
    /// Callback invoked when the Unicode code point belongs to
    /// [Spaces](https://datatracker.ietf.org/doc/html/rfc8264#section-9.14)
    fn on_spaces(&self) -> DerivedPropertyValue;
    /// Callback invoked when the Unicode code point belongs to
    /// [Symbols](https://datatracker.ietf.org/doc/html/rfc8264#section-9.15)
    fn on_symbols(&self) -> DerivedPropertyValue;
    /// Callback invoked when the Unicode code point belongs to
    /// [Punctuation](https://datatracker.ietf.org/doc/html/rfc8264#section-9.16)
    fn on_punctuation(&self) -> DerivedPropertyValue;
    /// Callback invoked when the Unicode code point belongs to
    /// [`HasCompat`](https://datatracker.ietf.org/doc/html/rfc8264#section-9.17)
    fn on_has_compat(&self) -> DerivedPropertyValue;
    /// Callback invoked when the Unicode code point belongs to
    /// [`OtherLetterDigits`](https://datatracker.ietf.org/doc/html/rfc8264#section-9.18)
    fn on_other_letter_digits(&self) -> DerivedPropertyValue;
}

/// Implements the algorithm to calculate the value of the derived property.
/// This algorithm is as follows (implementations MUST NOT modify the order
/// of operations within this algorithm, because doing so would cause
/// inconsistent results across implementations):
///
/// > If .`cp`. .in. `Exceptions` Then `Exceptions`(`cp`);\
/// > Else If .`cp`. .in. `BackwardCompatible` Then `BackwardCompatible`(`cp`);\
/// > Else If .`cp`. .in. `Unassigned` Then `UNASSIGNED`;\
/// > Else If .`cp`. .in. `ASCII7` Then `PVALID`;\
/// > Else If .`cp`. .in. `JoinControl` Then `CONTEXTJ`;\
/// > Else If .`cp`. .in. `OldHangulJamo` Then `DISALLOWED`;\
/// > Else If .`cp`. .in. `PrecisIgnorableProperties` Then `DISALLOWED`;\
/// > Else If .`cp`. .in. `Controls` Then `DISALLOWED`;\
/// > Else If .`cp`. .in. `HasCompat` Then `ID_DIS` or `FREE_PVAL`;\
/// > Else If .`cp`. .in. `LetterDigits` Then `PVALID`;\
/// > Else If .`cp`. .in. `OtherLetterDigits` Then `ID_DIS` or `FREE_PVAL`;\
/// > Else If .`cp`. .in. `Spaces` Then `ID_DIS` or `FREE_PVAL`;\
/// > Else If .`cp`. .in. `Symbols` Then `ID_DIS` or `FREE_PVAL`;\
/// > Else If .`cp`. .in. `Punctuation` Then `ID_DIS` or `FREE_PVAL`;\
/// > Else DISALLOWED;\
///
/// # Arguments
/// * `cp` - Unicode code point
/// * `obj` - Object implementing the [`SpecificDerivedPropertyValue`] trait.
///
/// # Return
/// This function returns the derived property value as defined in
/// [RFC 8264](https://datatracker.ietf.org/doc/html/rfc8264#section-8)
#[allow(clippy::if_same_then_else)]
pub fn get_derived_property_value(
    cp: u32,
    obj: &dyn SpecificDerivedPropertyValue,
) -> DerivedPropertyValue {
    match common::get_exception_val(cp) {
        Some(val) => *val,
        None => match common::get_backward_compatible_val(cp) {
            Some(val) => *val,
            None => {
                if common::is_unassigned(cp) {
                    DerivedPropertyValue::Unassigned
                } else if common::is_ascii7(cp) {
                    DerivedPropertyValue::PValid
                } else if common::is_join_control(cp) {
                    DerivedPropertyValue::ContextJ
                } else if common::is_old_hangul_jamo(cp) {
                    DerivedPropertyValue::Disallowed
                } else if common::is_precis_ignorable_property(cp) {
                    DerivedPropertyValue::Disallowed
                } else if common::is_control(cp) {
                    DerivedPropertyValue::Disallowed
                } else if common::has_compat(cp) {
                    obj.on_has_compat()
                } else if common::is_letter_digit(cp) {
                    DerivedPropertyValue::PValid
                } else if common::is_other_letter_digit(cp) {
                    obj.on_other_letter_digits()
                } else if common::is_space(cp) {
                    obj.on_spaces()
                } else if common::is_symbol(cp) {
                    obj.on_symbols()
                } else if common::is_punctuation(cp) {
                    obj.on_punctuation()
                } else {
                    DerivedPropertyValue::Disallowed
                }
            }
        },
    }
}

fn allowed_by_context_rule(s: &str, cp: u32, offset: usize) -> bool {
    let val = context::get_context_rule(cp);

    debug_assert!(
        val.is_some(),
        "No context rule found for Unicode code point: {:#04x}",
        cp
    );

    let mut allowed = false;
    if let Some(rule) = val {
        if let Ok(ret) = rule(s, offset) {
            allowed = ret;
        };
    }

    allowed
}

/// Base interface for all String classes in PRECIS framework.
pub trait StringClass {
    /// Gets the derived property value according to the algorithm defined
    /// in [`get_derived_property_value`]
    /// # Arguments
    /// * `c`- Unicode character
    /// # Return
    /// This method returns the derived property value associated to a Unicode character
    fn get_value_from_char(&self, c: char) -> DerivedPropertyValue;

    /// Gets the derived property value according to the algorithm defined
    /// in [`get_derived_property_value`]
    /// # Arguments:
    /// * `cp`- Unicode code point
    /// # Return
    /// This method returns the derived property value associated to a Unicode character
    fn get_value_from_codepoint(&self, cp: u32) -> DerivedPropertyValue;

    /// Ensures that the string consists only of Unicode code points that
    /// are explicitly allowed by the PRECIS
    /// [String Class](https://datatracker.ietf.org/doc/html/rfc8264#section-4)
    /// # Arguments:
    /// * `label` - string to check
    /// # Returns
    /// true if all character of `label` are allowed by the String Class.
    fn allows(&self, label: &str) -> bool {
        for (offset, c) in label.chars().enumerate() {
            let val = self.get_value_from_char(c);

            match val {
                DerivedPropertyValue::PValid | DerivedPropertyValue::SpecClassPval => {}
                DerivedPropertyValue::SpecClassDis
                | DerivedPropertyValue::Disallowed
                | DerivedPropertyValue::Unassigned => return false,
                DerivedPropertyValue::ContextJ | DerivedPropertyValue::ContextO => {
                    if !allowed_by_context_rule(label, c as u32, offset) {
                        return false;
                    }
                }
            }
        }

        true
    }
}

/// Concrete class representing PRECIS `IdentifierClass` from
/// [RFC 8264](https://datatracker.ietf.org/doc/html/rfc8264#section-4.2).
/// # Example
/// ```rust
/// use precis_core::{DerivedPropertyValue,IdentifierClass,StringClass};
///
/// let id = IdentifierClass {};
/// // character 𐍁 is OtherLetterDigits (R)
/// assert_eq!(id.get_value_from_char('𐍁'), DerivedPropertyValue::SpecClassDis);
/// // Character S is ASCII7 (K)
/// assert_eq!(id.get_value_from_char('S'), DerivedPropertyValue::PValid);
/// // Character 0x1170 is OldHangulJamo (I)
/// assert_eq!(id.get_value_from_codepoint(0x1170), DerivedPropertyValue::Disallowed);
/// ```
pub struct IdentifierClass {}

impl SpecificDerivedPropertyValue for IdentifierClass {
    // `ID_DIS` mapped to `SPEC_CLASS_DIS`
    fn on_has_compat(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassDis
    }
    fn on_other_letter_digits(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassDis
    }
    fn on_spaces(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassDis
    }
    fn on_symbols(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassDis
    }
    fn on_punctuation(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassDis
    }
}

impl StringClass for IdentifierClass {
    fn get_value_from_char(&self, c: char) -> DerivedPropertyValue {
        get_derived_property_value(c as u32, self)
    }

    fn get_value_from_codepoint(&self, cp: u32) -> DerivedPropertyValue {
        get_derived_property_value(cp, self)
    }
}

/// Concrete class representing PRECIS `FreeformClass` from
/// [RFC 8264](https://datatracker.ietf.org/doc/html/rfc8264#section-4.3).
/// # Example
/// ```rust
/// use precis_core::{DerivedPropertyValue,FreeformClass,StringClass};
///
/// let ff = FreeformClass {};
/// // character 𐍁 is OtherLetterDigits (R)
/// assert_eq!(ff.get_value_from_char('𐍁'), DerivedPropertyValue::SpecClassPval);
/// // Character S is ASCII7 (K)
/// assert_eq!(ff.get_value_from_char('S'), DerivedPropertyValue::PValid);
/// // Character 0x1170 is OldHangulJamo (I)
/// assert_eq!(ff.get_value_from_codepoint(0x1170), DerivedPropertyValue::Disallowed);
/// ```
pub struct FreeformClass {}

impl SpecificDerivedPropertyValue for FreeformClass {
    fn on_has_compat(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassPval
    }
    fn on_other_letter_digits(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassPval
    }
    fn on_spaces(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassPval
    }
    fn on_symbols(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassPval
    }
    fn on_punctuation(&self) -> DerivedPropertyValue {
        DerivedPropertyValue::SpecClassPval
    }
}

impl StringClass for FreeformClass {
    fn get_value_from_char(&self, c: char) -> DerivedPropertyValue {
        get_derived_property_value(c as u32, self)
    }

    fn get_value_from_codepoint(&self, cp: u32) -> DerivedPropertyValue {
        get_derived_property_value(cp, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use precis_tools::*;
    use std::env;
    use std::fs::File;
    use std::path::Path;
    use std::path::PathBuf;

    fn validate_result(
        cp: u32,
        expected: precis_tools::DerivedProperty,
        id: &IdentifierClass,
        ff: &FreeformClass,
    ) {
        match expected {
            precis_tools::DerivedProperty::PValid => {
                let id_prop = id.get_value_from_codepoint(cp);
                let ff_prop = ff.get_value_from_codepoint(cp);

                assert_eq!(id_prop, DerivedPropertyValue::PValid);
                assert_eq!(ff_prop, DerivedPropertyValue::PValid);
            }
            precis_tools::DerivedProperty::FreePVal => {
                let ff_prop = ff.get_value_from_codepoint(cp);

                assert_eq!(ff_prop, DerivedPropertyValue::SpecClassPval)
            }
            precis_tools::DerivedProperty::ContextJ => {
                let id_prop = id.get_value_from_codepoint(cp);
                let ff_prop = ff.get_value_from_codepoint(cp);

                assert_eq!(id_prop, DerivedPropertyValue::ContextJ);
                assert_eq!(ff_prop, DerivedPropertyValue::ContextJ);
            }
            precis_tools::DerivedProperty::ContextO => {
                let id_prop = id.get_value_from_codepoint(cp);
                let ff_prop = ff.get_value_from_codepoint(cp);

                assert_eq!(id_prop, DerivedPropertyValue::ContextO);
                assert_eq!(ff_prop, DerivedPropertyValue::ContextO);
            }
            precis_tools::DerivedProperty::Disallowed => {
                let id_prop = id.get_value_from_codepoint(cp);
                let ff_prop = ff.get_value_from_codepoint(cp);

                assert_eq!(id_prop, DerivedPropertyValue::Disallowed);
                assert_eq!(ff_prop, DerivedPropertyValue::Disallowed);
            }
            precis_tools::DerivedProperty::IdDis => {
                let id_prop = id.get_value_from_codepoint(cp);
                assert_eq!(id_prop, DerivedPropertyValue::SpecClassDis);
            }
            precis_tools::DerivedProperty::Unassigned => {
                let id_prop = id.get_value_from_codepoint(cp);
                let ff_prop = ff.get_value_from_codepoint(cp);

                assert!(
                    id_prop == DerivedPropertyValue::Unassigned,
                    "failed check for unicode point: {:#06x}. Expected: {:?}, Got: {:?}",
                    cp,
                    expected,
                    id_prop
                );
                assert!(
                    ff_prop == DerivedPropertyValue::Unassigned,
                    "failed check for unicode point: {:#06x}. Expected: {:?}, Got: {:?}",
                    cp,
                    expected,
                    ff_prop
                );
            }
        }
    }

    fn check_derived_property(
        cp: u32,
        props: &DerivedProperties,
        id: &IdentifierClass,
        ff: &FreeformClass,
    ) {
        match props {
            precis_tools::DerivedProperties::Single(p) => validate_result(cp, *p, &id, &ff),
            precis_tools::DerivedProperties::Tuple((p1, p2)) => {
                validate_result(cp, *p1, &id, &ff);
                validate_result(cp, *p2, &id, &ff);
            }
        }
    }

    #[cfg(feature = "networking")]
    fn get_csv_path() -> PathBuf {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        Path::new(&out_dir).join("csv/precis-tables-6.3.0.csv")
    }

    #[cfg(not(feature = "networking"))]
    fn get_csv_path() -> PathBuf {
        let base_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&base_dir).join("resources/csv/precis-tables-6.3.0.csv")
    }

    #[test]
    fn check_derived_properties() {
        let id = IdentifierClass {};
        let ff = FreeformClass {};

        let csv_path = get_csv_path();

        let parser: precis_tools::CsvLineParser<File, precis_tools::PrecisDerivedProperty> =
            precis_tools::CsvLineParser::from_path(csv_path).unwrap();

        for result in parser {
            let prop = result.unwrap();
            match prop.codepoints {
                ucd_parse::Codepoints::Single(cp) => {
                    check_derived_property(cp.value(), &prop.properties, &id, &ff)
                }
                ucd_parse::Codepoints::Range(r) => {
                    for cp in r {
                        check_derived_property(cp.value(), &prop.properties, &id, &ff)
                    }
                }
            }
        }
    }
}
