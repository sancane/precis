//! `WebAssembly` bindings for `PRECIS` Framework
//!
//! This crate provides JavaScript-friendly bindings for the `PRECIS` Framework,
//! including multiple profiles:
//! - `Nickname` (RFC 8266)
//! - `OpaqueString` for passwords (RFC 8265)
//! - `UsernameCaseMapped` (RFC 8265)
//! - `UsernameCasePreserved` (RFC 8265)
//!
//! # Example (JavaScript)
//!
//! ```js
//! import { nickname_enforce, usernamecasemapped_enforce } from './precis.js';
//!
//! // Enforce nickname
//! const nick = nickname_enforce("  Alice  ");
//! console.log(nick); // "alice"
//!
//! // Enforce username (case-insensitive)
//! const username = usernamecasemapped_enforce("Alice");
//! console.log(username); // "alice"
//! ```

use precis_core::profile::PrecisFastInvocation;
use precis_profiles::{Nickname, OpaqueString, UsernameCaseMapped, UsernameCasePreserved};
use std::borrow::Cow;
use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in browser console
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generic helper for enforce/prepare operations that return Cow<str>
fn apply_string_operation<F>(input: JsValue, operation: F) -> Result<JsValue, JsError>
where
    F: FnOnce(&str) -> Result<Cow<str>, precis_core::Error>,
{
    if let Some(s) = input.as_string() {
        let result = operation(&s).map_err(|e| JsError::new(&format!("{}", e)))?;
        match result {
            Cow::Borrowed(_) => Ok(input), // Zero-copy when unchanged
            Cow::Owned(new_str) => Ok(JsValue::from_str(&new_str)),
        }
    } else {
        Err(JsError::new("Input must be a string"))
    }
}

/// Generic helper for compare operations
fn apply_compare_operation<F>(a: JsValue, b: JsValue, operation: F) -> Result<bool, JsError>
where
    F: FnOnce(&str, &str) -> Result<bool, precis_core::Error>,
{
    let a_str = a
        .as_string()
        .ok_or_else(|| JsError::new("First argument must be a string"))?;
    let b_str = b
        .as_string()
        .ok_or_else(|| JsError::new("Second argument must be a string"))?;
    operation(&a_str, &b_str).map_err(|e| JsError::new(&format!("{}", e)))
}

// ============================================================================
// Nickname Profile (RFC 8266)
// ============================================================================

/// Prepare `Nickname` string.
///
/// Ensures that the code points in a single input string are allowed by the
/// underlying `PRECIS` string class.
///
/// # Arguments
/// * `input` - String to prepare
///
/// # Returns
/// * `Ok(string)` - Prepared nickname
/// * `Err(string)` - Error message if preparation fails
///
/// # Example (JavaScript)
/// ```js
/// const prepared = nickname_prepare("Alice");
/// console.log(prepared); // "Alice"
/// ```
///
/// # Specification
/// [RFC 8266, Section 2.2: Nickname Profile](https://datatracker.ietf.org/doc/html/rfc8266#section-2.2)
#[wasm_bindgen]
pub fn nickname_prepare(input: JsValue) -> Result<JsValue, JsError> {
    apply_string_operation(input, |s| Nickname::prepare(s))
}

/// Enforce `Nickname` profile on input string.
///
/// Applies all rules specified for the `Nickname` profile to a single input string,
/// checking whether the string conforms to all rules.
///
/// # Arguments
/// * `input` - String to validate and normalize
///
/// # Returns
/// * `Ok(string)` - Normalized nickname
/// * `Err(string)` - Error message describing validation failure
///
/// # Example (JavaScript)
/// ```js
/// import { nickname_enforce } from 'precis-wasm';
///
/// try {
///   const nick = nickname_enforce("  Alice  ");
///   console.log(nick); // "Alice"
/// } catch (error) {
///   console.error(error);
/// }
/// ```
///
/// # Specification
/// [RFC 8266, Section 2.3: Enforcement](https://datatracker.ietf.org/doc/html/rfc8266#section-2.3)
#[wasm_bindgen]
pub fn nickname_enforce(input: JsValue) -> Result<JsValue, JsError> {
    apply_string_operation(input, |s| Nickname::enforce(s))
}

