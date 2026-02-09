# precis-wasm

WebAssembly bindings for PRECIS Framework profiles (RFC 8264, 8265, 8266).

This library provides JavaScript-friendly bindings for validating and normalizing internationalized strings according to the PRECIS Framework specification.

## Supported Profiles

- **Nickname** (RFC 8266) - For display names and handles
- **UsernameCaseMapped** (RFC 8265) - For case-insensitive usernames
- **UsernameCasePreserved** (RFC 8265) - For case-sensitive usernames
- **OpaqueString** (RFC 8265) - For passwords and opaque identifiers

## Features

- ✅ **Multiple PRECIS Profiles**: Nickname, Username (mapped/preserved), and OpaqueString
- ✅ **RFC Compliant**: Full implementation of RFC 8264, 8265, and 8266
- ✅ **Unicode Normalization**: NFC normalization with proper case mapping
- ✅ **Dual API**: High-level ergonomic wrapper + low-level zero-copy WASM bindings
- ✅ **TypeScript Support**: Full type safety with `string → string` signatures
- ✅ **Easy Integration**: Simple `await init()` in browsers, automatic in Node.js
- ✅ **Small Bundle**: ~250 KB uncompressed (~60-80 KB gzipped)

## Installation

### NPM

```bash
npm install @precis/wasm
```

### Yarn

```bash
yarn add @precis/wasm
```

## Usage

### Browser (ES Modules)

```html
<!DOCTYPE html>
<html>
<head>
    <title>PRECIS Nickname Demo</title>
</head>
<body>
    <script type="module">
        // Import high-level wrapper with better types
        import { init, nickname_enforce, nickname_compare, version } from './pkg/precis.js';

        // Initialize WASM (required in browsers)
        await init();

        console.log('PRECIS WASM version:', version());

        // Enforce nickname
        try {
            const normalized = nickname_enforce("  Alice  ");
            console.log(normalized); // "Alice" (case preserved)
        } catch (error) {
            console.error('Invalid nickname:', error);
        }

        // Compare nicknames (case-insensitive)
        const match = nickname_compare("Alice", "alice");
        console.log('Nicknames match:', match); // true
    </script>
</body>
</html>
```

### Node.js (ES Modules)

```javascript
// Recommended: Use the high-level wrapper API
import { nickname_enforce, nickname_compare, nickname_prepare } from '@precis/wasm';

// Enforce nickname
try {
    const normalized = nickname_enforce("  Alice  ");
    console.log(normalized); // "Alice" (case preserved)
} catch (error) {
    console.error('Invalid nickname:', error);
}

// Compare nicknames (case-insensitive)
const match = nickname_compare("Alice", "ALICE");
console.log('Match:', match); // true
```

### Node.js (CommonJS)

```javascript
// For CommonJS, require the wrapper
const { nickname_enforce, nickname_compare } = require('@precis/wasm');

const normalized = nickname_enforce("  Alice  ");
console.log(normalized); // "Alice" (case preserved)
```

### TypeScript

```typescript
// Recommended: Use the high-level wrapper with proper types
import { nickname_enforce, nickname_compare, nickname_prepare } from '@precis/wasm';

// Type-safe API - string → string (no 'any' types!)
const normalized: string = nickname_enforce("Alice");
const isMatch: boolean = nickname_compare("Alice", "alice");
const prepared: string = nickname_prepare("Bob");
```

### Profile-Specific Examples

> **Note:** The following examples are for **Node.js/bundlers**. For **browsers**, you must call `await init()` before using any functions (see Browser example above).

#### Nicknames (RFC 8266)

```typescript
import { nickname_enforce, nickname_compare } from '@precis/wasm';

// Trims/collapses spaces, case-preserving enforcement
const nick = nickname_enforce("  Alice  ");
console.log(nick); // "Alice"

// Compare (case-insensitive)
console.log(nickname_compare("Alice", "alice")); // true
```

#### Usernames - Case Mapped (RFC 8265)

```typescript
import { usernamecasemapped_enforce, usernamecasemapped_compare } from '@precis/wasm';

// Case-insensitive username
const username = usernamecasemapped_enforce("Alice");
console.log(username); // "alice"

// Compare (case-insensitive)
console.log(usernamecasemapped_compare("Alice", "alice")); // true
```

#### Usernames - Case Preserved (RFC 8265)

```typescript
import { usernamecasepreserved_enforce, usernamecasepreserved_compare } from '@precis/wasm';

// Case-sensitive username
const username = usernamecasepreserved_enforce("Alice");
console.log(username); // "Alice"

// Compare (case-sensitive)
console.log(usernamecasepreserved_compare("Alice", "alice")); // false
```

#### Passwords / OpaqueString (RFC 8265)

```typescript
import { opaquestring_enforce, opaquestring_compare } from '@precis/wasm';

// Case-sensitive, preserves special characters
const password = opaquestring_enforce("MyP@ssw0rd!");
console.log(password); // "MyP@ssw0rd!"

// Compare (exact match)
console.log(opaquestring_compare("MyP@ssw0rd!", "MyP@ssw0rd!")); // true
console.log(opaquestring_compare("MyP@ssw0rd!", "myp@ssw0rd!")); // false
```

### Low-level API (Advanced)

