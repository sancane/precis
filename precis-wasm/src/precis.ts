/**
 * TypeScript wrapper for `precis-wasm`
 *
 * This module provides ergonomic TypeScript types for the `PRECIS` `Nickname` profile.
 * It wraps the auto-generated `WASM` bindings with proper type signatures.
 */

// @ts-ignore - precis_wasm.js is generated at build time
import * as wasm from './precis_wasm.js';

/**
 * Prepare `Nickname` string.
 *
 * Ensures that the code points in a single input string are allowed by the
 * underlying `PRECIS` string class.
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
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8266#section-2.2 | RFC 8266, Section 2.2: Nickname Profile}
 */
export function nickname_prepare(input: string): string {
    return wasm.nickname_prepare(input);
}

/**
 * Enforce `Nickname` profile on input string.
 *
 * Applies all rules specified for the `Nickname` profile to a single input string,
 * checking whether the string conforms to all rules.
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
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8266#section-2.3 | RFC 8266, Section 2.3: Enforcement}
 */
export function nickname_enforce(input: string): string {
    return wasm.nickname_enforce(input);
}

/**
 * Compare two nicknames for equality.
 *
 * Applies all rules specified for the `Nickname` profile to two separate input
 * strings and compares them.
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
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8266#section-2.4 | RFC 8266, Section 2.4: Comparison}
 */
export function nickname_compare(a: string, b: string): boolean {
    return wasm.nickname_compare(a, b);
}

// ============================================================================
// OpaqueString Profile (RFC 8265) - Passwords
// ============================================================================

/**
 * Prepare `OpaqueString`.
 *
 * Ensures that the code points in a single input string are allowed by the
 * underlying `PRECIS` string class.
 *
 * @param input - String to prepare
 * @returns Prepared opaque string
 * @throws Error if preparation fails or input is not a string
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8265#section-4.2.1 | RFC 8265, Section 4.2.1: OpaqueString Profile}
 */
export function opaquestring_prepare(input: string): string {
    return wasm.opaquestring_prepare(input);
}

/**
 * Enforce `OpaqueString` profile on input string.
 *
 * Applies all rules specified for the `OpaqueString` profile. This profile is
 * used for passwords and other opaque strings where case preservation is important.
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
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8265#section-4.2.2 | RFC 8265, Section 4.2.2: Enforcement}
 */
export function opaquestring_enforce(input: string): string {
    return wasm.opaquestring_enforce(input);
}

/**
 * Compare two opaque strings for equality.
 *
 * Applies all rules specified for the `OpaqueString` profile to two separate
 * input strings and compares them.
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
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8265#section-4.2.3 | RFC 8265, Section 4.2.3: Comparison}
 */
export function opaquestring_compare(a: string, b: string): boolean {
    return wasm.opaquestring_compare(a, b);
}

// ============================================================================
// UsernameCaseMapped Profile (RFC 8265)
// ============================================================================

/**
 * Prepare `UsernameCaseMapped` string.
 *
 * Ensures that the code points in a single input string are allowed by the
 * underlying `PRECIS` string class.
 *
 * @param input - String to prepare
 * @returns Prepared username
 * @throws Error if preparation fails or input is not a string
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8265#section-3.3.2 | RFC 8265, Section 3.3.2: UsernameCaseMapped Profile}
 */
export function usernamecasemapped_prepare(input: string): string {
    return wasm.usernamecasemapped_prepare(input);
}

/**
 * Enforce `UsernameCaseMapped` profile on input string.
 *
 * Applies all rules specified for the `UsernameCaseMapped` profile. This profile
 * applies case mapping (lower-casing) to usernames for case-insensitive comparison.
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
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8265#section-3.3.3 | RFC 8265, Section 3.3.3: Enforcement}
 */
export function usernamecasemapped_enforce(input: string): string {
    return wasm.usernamecasemapped_enforce(input);
}

/**
 * Compare two usernames for equality (case-insensitive).
 *
 * Applies all rules specified for the `UsernameCaseMapped` profile to two separate
 * input strings and compares them.
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
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8265#section-3.3.4 | RFC 8265, Section 3.3.4: Comparison}
 */
export function usernamecasemapped_compare(a: string, b: string): boolean {
    return wasm.usernamecasemapped_compare(a, b);
}

// ============================================================================
// UsernameCasePreserved Profile (RFC 8265)
// ============================================================================

/**
 * Prepare `UsernameCasePreserved` string.
 *
 * Ensures that the code points in a single input string are allowed by the
 * underlying `PRECIS` string class.
 *
 * @param input - String to prepare
 * @returns Prepared username
 * @throws Error if preparation fails or input is not a string
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8265#section-3.4.2 | RFC 8265, Section 3.4.2: UsernameCasePreserved Profile}
 */
export function usernamecasepreserved_prepare(input: string): string {
    return wasm.usernamecasepreserved_prepare(input);
}

/**
 * Enforce `UsernameCasePreserved` profile on input string.
 *
 * Applies all rules specified for the `UsernameCasePreserved` profile. This profile
 * preserves case in usernames for case-sensitive comparison.
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
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8265#section-3.4.3 | RFC 8265, Section 3.4.3: Enforcement}
 */
export function usernamecasepreserved_enforce(input: string): string {
    return wasm.usernamecasepreserved_enforce(input);
}

/**
 * Compare two usernames for equality (case-sensitive).
 *
 * Applies all rules specified for the `UsernameCasePreserved` profile to two
 * separate input strings and compares them.
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
 *
 * @see {@link https://datatracker.ietf.org/doc/html/rfc8265#section-3.4.4 | RFC 8265, Section 3.4.4: Comparison}
 */
export function usernamecasepreserved_compare(a: string, b: string): boolean {
    return wasm.usernamecasepreserved_compare(a, b);
}

// ============================================================================
// Version
// ============================================================================

/**
 * Get the version of the `PRECIS` `WASM` library.
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
 * Initialize the `WASM` module.
 *
 * This function must be called before using any `PRECIS` functions in browser environments.
 * In Node.js, the `WASM` module initializes automatically on first use.
 *
 * @returns Promise that resolves when `WASM` is initialized
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