/// Compare two nicknames for equality.
///
/// Applies all rules specified for the `Nickname` profile to two separate input
/// strings and compares them.
///
/// # Arguments
/// * `a` - First nickname
/// * `b` - Second nickname
///
/// # Returns
/// * `Ok(true)` - Nicknames are equivalent
/// * `Ok(false)` - Nicknames are different
/// * `Err(string)` - Validation error
///
/// # Example (JavaScript)
/// ```js
/// const match1 = nickname_compare("Alice", "alice");
/// console.log(match1); // true
///
/// const match2 = nickname_compare("  Alice  ", "alice");
/// console.log(match2); // true
///
/// const match3 = nickname_compare("Alice", "Bob");
/// console.log(match3); // false
/// ```
///
/// # Specification
/// [RFC 8266, Section 2.4: Comparison](https://datatracker.ietf.org/doc/html/rfc8266#section-2.4)
#[wasm_bindgen]
pub fn nickname_compare(a: JsValue, b: JsValue) -> Result<bool, JsError> {
    apply_compare_operation(a, b, |s1, s2| Nickname::compare(s1, s2))
}

// ============================================================================
// OpaqueString Profile (RFC 8265) - Passwords
// ============================================================================

/// Prepare `OpaqueString`.
///
/// Ensures that the code points in a single input string are allowed by the
/// underlying `PRECIS` string class.
///
/// # Arguments
/// * `input` - String to prepare
///
/// # Returns
/// * `Ok(string)` - Prepared opaque string
/// * `Err(string)` - Error message if preparation fails
///
/// # Specification
/// [RFC 8265, Section 4.2.1: OpaqueString Profile](https://datatracker.ietf.org/doc/html/rfc8265#section-4.2.1)
#[wasm_bindgen]
pub fn opaquestring_prepare(input: JsValue) -> Result<JsValue, JsError> {
    apply_string_operation(input, |s| OpaqueString::prepare(s))
}

/// Enforce `OpaqueString` profile on input string.
///
/// Applies all rules specified for the `OpaqueString` profile. This profile is
/// used for passwords and other opaque strings where case preservation is important.
///
/// # Arguments
/// * `input` - String to validate and normalize
///
/// # Returns
/// * `Ok(string)` - Normalized opaque string
/// * `Err(string)` - Error message describing validation failure
///
/// # Example (JavaScript)
/// ```js
/// const password = opaquestring_enforce("MyP@ssw0rd");
/// console.log(password);
/// ```
///
/// # Specification
/// [RFC 8265, Section 4.2.2: Enforcement](https://datatracker.ietf.org/doc/html/rfc8265#section-4.2.2)
#[wasm_bindgen]
pub fn opaquestring_enforce(input: JsValue) -> Result<JsValue, JsError> {
    apply_string_operation(input, |s| OpaqueString::enforce(s))
}

/// Compare two opaque strings for equality.
///
/// Applies all rules specified for the `OpaqueString` profile to two separate
/// input strings and compares them.
///
/// # Arguments
/// * `a` - First opaque string
/// * `b` - Second opaque string
///
/// # Returns
/// * `Ok(true)` - Strings are equivalent
/// * `Ok(false)` - Strings are different
/// * `Err(string)` - Validation error
///
/// # Specification
/// [RFC 8265, Section 4.2.3: Comparison](https://datatracker.ietf.org/doc/html/rfc8265#section-4.2.3)
#[wasm_bindgen]
pub fn opaquestring_compare(a: JsValue, b: JsValue) -> Result<bool, JsError> {
    apply_compare_operation(a, b, |s1, s2| OpaqueString::compare(s1, s2))
}

