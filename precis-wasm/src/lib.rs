//! WebAssembly bindings for PRECIS Framework
//!
//! This crate provides JavaScript-friendly bindings for the PRECIS Framework,
//! including multiple profiles:
//! - Nickname (RFC 8266)
//! - OpaqueString for passwords (RFC 8265)
//! - UsernameCaseMapped (RFC 8265)
//! - UsernameCasePreserved (RFC 8265)
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
// Nickname Profile (RFC 8266)
// ============================================================================

/// Enforce Nickname profile (RFC 8266) on input string.
///
/// This applies Unicode normalization (NFC), width mapping (fullwidth/halfwidth),
/// and trims leading/trailing spaces.
///
/// **Important:** Case is preserved during enforcement. Use `nickname_compare()`
/// for case-insensitive comparison.
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
///   console.log(nick); // "alice"
/// } catch (error) {
///   console.error(error);
/// }
/// ```
///
/// # RFC 8266 Section 2
///
/// The Nickname profile enforces the following rules:
/// 1. Width Mapping Rule: Fullwidth and halfwidth characters are mapped
/// 2. Additional Mapping Rule: Non-ASCII spaces mapped to ASCII space (U+0020)
/// 3. Case Mapping Rule: Case-insensitive for comparison only (case preserved in enforce)
/// 4. Normalization Rule: NFC
/// 5. Directionality Rule: Bidi rule (RFC 5893)
/// 6. Additional enforcement: Trim leading/trailing spaces, collapse multiple spaces
#[wasm_bindgen]
pub fn nickname_enforce(input: JsValue) -> Result<JsValue, JsError> {
    if let Some(s) = input.as_string() {
        let result = Nickname::enforce(&s).map_err(|e| JsError::new(&format!("{}", e)))?;

        match result {
            Cow::Borrowed(_) => Ok(input), // Zero-copy when unchanged
            Cow::Owned(new_str) => Ok(JsValue::from_str(&new_str)), // Copy only when modified
        }
    } else {
        Err(JsError::new("Input must be a string"))
    }
}

/// Prepare Nickname for comparison (RFC 8266).
///
/// Similar to `enforce` but applies additional normalization rules without
/// validation. Use this when you need to prepare a nickname that may already
/// be in a valid form.
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
/// console.log(prepared); // "alice"
/// ```
#[wasm_bindgen]
pub fn nickname_prepare(input: JsValue) -> Result<JsValue, JsError> {
    if let Some(s) = input.as_string() {
        let result = Nickname::prepare(&s).map_err(|e| JsError::new(&format!("{}", e)))?;

        match result {
            Cow::Borrowed(_) => Ok(input), // Zero-copy when unchanged
            Cow::Owned(new_str) => Ok(JsValue::from_str(&new_str)), // Copy only when modified
        }
    } else {
        Err(JsError::new("Input must be a string"))
    }
}

/// Compare two nicknames for equality after normalization.
///
/// This function enforces both nicknames and then compares them.
/// Nicknames are considered equal if they normalize to the same string.
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
#[wasm_bindgen]
pub fn nickname_compare(a: JsValue, b: JsValue) -> Result<bool, JsError> {
    let a_str = a
        .as_string()
        .ok_or_else(|| JsError::new("First argument must be a string"))?;
    let b_str = b
        .as_string()
        .ok_or_else(|| JsError::new("Second argument must be a string"))?;

    Nickname::compare(&a_str, &b_str).map_err(|e| JsError::new(&format!("{}", e)))
}

// ============================================================================
// OpaqueString Profile (RFC 8265) - Passwords
// ============================================================================

/// Enforce OpaqueString profile (RFC 8265) on input string.
///
/// The OpaqueString profile is used for passwords and other opaque strings
/// where case preservation is important.
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
#[wasm_bindgen]
pub fn opaquestring_enforce(input: JsValue) -> Result<JsValue, JsError> {
    if let Some(s) = input.as_string() {
        let result = OpaqueString::enforce(&s).map_err(|e| JsError::new(&format!("{}", e)))?;

        match result {
            Cow::Borrowed(_) => Ok(input),
            Cow::Owned(new_str) => Ok(JsValue::from_str(&new_str)),
        }
    } else {
        Err(JsError::new("Input must be a string"))
    }
}

