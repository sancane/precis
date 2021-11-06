/// Represents any kind of error that can occur while parsing files
#[derive(Debug, PartialEq)]
pub enum Error {
    Disallowed,
    NotApplicable,
    Undefined,
    Unexpected,
}
