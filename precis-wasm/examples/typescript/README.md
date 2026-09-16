# TypeScript Example

This example demonstrates how to use `precis-wasm` in a TypeScript project with full type safety.

## Features Demonstrated

- ✅ All 4 PRECIS profiles: Nickname, OpaqueString, UsernameCaseMapped, UsernameCasePreserved
- ✅ Full TypeScript type safety (`string → string` instead of `any`)
- ✅ Proper error handling with TypeScript
- ✅ Type inference and compile-time checking

## Prerequisites

Make sure you've built the WASM package first:

```bash
cd ../..
npm install
npm run build
```

## Installation

Install dependencies:

```bash
npm install
```

## Running the Example

### Option 1: Run directly with tsx (fastest for development)

```bash
npm run dev
```

### Option 2: Compile and run

```bash
npm run build  # Compiles TypeScript to JavaScript
npm start      # Runs the compiled JavaScript
```

### Option 3: Run compiled JavaScript directly

```bash
npm run build
node precis.js
```

## Expected Output

You should see output demonstrating all PRECIS profiles:

```
============================================================
PRECIS WASM TypeScript Example
Version: 0.1.0
============================================================

📝 Nickname Profile (RFC 8266)
------------------------------------------------------------
✓ nickname_enforce("  Alice  ") = "Alice"
✓ nickname_enforce("BOB") = "BOB"
✓ nickname_enforce("José") = "José"
✓ nickname_prepare("Alice") = "Alice"
✓ nickname_compare("Alice", "alice") = true
✓ nickname_compare("Alice", "Bob") = false

🔒 OpaqueString Profile (RFC 8265) - Passwords
------------------------------------------------------------
✓ opaquestring_enforce("MyP@ssw0rd!") = "MyP@ssw0rd!"
...

✅ All examples completed successfully!
```

## TypeScript Benefits

This example shows the benefits of using TypeScript with precis-wasm:

1. **Type Safety**: All functions have proper `string → string` types
2. **IntelliSense**: Your IDE will provide autocomplete and inline documentation
3. **Compile-time Errors**: Catch mistakes before runtime
4. **Better Refactoring**: Rename variables safely across the codebase

## Project Structure

```
typescript/
├── precis.ts          # Main example code (TypeScript)
├── precis.js          # Compiled JavaScript (after npm run build)
├── precis.d.ts        # Type definitions (after npm run build)
├── package.json       # Dependencies and scripts
├── tsconfig.json      # TypeScript configuration
└── README.md          # This file
```

Compiled files (`precis.js`, `precis.d.ts`, `*.map`) are ignored by git.

## Using in Your Own Project

To use precis-wasm in your TypeScript project:

```bash
npm install precis-wasm
```

Then import and use (after publishing to npm):

```typescript
import { nickname_enforce, nickname_compare } from 'precis-wasm';

// Full type safety - TypeScript knows these are string → string
const normalized: string = nickname_enforce("  Alice  ");
const match: boolean = nickname_compare("Alice", "alice");
```

**Note:** This example uses dynamic imports to work with the local build. Once the package is published to npm, you can use standard ES module imports as shown above.

## Related Examples

- [Web Example](../web/) - Browser usage with ES modules
- [Main README](../../README.md) - Full documentation
