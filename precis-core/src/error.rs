/// Represents any kind of error that may happen when
/// preparing, enforcing or comparing of internationalized
/// strings
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Operation disallowed
    Disallowed,
    /// Operation not applicable
    NotApplicable,
    /// Undefined error
    Undefined,
    /// Unexpected error
    Unexpected,
}
