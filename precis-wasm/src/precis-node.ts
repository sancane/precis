/**
 * Node.js wrapper for `precis-wasm`
 *
 * This module provides ergonomic TypeScript types for the `PRECIS` profiles on
 * top of the `wasm-pack --target nodejs` glue. Unlike the browser build, the
 * Node.js target instantiates the `WASM` module synchronously on `require`, so
 * no asynchronous initialization is needed.
 */

// @ts-ignore - precis_node.js is the `wasm-pack --target nodejs` glue, generated at build time
import * as wasm from './precis_node.js';

/**
 * Initialize the `WASM` module.
 *
 * The Node.js target initializes automatically on first `require`, so this is a
 * no-op kept only for API symmetry with the browser build (where `await init()`
 * is required). It resolves immediately.
 */
export async function init(): Promise<void> {
    return;
}

/** Prepare `Nickname` string. @see RFC 8266 §2.2 */
export function nickname_prepare(input: string): string {
    return wasm.nickname_prepare(input);
}

/** Enforce `Nickname` profile (trims spaces, preserves case). @see RFC 8266 §2.3 */
export function nickname_enforce(input: string): string {
    return wasm.nickname_enforce(input);
}

/** Compare two nicknames for equality. @see RFC 8266 §2.4 */
export function nickname_compare(a: string, b: string): boolean {
    return wasm.nickname_compare(a, b);
}

/** Prepare `OpaqueString`. @see RFC 8265 §4.2.1 */
export function opaquestring_prepare(input: string): string {
    return wasm.opaquestring_prepare(input);
}

/** Enforce `OpaqueString` profile (passwords; case preserved). @see RFC 8265 §4.2.2 */
export function opaquestring_enforce(input: string): string {
    return wasm.opaquestring_enforce(input);
}

/** Compare two opaque strings for equality (case-sensitive). @see RFC 8265 §4.2.3 */
export function opaquestring_compare(a: string, b: string): boolean {
    return wasm.opaquestring_compare(a, b);
}

/** Prepare `UsernameCaseMapped` string. @see RFC 8265 §3.3.2 */
export function usernamecasemapped_prepare(input: string): string {
    return wasm.usernamecasemapped_prepare(input);
}

/** Enforce `UsernameCaseMapped` profile (lower-cased). @see RFC 8265 §3.3.3 */
export function usernamecasemapped_enforce(input: string): string {
    return wasm.usernamecasemapped_enforce(input);
}

/** Compare two usernames for equality (case-insensitive). @see RFC 8265 §3.3.4 */
export function usernamecasemapped_compare(a: string, b: string): boolean {
    return wasm.usernamecasemapped_compare(a, b);
}

/** Prepare `UsernameCasePreserved` string. @see RFC 8265 §3.4.2 */
export function usernamecasepreserved_prepare(input: string): string {
    return wasm.usernamecasepreserved_prepare(input);
}

/** Enforce `UsernameCasePreserved` profile (case preserved). @see RFC 8265 §3.4.3 */
export function usernamecasepreserved_enforce(input: string): string {
    return wasm.usernamecasepreserved_enforce(input);
}

/** Compare two usernames for equality (case-sensitive). @see RFC 8265 §3.4.4 */
export function usernamecasepreserved_compare(a: string, b: string): boolean {
    return wasm.usernamecasepreserved_compare(a, b);
}

/** Get the version of the `precis-wasm` bindings. */
export function version(): string {
    return wasm.version();
}
