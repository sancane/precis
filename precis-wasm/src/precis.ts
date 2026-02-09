/**
 * TypeScript wrapper for precis-wasm
 *
 * This module provides ergonomic TypeScript types for the PRECIS Nickname profile.
 * It wraps the auto-generated WASM bindings with proper type signatures.
 */

// @ts-ignore - precis_wasm.js is generated at build time
import * as wasm from './precis_wasm.js';

/**
 * Enforce Nickname profile (RFC 8266) on input string.
 *
 * This applies Unicode normalization (NFC), width mapping (fullwidth/halfwidth),
 * and trims leading/trailing spaces. Note: Case is preserved during enforcement.
 *
 * @param input - String to validate and normalize
 * @returns Normalized nickname
 * @throws Error if validation fails or input is not a string
 *
 * @example
 * ```typescript
 * import { nickname_enforce } from '@precis/wasm';
 *
 * const nick = nickname_enforce("  Alice  ");
 * console.log(nick); // "Alice"
 * ```
 *
 * RFC 8266 Section 2 - The Nickname profile enforces:
 * 1. Width Mapping Rule: Fullwidth and halfwidth characters are mapped
 * 2. Additional Mapping Rule: Non-ASCII spaces mapped to ASCII space (U+0020)
 * 3. Normalization Rule: NFC
 * 4. Directionality Rule: Bidi rule (RFC 5893)
 * 5. Additional enforcement: Trim leading/trailing spaces, collapse multiple spaces
 */
export function nickname_enforce(input: string): string {
    return wasm.nickname_enforce(input);
}

/**
 * Prepare Nickname for comparison (RFC 8266).
 *
 * Similar to `enforce` but applies additional normalization rules without
 * validation. Use this when you need to prepare a nickname that may already
 * be in a valid form.
 *
 * @param input - String to prepare
 * @returns Prepared nickname
 * @throws Error if preparation fails or input is not a string
 *
 * @example
 * ```typescript
 * const prepared = nickname_prepare("Alice");
 * console.log(prepared); // "Alice"
 * ```
 */
export function nickname_prepare(input: string): string {
    return wasm.nickname_prepare(input);
}

/**
 * Compare two nicknames for equality after normalization.
 *
 * This function enforces both nicknames and then compares them.
 * Nicknames are considered equal if they normalize to the same string.
 *
 * @param a - First nickname
 * @param b - Second nickname
 * @returns true if nicknames are equivalent, false otherwise
 * @throws Error if validation fails or inputs are not strings
 *
 * @example
 * ```typescript
 * const match1 = nickname_compare("Alice", "alice");
 * console.log(match1); // true
 *
 * const match2 = nickname_compare("  Alice  ", "alice");
 * console.log(match2); // true
 *
 * const match3 = nickname_compare("Alice", "Bob");
 * console.log(match3); // false
 * ```
 */
export function nickname_compare(a: string, b: string): boolean {
    return wasm.nickname_compare(a, b);
}

// ============================================================================
// OpaqueString Profile (RFC 8265) - Passwords
// ============================================================================

/**
 * Enforce OpaqueString profile (RFC 8265) on input string.
 *
 * The OpaqueString profile is used for passwords and other opaque strings
 * where case preservation is important.
 *
 * @param input - String to validate and normalize
 * @returns Normalized opaque string
 * @throws Error if validation fails or input is not a string
 *
 * @example
 * ```typescript
 * const password = opaquestring_enforce("MyP@ssw0rd");
 * console.log(password);
 * ```
 */
export function opaquestring_enforce(input: string): string {
    return wasm.opaquestring_enforce(input);
}

/**
 * Prepare OpaqueString for comparison (RFC 8265).
 *
 * @param input - String to prepare
 * @returns Prepared opaque string
 * @throws Error if preparation fails or input is not a string
 */
export function opaquestring_prepare(input: string): string {
    return wasm.opaquestring_prepare(input);
}

/**
 * Compare two opaque strings for equality after normalization.
 *
 * @param a - First opaque string
 * @param b - Second opaque string
 * @returns true if strings are equivalent, false otherwise
 * @throws Error if validation fails or inputs are not strings
 *
 * @example
 * ```typescript
 * const match = opaquestring_compare("password", "password");
 * console.log(match); // true
 * ```
 */
