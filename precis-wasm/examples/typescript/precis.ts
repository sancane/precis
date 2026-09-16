/**
 * TypeScript example for precis-wasm
 *
 * This example demonstrates the use of all PRECIS profiles with proper TypeScript types.
 */

// Dynamic import to handle CommonJS/ESM interop with pkg-node
const precis = await import('../../pkg/precis-node.js');

const {
    nickname_enforce,
    nickname_prepare,
    nickname_compare,
    opaquestring_enforce,
    opaquestring_compare,
    usernamecasemapped_enforce,
    usernamecasemapped_compare,
    usernamecasepreserved_enforce,
    usernamecasepreserved_compare,
    version
} = precis;

// Note: When using Node.js target (pkg-node), WASM initializes automatically

console.log('='.repeat(60));
console.log('PRECIS WASM TypeScript Example');
console.log(`Version: ${version()}`);
console.log('='.repeat(60));
console.log();

// ============================================================================
// Nickname Profile (RFC 8266)
// ============================================================================

console.log('📝 Nickname Profile (RFC 8266)');
console.log('-'.repeat(60));

try {
    // Enforce: Trims spaces, preserves case
    const nick1: string = nickname_enforce("  Alice  ");
    console.log(`✓ nickname_enforce("  Alice  ") = "${nick1}"`);

    const nick2: string = nickname_enforce("BOB");
    console.log(`✓ nickname_enforce("BOB") = "${nick2}"`);

    const nick3: string = nickname_enforce("José");
    console.log(`✓ nickname_enforce("José") = "${nick3}"`);

    // Prepare: For comparison without strict validation
    const prepared: string = nickname_prepare("Alice");
    console.log(`✓ nickname_prepare("Alice") = "${prepared}"`);

    // Compare: Case-insensitive comparison
    const match1: boolean = nickname_compare("Alice", "alice");
    console.log(`✓ nickname_compare("Alice", "alice") = ${match1}`);

    const match2: boolean = nickname_compare("Alice", "Bob");
    console.log(`✓ nickname_compare("Alice", "Bob") = ${match2}`);
} catch (error: unknown) {
    if (error instanceof Error) {
        console.error(`✗ Error: ${error.message}`);
    }
}

console.log();

// ============================================================================
// OpaqueString Profile (RFC 8265) - Passwords
// ============================================================================

console.log('🔒 OpaqueString Profile (RFC 8265) - Passwords');
console.log('-'.repeat(60));

try {
    // Enforce: Case-sensitive, preserves special characters
    const password1: string = opaquestring_enforce("MyP@ssw0rd!");
    console.log(`✓ opaquestring_enforce("MyP@ssw0rd!") = "${password1}"`);

    const password2: string = opaquestring_enforce("correct horse battery staple");
    console.log(`✓ opaquestring_enforce("correct horse...") = "${password2}"`);

    // Compare: Exact match (case-sensitive)
    const passMatch1: boolean = opaquestring_compare("MyP@ssw0rd!", "MyP@ssw0rd!");
    console.log(`✓ opaquestring_compare("MyP@ssw0rd!", "MyP@ssw0rd!") = ${passMatch1}`);

    const passMatch2: boolean = opaquestring_compare("MyP@ssw0rd!", "myp@ssw0rd!");
    console.log(`✓ opaquestring_compare("MyP@ssw0rd!", "myp@ssw0rd!") = ${passMatch2} (case matters!)`);
} catch (error: unknown) {
    if (error instanceof Error) {
        console.error(`✗ Error: ${error.message}`);
    }
}

console.log();

// ============================================================================
// UsernameCaseMapped Profile (RFC 8265) - Case-insensitive usernames
// ============================================================================

console.log('👤 UsernameCaseMapped Profile (RFC 8265)');
console.log('-'.repeat(60));

try {
    // Enforce: Converts to lowercase
    const user1: string = usernamecasemapped_enforce("Alice");
    console.log(`✓ usernamecasemapped_enforce("Alice") = "${user1}"`);

    const user2: string = usernamecasemapped_enforce("bob_smith");
    console.log(`✓ usernamecasemapped_enforce("bob_smith") = "${user2}"`);

    // Compare: Case-insensitive
    const userMatch1: boolean = usernamecasemapped_compare("Alice", "alice");
    console.log(`✓ usernamecasemapped_compare("Alice", "alice") = ${userMatch1}`);

    const userMatch2: boolean = usernamecasemapped_compare("Alice", "Bob");
    console.log(`✓ usernamecasemapped_compare("Alice", "Bob") = ${userMatch2}`);
} catch (error: unknown) {
    if (error instanceof Error) {
        console.error(`✗ Error: ${error.message}`);
    }
}

console.log();

// ============================================================================
// UsernameCasePreserved Profile (RFC 8265) - Case-sensitive usernames
// ============================================================================

console.log('👤 UsernameCasePreserved Profile (RFC 8265)');
console.log('-'.repeat(60));

try {
    // Enforce: Preserves case
    const user1: string = usernamecasepreserved_enforce("Alice");
    console.log(`✓ usernamecasepreserved_enforce("Alice") = "${user1}"`);

    const user2: string = usernamecasepreserved_enforce("Bob_Smith");
    console.log(`✓ usernamecasepreserved_enforce("Bob_Smith") = "${user2}"`);

    // Compare: Case-sensitive
    const userMatch1: boolean = usernamecasepreserved_compare("Alice", "Alice");
    console.log(`✓ usernamecasepreserved_compare("Alice", "Alice") = ${userMatch1}`);

    const userMatch2: boolean = usernamecasepreserved_compare("Alice", "alice");
    console.log(`✓ usernamecasepreserved_compare("Alice", "alice") = ${userMatch2} (case matters!)`);
} catch (error: unknown) {
    if (error instanceof Error) {
        console.error(`✗ Error: ${error.message}`);
    }
}

console.log();

// ============================================================================
// Type Safety Example
// ============================================================================

console.log('🔒 TypeScript Type Safety');
console.log('-'.repeat(60));

// All functions have proper string → string types
const enforced: string = nickname_enforce("test");
const compared: boolean = nickname_compare("a", "b");

// TypeScript will catch errors at compile time:
// nickname_enforce(123);        // ✗ Compile error: Argument of type 'number' is not assignable
// nickname_enforce();           // ✗ Compile error: Expected 1 argument
// const x: number = enforced;   // ✗ Compile error: Type 'string' is not assignable to type 'number'

console.log('✓ All functions have correct TypeScript types');
console.log('✓ string → string (not any → any)');
console.log('✓ Compile-time type checking prevents errors');

console.log();
console.log('='.repeat(60));
console.log('✅ All examples completed successfully!');
console.log('='.repeat(60));

// Make this file a module to allow top-level await
export {};