For performance-critical scenarios where you need manual WASM initialization control:

```typescript
import init, { nickname_enforce, nickname_compare } from '@precis/wasm/precis_wasm.js';

// Must call init() before using WASM functions
await init();

// Direct WASM bindings - faster but requires manual initialization
const normalized = nickname_enforce("Alice");
const match = nickname_compare("Alice", "alice");
```

## Architecture & Design

This library provides **two API layers** to balance performance and developer experience:

### Layer 1: Raw WASM Bindings (Low-Level)

Generated by `wasm-pack` from Rust code using `wasm-bindgen`:

```rust
#[wasm_bindgen]
pub fn nickname_enforce(input: &str) -> Result<String, JsValue>
```

**Characteristics:**
- **Zero-copy semantics** - Direct Rust ↔ JavaScript string passing
- **Maximum performance** - No intermediate wrapper overhead
- **Manual initialization** - Requires `await init()` before use
- **TypeScript types are `any`** - wasm-bindgen doesn't preserve string types through `Result<T, E>`
- **Naming convention** - Rust snake_case: `nickname_enforce`, `nickname_compare`

### Layer 2: TypeScript Wrapper (High-Level)

Thin TypeScript wrapper (`precis.ts`) compiled to `precis.js`:

```typescript
export function nickname_enforce(input: string): string {
    return wasm.nickname_enforce(input);
}
```

**Characteristics:**
- **Correct TypeScript types** - `string → string` instead of `any`
- **Same naming convention** - Keeps `nickname_enforce`, `nickname_compare`, etc. for consistency with future profiles (username, password)
- **Negligible overhead** - Simple function delegation, no data transformation
- **Better developer experience** - Proper type checking in TypeScript
- **No manual initialization needed** - Works out of the box

### Design Philosophy

**Why preserve the raw WASM API?**
- Zero-copy performance for advanced users who need it
- Allows manual control over WASM initialization timing
- Maintains compatibility with wasm-bindgen conventions
- Some edge cases may benefit from direct access

**Why add the TypeScript wrapper?**
- 99% of users prefer ergonomics over microseconds
- Correct TypeScript types prevent runtime errors
- Eliminates `any` types from wasm-bindgen's auto-generated bindings
- Future-proof naming for multiple profiles (nickname, username, password)
- Easier onboarding for new users

### Which API Should You Use?

**Use the high-level wrapper** (`nickname_enforce`, `nickname_compare`, `nickname_prepare`) unless:
- You're processing millions of strings per second and every microsecond counts
- You need precise control over when WASM initializes
- You're integrating with existing code that uses wasm-bindgen patterns

For typical web applications, the high-level API is recommended. The performance difference is negligible (< 0.1µs per call), and the ergonomic benefits are substantial.

## Building from Source

### Prerequisites

- Rust 1.80+
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- Node.js (for TypeScript compilation)

### Build

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Node.js dependencies
npm install

# Build everything (WASM + TypeScript wrapper)
npm run build

# This script:
# 1. Compiles Rust → WASM for web target (pkg/)
# 2. Compiles Rust → WASM for Node.js target (pkg-node/)
# 3. Compiles TypeScript wrapper (src/precis.ts → precis.js)
# 4. Updates package.json entry points to use the wrapper
```

**Manual builds** (if you need specific targets):

```bash
# Build only raw WASM for web
npm run build:raw

# Build only raw WASM for Node.js
npm run build:nodejs

# Build for bundlers (webpack, rollup, etc.)
npm run build:bundler
```

### Test

```bash
# Run tests in headless browser
wasm-pack test --headless --firefox

# Run tests in Chrome
wasm-pack test --headless --chrome
```

## Examples

See the [examples](examples/) directory for complete working examples:

- [`examples/web/`](examples/web/) - Interactive HTML/JavaScript browser demo
- [`examples/typescript/`](examples/typescript/) - TypeScript/Node.js example with all profiles

## Performance

- **Bundle Size**: ~250 KB uncompressed (~60-80 KB gzipped)
- **Initialization**: < 10ms on modern browsers
- **Operations**: ~1-5µs per operation (depends on string length)

## Browser Support

- ✅ Chrome 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Edge 16+
- ✅ Node.js 12+

All modern browsers with WebAssembly support.

## RFC Compliance

This library implements:
- [RFC 8266](https://www.rfc-editor.org/rfc/rfc8266.html): Preparation, Enforcement, and Comparison of Internationalized Strings Representing Nicknames
- [RFC 8264](https://www.rfc-editor.org/rfc/rfc8264.html): PRECIS Framework: Preparation, Enforcement, and Comparison of Internationalized Strings in Application Protocols
- [RFC 5893](https://www.rfc-editor.org/rfc/rfc5893.html): Right-to-Left Scripts for Internationalized Domain Names for Applications (IDNA)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

## Related Crates

- [`precis-core`](../precis-core) - Core PRECIS Framework implementation
- [`precis-profiles`](../precis-profiles) - PRECIS profiles (Username, Password, Nickname)

## Links

- [GitHub Repository](https://github.com/sancane/precis)
- [Documentation](https://docs.rs/precis-wasm)
- [NPM Package](https://www.npmjs.com/package/@precis/wasm)