/// Prepare OpaqueString for comparison (RFC 8265).
///
/// # Arguments
/// * `input` - String to prepare
///
/// # Returns
/// * `Ok(string)` - Prepared opaque string
/// * `Err(string)` - Error message if preparation fails
#[wasm_bindgen]
pub fn opaquestring_prepare(input: JsValue) -> Result<JsValue, JsError> {
    if let Some(s) = input.as_string() {
        let result = OpaqueString::prepare(&s).map_err(|e| JsError::new(&format!("{}", e)))?;

        match result {
            Cow::Borrowed(_) => Ok(input),
            Cow::Owned(new_str) => Ok(JsValue::from_str(&new_str)),
        }
    } else {
        Err(JsError::new("Input must be a string"))
    }
}

/// Compare two opaque strings for equality after normalization.
///
/// # Arguments
/// * `a` - First opaque string
/// * `b` - Second opaque string
///
/// # Returns
/// * `Ok(true)` - Strings are equivalent
/// * `Ok(false)` - Strings are different
/// * `Err(string)` - Validation error
#[wasm_bindgen]
pub fn opaquestring_compare(a: JsValue, b: JsValue) -> Result<bool, JsError> {
    let a_str = a
        .as_string()
        .ok_or_else(|| JsError::new("First argument must be a string"))?;
    let b_str = b
        .as_string()
        .ok_or_else(|| JsError::new("Second argument must be a string"))?;

    OpaqueString::compare(&a_str, &b_str).map_err(|e| JsError::new(&format!("{}", e)))
}

// ============================================================================
// UsernameCaseMapped Profile (RFC 8265)
// ============================================================================

/// Enforce UsernameCaseMapped profile (RFC 8265) on input string.
///
/// This profile applies case mapping (lowercasing) to usernames for
/// case-insensitive comparison.
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
#[wasm_bindgen]
pub fn usernamecasemapped_enforce(input: JsValue) -> Result<JsValue, JsError> {
    if let Some(s) = input.as_string() {
        let result =
            UsernameCaseMapped::enforce(&s).map_err(|e| JsError::new(&format!("{}", e)))?;

        match result {
            Cow::Borrowed(_) => Ok(input),
            Cow::Owned(new_str) => Ok(JsValue::from_str(&new_str)),
        }
    } else {
        Err(JsError::new("Input must be a string"))
    }
}

/// Prepare UsernameCaseMapped for comparison (RFC 8265).
///
/// # Arguments
/// * `input` - String to prepare
///
/// # Returns
/// * `Ok(string)` - Prepared username
/// * `Err(string)` - Error message if preparation fails
#[wasm_bindgen]
pub fn usernamecasemapped_prepare(input: JsValue) -> Result<JsValue, JsError> {
    if let Some(s) = input.as_string() {
        let result =
            UsernameCaseMapped::prepare(&s).map_err(|e| JsError::new(&format!("{}", e)))?;

        match result {
            Cow::Borrowed(_) => Ok(input),
            Cow::Owned(new_str) => Ok(JsValue::from_str(&new_str)),
        }
    } else {
        Err(JsError::new("Input must be a string"))
    }
}

/// Compare two usernames for equality after normalization (case-insensitive).
///
/// # Arguments
/// * `a` - First username
/// * `b` - Second username
///
/// # Returns
/// * `Ok(true)` - Usernames are equivalent
/// * `Ok(false)` - Usernames are different
/// * `Err(string)` - Validation error
#[wasm_bindgen]
pub fn usernamecasemapped_compare(a: JsValue, b: JsValue) -> Result<bool, JsError> {
    let a_str = a
        .as_string()
        .ok_or_else(|| JsError::new("First argument must be a string"))?;
    let b_str = b
        .as_string()
        .ok_or_else(|| JsError::new("Second argument must be a string"))?;

    UsernameCaseMapped::compare(&a_str, &b_str).map_err(|e| JsError::new(&format!("{}", e)))
}