// ============================================================================
// UsernameCaseMapped Profile (RFC 8265)
// ============================================================================

/// Prepare `UsernameCaseMapped` string.
///
/// Ensures that the code points in a single input string are allowed by the
/// underlying `PRECIS` string class.
///
/// # Arguments
/// * `input` - String to prepare
///
/// # Returns
/// * `Ok(string)` - Prepared username
/// * `Err(string)` - Error message if preparation fails
///
/// # Specification
/// [RFC 8265, Section 3.3.2: UsernameCaseMapped Profile](https://datatracker.ietf.org/doc/html/rfc8265#section-3.3.2)
#[wasm_bindgen]
pub fn usernamecasemapped_prepare(input: JsValue) -> Result<JsValue, JsError> {
    apply_string_operation(input, |s| UsernameCaseMapped::prepare(s))
}

/// Enforce `UsernameCaseMapped` profile on input string.
///
/// Applies all rules specified for the `UsernameCaseMapped` profile. This profile
/// applies case mapping (lower-casing) to usernames for case-insensitive comparison.
///
/// # Arguments
/// * `input` - String to validate and normalize
///
/// # Returns
/// * `Ok(string)` - Normalized username (lowercase)
/// * `Err(string)` - Error message describing validation failure
///
/// # Example (JavaScript)
/// ```js
/// const username = usernamecasemapped_enforce("Alice");
/// console.log(username); // "alice"
/// ```
///
/// # Specification
/// [RFC 8265, Section 3.3.3: Enforcement](https://datatracker.ietf.org/doc/html/rfc8265#section-3.3.3)
#[wasm_bindgen]
pub fn usernamecasemapped_enforce(input: JsValue) -> Result<JsValue, JsError> {
    apply_string_operation(input, |s| UsernameCaseMapped::enforce(s))
}

/// Compare two usernames for equality (case-insensitive).
///
/// Applies all rules specified for the `UsernameCaseMapped` profile to two separate
/// input strings and compares them.
///
/// # Arguments
/// * `a` - First username
/// * `b` - Second username
///
/// # Returns
/// * `Ok(true)` - Usernames are equivalent
/// * `Ok(false)` - Usernames are different
/// * `Err(string)` - Validation error
///
/// # Specification
/// [RFC 8265, Section 3.3.4: Comparison](https://datatracker.ietf.org/doc/html/rfc8265#section-3.3.4)
#[wasm_bindgen]
pub fn usernamecasemapped_compare(a: JsValue, b: JsValue) -> Result<bool, JsError> {
    apply_compare_operation(a, b, |s1, s2| UsernameCaseMapped::compare(s1, s2))
}

// ============================================================================
// UsernameCasePreserved Profile (RFC 8265)
// ============================================================================

/// Prepare `UsernameCasePreserved` string.
///
/// Ensures that the code points in a single input string are allowed by the
/// underlying `PRECIS` string class.
///
/// # Arguments
/// * `input` - String to prepare
///
/// # Returns
/// * `Ok(string)` - Prepared username
/// * `Err(string)` - Error message if preparation fails
///
/// # Specification
/// [RFC 8265, Section 3.4.2: UsernameCasePreserved Profile](https://datatracker.ietf.org/doc/html/rfc8265#section-3.4.2)
#[wasm_bindgen]
pub fn usernamecasepreserved_prepare(input: JsValue) -> Result<JsValue, JsError> {
    apply_string_operation(input, |s| UsernameCasePreserved::prepare(s))
}

/// Enforce `UsernameCasePreserved` profile on input string.
///
/// Applies all rules specified for the `UsernameCasePreserved` profile. This profile
/// preserves case in usernames for case-sensitive comparison.
///
/// # Arguments
/// * `input` - String to validate and normalize
///
/// # Returns
/// * `Ok(string)` - Normalized username (case preserved)
/// * `Err(string)` - Error message describing validation failure
///
/// # Example (JavaScript)
/// ```js
/// const username = usernamecasepreserved_enforce("Alice");
/// console.log(username); // "Alice"
/// ```
///
/// # Specification
/// [RFC 8265, Section 3.4.3: Enforcement](https://datatracker.ietf.org/doc/html/rfc8265#section-3.4.3)
#[wasm_bindgen]
pub fn usernamecasepreserved_enforce(input: JsValue) -> Result<JsValue, JsError> {
    apply_string_operation(input, |s| UsernameCasePreserved::enforce(s))
}

