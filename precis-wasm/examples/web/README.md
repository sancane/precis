# Web Example - PRECIS Nickname Demo

Interactive web demo for the PRECIS Nickname profile (RFC 8266) using WebAssembly.

## Features

- ✅ **Nickname Enforcement**: Normalize and validate nicknames according to RFC 8266
- ✅ **Nickname Preparation**: Prepare nicknames for comparison without strict validation
- ✅ **Nickname Comparison**: Case-insensitive comparison of nicknames
- ✅ **Real-time Validation**: Instant feedback as you type
- ✅ **Pure Browser**: No bundler needed, uses ES modules directly
- ✅ **TypeScript Types**: Full type safety with the wrapper API

## Quick Start

Since browsers don't allow loading ES modules from `file://` protocol due to CORS, you need to serve the files through HTTP.

### Option 1: Python HTTP Server (Recommended)

```bash
# From the precis-wasm directory
python3 -m http.server 8000
```

Then open: http://localhost:8000/examples/web/

### Option 2: Node.js - npx serve

```bash
# From the precis-wasm directory
npx serve .
```

Then open the URL shown (usually http://localhost:3000/examples/web/)

### Option 3: Node.js - http-server

```bash
# From the precis-wasm directory
npx http-server -p 8000
```

Then open: http://localhost:8000/examples/web/

## Prerequisites

Make sure you've built the WASM package first:

```bash
cd precis-wasm
npm install
npm run build
```

This generates the `pkg/` directory with the compiled WASM module and TypeScript wrapper.

## How It Works

The demo uses ES modules to load the PRECIS WASM library directly in the browser:

```javascript
// Import the high-level wrapper with proper TypeScript types
import { init, nickname_enforce, nickname_compare, version } from '../../pkg/precis.js';

// Initialize WASM (required in browsers)
await init();

// Use the functions
const normalized = nickname_enforce("  Alice  ");
console.log(normalized); // "Alice"
```

### Key Points

1. **`init()` is required**: In browsers, you must call `await init()` before using any functions
2. **ES Modules**: Uses `<script type="module">` for native ES module support
3. **High-level API**: Uses the TypeScript wrapper for better ergonomics and correct types
4. **Zero-copy**: The WASM bindings use zero-copy string passing for performance

## Features Demonstrated

### 1. Enforce
Validates and normalizes nicknames according to RFC 8266:
- Trims leading/trailing spaces
- Collapses multiple spaces
- Preserves case (comparison is case-insensitive)
- Unicode normalization (NFC)

### 2. Prepare
Prepares nicknames for comparison without strict validation. Useful when you need to normalize a string that may already be valid.

### 3. Compare
Compares two nicknames for equality after normalization. Case-insensitive comparison.

## Project Structure

```
web/
├── index.html         # Complete demo with UI
└── README.md          # This file
```

The demo imports from:
```
../../pkg/
├── precis.js          # TypeScript wrapper (recommended)
├── precis.d.ts        # TypeScript definitions
├── precis_wasm.js     # Low-level WASM bindings
├── precis_wasm_bg.wasm # Compiled WebAssembly binary
└── package.json       # Package metadata
```

## Architecture

The demo uses a **two-layer API**:

### Layer 1: High-Level Wrapper (Recommended)
```javascript
import { init, nickname_enforce, nickname_compare } from '../../pkg/precis.js';
```

- ✅ Correct TypeScript types (`string → string`)
- ✅ Same API as Node.js examples
- ✅ Clean function names
- ⚠️ Requires `await init()` in browsers

### Layer 2: Low-Level WASM Bindings (Advanced)
```javascript
import init, { nickname_enforce, nickname_compare } from '../../pkg/precis_wasm.js';
```

- ⚠️ Types are `any` (wasm-bindgen limitation)
- ⚠️ Requires manual `await init()`
- ✅ Zero-copy performance
- ✅ Direct WASM access

## Example Usage in Your Project

### For Static Sites

Copy the files from `pkg/` to your project and import:

```html
<script type="module">
  import { init, nickname_enforce } from './pkg/precis.js';

  await init();
  const result = nickname_enforce("Alice");
</script>
```

### For Published Package (npm/CDN)

Once published, use a CDN:

```html
<script type="module">
  import { init, nickname_enforce }
    from 'https://unpkg.com/@precis/wasm@0.1.0/precis.js';

  await init();
  const result = nickname_enforce("Alice");
</script>
```

## Browser Compatibility

- ✅ Chrome 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Edge 16+

All modern browsers with WebAssembly and ES modules support.

## Related Examples

- [TypeScript Example](../typescript/) - Node.js with TypeScript
- [Main README](../../README.md) - Full documentation

## Troubleshooting

### CORS Error

```
Access to script at 'file://...' has been blocked by CORS policy
```

**Solution**: Serve the files through HTTP (see Quick Start above). Modern browsers block ES modules from `file://` for security.

### Module Not Found

```
GET http://localhost:8000/pkg/precis.js 404 (Not Found)
```

**Solution**: Make sure you're serving from the `precis-wasm` directory (not from `examples/web/`), so the relative path `../../pkg/` resolves correctly.

### WASM Initialization Error

```
Failed to initialize WASM: TypeError: Cannot read properties of undefined
```

**Solution**: Make sure you've run `npm run build` in the `precis-wasm` directory to generate the `pkg/` folder.