// ============================================================================
// UsernameCasePreserved Profile (RFC 8265)
// ============================================================================

/// Enforce UsernameCasePreserved profile (RFC 8265) on input string.
///
/// This profile preserves case in usernames for case-sensitive comparison.
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
#[wasm_bindgen]
pub fn usernamecasepreserved_enforce(input: JsValue) -> Result<JsValue, JsError> {
    if let Some(s) = input.as_string() {
        let result =
            UsernameCasePreserved::enforce(&s).map_err(|e| JsError::new(&format!("{}", e)))?;

        match result {
            Cow::Borrowed(_) => Ok(input),
            Cow::Owned(new_str) => Ok(JsValue::from_str(&new_str)),
        }
    } else {
        Err(JsError::new("Input must be a string"))
    }
}

/// Prepare UsernameCasePreserved for comparison (RFC 8265).
///
/// # Arguments
/// * `input` - String to prepare
///
/// # Returns
/// * `Ok(string)` - Prepared username
/// * `Err(string)` - Error message if preparation fails
#[wasm_bindgen]
pub fn usernamecasepreserved_prepare(input: JsValue) -> Result<JsValue, JsError> {
    if let Some(s) = input.as_string() {
        let result =
            UsernameCasePreserved::prepare(&s).map_err(|e| JsError::new(&format!("{}", e)))?;

        match result {
            Cow::Borrowed(_) => Ok(input),
            Cow::Owned(new_str) => Ok(JsValue::from_str(&new_str)),
        }
    } else {
        Err(JsError::new("Input must be a string"))
    }
}

/// Compare two usernames for equality after normalization (case-sensitive).
///
/// # Arguments
/// * `a` - First username
/// * `b` - Second username
///
/// # Returns
/// * `Ok(true)` - Usernames are equivalent
/// * `Ok(false)` - Usernames are different
/// * `Err(string)` - Validation error
#[wasm_bindgen]
pub fn usernamecasepreserved_compare(a: JsValue, b: JsValue) -> Result<bool, JsError> {
    let a_str = a
        .as_string()
        .ok_or_else(|| JsError::new("First argument must be a string"))?;
    let b_str = b
        .as_string()
        .ok_or_else(|| JsError::new("Second argument must be a string"))?;

    UsernameCasePreserved::compare(&a_str, &b_str).map_err(|e| JsError::new(&format!("{}", e)))
}

// ============================================================================
// Version
// ============================================================================

/// Get the version of the PRECIS WASM library.
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
            "alice"
        );
        assert_eq!(
            nickname_enforce(JsValue::from_str("BOB"))
                .unwrap()
                .as_string()
                .unwrap(),
            "bob"
        );
    }

    #[wasm_bindgen_test]
    fn test_nickname_enforce_spaces() {
        assert_eq!(
            nickname_enforce(JsValue::from_str("  Alice  "))
                .unwrap()
                .as_string()
                .unwrap(),
            "alice"
        );
        assert_eq!(
            nickname_enforce(JsValue::from_str("Alice   Bob"))
                .unwrap()
                .as_string()
                .unwrap(),
            "alice bob"
        );
    }

    #[wasm_bindgen_test]
    fn test_nickname_enforce_unicode() {
        assert_eq!(
            nickname_enforce(JsValue::from_str("Café"))
                .unwrap()
                .as_string()
                .unwrap(),
            "café"
        );
        assert_eq!(
            nickname_enforce(JsValue::from_str("José"))
                .unwrap()
                .as_string()
                .unwrap(),
            "josé"
        );
    }

    #[wasm_bindgen_test]
    fn test_nickname_prepare() {
        assert_eq!(
            nickname_prepare(JsValue::from_str("Alice"))
                .unwrap()
                .as_string()
                .unwrap(),
            "alice"
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
