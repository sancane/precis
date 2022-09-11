use crate::DerivedPropertyValue;

/// Represents any kind of error that may happen when
/// preparing, enforcing or comparing internationalized
/// strings
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid label
    Invalid,
    /// Detected a disallowed Unicode code pint in the label.
    /// [`CodepointInfo`] contains information about the code point.
    BadCodepoint(CodepointInfo),
    /// Error used to deal with any unexpected condition not directly
    /// covered by any other category.
    Unexpected(UnexpectedError),
}

/// Error that contains information regarding the wrong Unicode code point
#[derive(Debug, PartialEq, Eq)]
pub struct CodepointInfo {
    /// Unicode code point
    pub cp: u32,
    /// The position of the Unicode code point in the label
    pub position: usize,
    /// The derived property value
    pub property: DerivedPropertyValue,
}

impl CodepointInfo {
    /// Creates a new `CodepointInfo` `struct`
    pub fn new(cp: u32, position: usize, property: DerivedPropertyValue) -> Self {
        Self {
            cp,
            position,
            property,
        }
    }
}

/// Internal errors that group unusual error conditions that mostly
/// have to do with the processing of wrong labels, unexpected Unicode
/// code points if tested against another version defined in PRECIS, etc.
#[derive(Debug, PartialEq, Eq)]
pub enum UnexpectedError {
    /// Error caused when trying to apply a context rule over
    /// an invalid code point.
    ContextRuleNotApplicable(CodepointInfo),
    /// The code point requires a context rule that is not implemented.
    /// [`CodepointInfo`] contains information about the code point.
    MissingContextRule(CodepointInfo),
    /// Error caused when trying to apply a context rule that is not defined
    /// by the PRECIS profile.
    ProfileRuleNotApplicable,
    /// Unexpected error condition such as an attempt to access to a character before
    /// the start of a label or after the end of a label.
    Undefined,
}