export function opaquestring_compare(a: string, b: string): boolean {
    return wasm.opaquestring_compare(a, b);
}

// ============================================================================
// UsernameCaseMapped Profile (RFC 8265)
// ============================================================================

/**
 * Enforce UsernameCaseMapped profile (RFC 8265) on input string.
 *
 * This profile applies case mapping (lowercasing) to usernames for
 * case-insensitive comparison.
 *
 * @param input - String to validate and normalize
 * @returns Normalized username (lowercase)
 * @throws Error if validation fails or input is not a string
 *
 * @example
 * ```typescript
 * const username = usernamecasemapped_enforce("Alice");
 * console.log(username); // "alice"
 * ```
 */
export function usernamecasemapped_enforce(input: string): string {
    return wasm.usernamecasemapped_enforce(input);
}

/**
 * Prepare UsernameCaseMapped for comparison (RFC 8265).
 *
 * @param input - String to prepare
 * @returns Prepared username
 * @throws Error if preparation fails or input is not a string
 */
export function usernamecasemapped_prepare(input: string): string {
    return wasm.usernamecasemapped_prepare(input);
}

/**
 * Compare two usernames for equality after normalization (case-insensitive).
 *
 * @param a - First username
 * @param b - Second username
 * @returns true if usernames are equivalent, false otherwise
 * @throws Error if validation fails or inputs are not strings
 *
 * @example
 * ```typescript
 * const match = usernamecasemapped_compare("Alice", "alice");
 * console.log(match); // true
 * ```
 */
export function usernamecasemapped_compare(a: string, b: string): boolean {
    return wasm.usernamecasemapped_compare(a, b);
}

// ============================================================================
// UsernameCasePreserved Profile (RFC 8265)
// ============================================================================

/**
 * Enforce UsernameCasePreserved profile (RFC 8265) on input string.
 *
 * This profile preserves case in usernames for case-sensitive comparison.
 *
 * @param input - String to validate and normalize
 * @returns Normalized username (case preserved)
 * @throws Error if validation fails or input is not a string
 *
 * @example
 * ```typescript
 * const username = usernamecasepreserved_enforce("Alice");
 * console.log(username); // "Alice"
 * ```
 */
export function usernamecasepreserved_enforce(input: string): string {
    return wasm.usernamecasepreserved_enforce(input);
}

/**
 * Prepare UsernameCasePreserved for comparison (RFC 8265).
 *
 * @param input - String to prepare
 * @returns Prepared username
 * @throws Error if preparation fails or input is not a string
 */
export function usernamecasepreserved_prepare(input: string): string {
    return wasm.usernamecasepreserved_prepare(input);
}

/**
 * Compare two usernames for equality after normalization (case-sensitive).
 *
 * @param a - First username
 * @param b - Second username
 * @returns true if usernames are equivalent, false otherwise
 * @throws Error if validation fails or inputs are not strings
 *
 * @example
 * ```typescript
 * const match = usernamecasepreserved_compare("Alice", "alice");
 * console.log(match); // false (case matters!)
 * ```
 */
export function usernamecasepreserved_compare(a: string, b: string): boolean {
    return wasm.usernamecasepreserved_compare(a, b);
}

// ============================================================================
// Version
// ============================================================================

/**
 * Get the version of the PRECIS WASM library.
 *
 * @returns Version string (e.g., "0.1.0")
 *
 * @example
 * ```typescript
 * console.log(version()); // "0.1.0"
 * ```
 */
export function version(): string {
    return wasm.version();
}

/**
 * Initialize the WASM module.
 *
 * This function must be called before using any PRECIS functions in browser environments.
 * In Node.js, the WASM module initializes automatically on first use.
 *
 * @returns Promise that resolves when WASM is initialized
 *
 * @example
 * ```typescript
 * // Browser usage
 * import { init, nickname_enforce } from '@precis/wasm';
 *
 * await init();
 * const result = nickname_enforce("Alice");
 * ```
 */
// @ts-ignore - precis_wasm.js is generated at build time
export { default as init } from './precis_wasm.js';
