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

The demo loads the compiled package from `../../pkg/`, so there are two things
to get right before it will work: the package must be **built**, and the server
must be started from the **`precis-wasm/` directory** (not from `examples/web/`)
so that `../../pkg/` is reachable.

### Step 1: Build the WASM package

From the `precis-wasm` directory:

```bash
cd precis-wasm
npm install
npm run build
```

This generates the `pkg/` directory with the compiled WASM module and the
TypeScript wrapper (`pkg/precis.js`). The demo imports `../../pkg/precis.js`, so
**this step is required** — without it you get a `404 (Not Found)` for
`precis.js` and the page stays stuck on "Loading PRECIS WASM module...".

### Step 2: Serve over HTTP from the `precis-wasm` directory

Browsers don't allow loading ES modules from the `file://` protocol (CORS), so
serve the files through HTTP. Run one of these **from `precis-wasm/`** (its
parent must contain `pkg/`):

```bash
# Option 1 (recommended): Python
python3 -m http.server 8000

# Option 2: npx serve
npx serve .

# Option 3: npx http-server
npx http-server -p 8000
```

### Step 3: Open the demo

Open **http://localhost:8000/examples/web/** (adjust the port for the server you
picked). If you started the server from inside `examples/web/` instead, the
browser resolves `../../pkg/precis.js` to `/pkg/precis.js`, which the server
can't reach — so it 404s. Serve from `precis-wasm/`.

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
├── precis_web.js     # Low-level WASM bindings
├── precis_web_bg.wasm # Compiled WebAssembly binary
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
import init, { nickname_enforce, nickname_compare } from '../../pkg/precis_web.js';
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
    from 'https://unpkg.com/precis-wasm@0.1.0/precis.js';

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
