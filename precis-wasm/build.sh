#!/bin/bash
# Build script for precis-wasm with TypeScript wrapper

set -e

echo "🔨 Building WASM for web target..."
wasm-pack build --target web --out-dir pkg

echo "🔨 Building WASM for Node.js target..."
wasm-pack build --target nodejs --out-dir pkg-node

echo "📝 Compiling TypeScript wrapper..."
npx tsc

echo "📋 Copying wrapper to pkg-node..."
cp pkg/precis.js pkg-node/
cp pkg/precis.d.ts pkg-node/

echo "🔧 Updating package.json entry points..."
# Update pkg/package.json (web target)
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' 's/"main": "precis_wasm.js"/"main": "precis.js"/' pkg/package.json
    sed -i '' 's/"types": "precis_wasm.d.ts"/"types": "precis.d.ts"/' pkg/package.json
    sed -i '' 's/"precis_wasm_bg.wasm",/"precis_wasm_bg.wasm",\n    "precis.js",\n    "precis.d.ts",/' pkg/package.json

    # Update pkg-node/package.json (nodejs target)
    sed -i '' 's/"main": "precis_wasm.js"/"main": "precis.js"/' pkg-node/package.json
    sed -i '' 's/"types": "precis_wasm.d.ts"/"types": "precis.d.ts"/' pkg-node/package.json
    sed -i '' 's/"precis_wasm_bg.wasm",/"precis_wasm_bg.wasm",\n    "precis.js",\n    "precis.d.ts",/' pkg-node/package.json
else
    # Linux
    sed -i 's/"main": "precis_wasm.js"/"main": "precis.js"/' pkg/package.json
    sed -i 's/"types": "precis_wasm.d.ts"/"types": "precis.d.ts"/' pkg/package.json
    sed -i 's/"precis_wasm_bg.wasm",/"precis_wasm_bg.wasm",\n    "precis.js",\n    "precis.d.ts",/' pkg/package.json

    # Update pkg-node/package.json (nodejs target)
    sed -i 's/"main": "precis_wasm.js"/"main": "precis.js"/' pkg-node/package.json
    sed -i 's/"types": "precis_wasm.d.ts"/"types": "precis.d.ts"/' pkg-node/package.json
    sed -i 's/"precis_wasm_bg.wasm",/"precis_wasm_bg.wasm",\n    "precis.js",\n    "precis.d.ts",/' pkg-node/package.json
fi

echo "✅ Build complete!"
echo ""
echo "📦 Web target (pkg/):"
echo "  - JavaScript: pkg/precis.js (wrapper with correct types)"
echo "  - TypeScript: pkg/precis.d.ts (string → string)"
echo ""
echo "📦 Node.js target (pkg-node/):"
echo "  - JavaScript: pkg-node/precis.js (wrapper with correct types)"
echo "  - TypeScript: pkg-node/precis.d.ts (string → string)"
echo ""
echo "Low-level WASM bindings (advanced use):"
echo "  - pkg/precis_wasm.js & pkg-node/precis_wasm.js"
echo "  - Binary: pkg/precis_wasm_bg.wasm"