/// Compare two usernames for equality (case-sensitive).
///
/// Applies all rules specified for the `UsernameCasePreserved` profile to two
/// separate input strings and compares them. Comparison is case-sensitive.
///
/// # Arguments
/// * `a` - First username
/// * `b` - Second username
///
/// # Returns
/// * `Ok(true)` - Usernames are equivalent
/// * `Ok(false)` - Usernames are different
/// * `Err(string)` - Validation error
///
/// # Specification
/// [RFC 8265, Section 3.4.4: Comparison](https://datatracker.ietf.org/doc/html/rfc8265#section-3.4.4)
#[wasm_bindgen]
pub fn usernamecasepreserved_compare(a: JsValue, b: JsValue) -> Result<bool, JsError> {
    apply_compare_operation(a, b, |s1, s2| UsernameCasePreserved::compare(s1, s2))
}

// ============================================================================
// Version
// ============================================================================

/// Get the version of the `PRECIS` `WASM` library.
///
/// # Returns
/// Version string (e.g., "0.1.0")
///
/// # Example (JavaScript)
/// ```js
/// console.log(version()); // "0.1.0"
/// ```
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_nickname_enforce_basic() {
        assert_eq!(
            nickname_enforce(JsValue::from_str("Alice"))
                .unwrap()
                .as_string()
                .unwrap(),
            "Alice"
        );
        assert_eq!(
            nickname_enforce(JsValue::from_str("BOB"))
                .unwrap()
                .as_string()
                .unwrap(),
            "BOB"
        );
    }

    #[wasm_bindgen_test]
    fn test_nickname_enforce_spaces() {
        assert_eq!(
            nickname_enforce(JsValue::from_str("  Alice  "))
                .unwrap()
                .as_string()
                .unwrap(),
            "Alice"
        );
        assert_eq!(
            nickname_enforce(JsValue::from_str("Alice   Bob"))
                .unwrap()
                .as_string()
                .unwrap(),
            "Alice Bob"
        );
    }

    #[wasm_bindgen_test]
    fn test_nickname_enforce_unicode() {
        assert_eq!(
            nickname_enforce(JsValue::from_str("Café"))
                .unwrap()
                .as_string()
                .unwrap(),
            "Café"
        );
        assert_eq!(
            nickname_enforce(JsValue::from_str("José"))
                .unwrap()
                .as_string()
                .unwrap(),
            "José"
        );
    }

    #[wasm_bindgen_test]
    fn test_nickname_prepare() {
        assert_eq!(
            nickname_prepare(JsValue::from_str("Alice"))
                .unwrap()
                .as_string()
                .unwrap(),
            "Alice"
        );
    }

    #[wasm_bindgen_test]
    fn test_nickname_compare_equal() {
        assert!(nickname_compare(JsValue::from_str("Alice"), JsValue::from_str("alice")).unwrap());
        assert!(
            nickname_compare(JsValue::from_str("  Alice  "), JsValue::from_str("alice")).unwrap()
        );
        assert!(nickname_compare(JsValue::from_str("ALICE"), JsValue::from_str("alice")).unwrap());
    }

    #[wasm_bindgen_test]
    fn test_nickname_compare_different() {
        assert!(!nickname_compare(JsValue::from_str("Alice"), JsValue::from_str("Bob")).unwrap());
    }

    #[wasm_bindgen_test]
    fn test_version() {
        let ver = version();
        assert!(!ver.is_empty());
        assert!(ver.contains('.'));
    }
}
