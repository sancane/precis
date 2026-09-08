#!/bin/bash
# Build script for precis-wasm.
#
# Produces a single publishable package in ./pkg that works in browsers,
# bundlers and Node.js via conditional `exports`:
#   - browser/bundler  -> precis.js   (ESM wrapper over the --target web glue)
#   - Node.js          -> precis-node.js (CJS wrapper over the --target nodejs glue)
#
# The published package.json is the committed one (copied verbatim), so there is
# no fragile post-processing of wasm-pack's generated manifest.

set -euo pipefail

OUT=pkg

echo "🧹 Cleaning previous build..."
rm -rf "$OUT" pkg-node

echo "🔨 Building WASM (web target)..."
wasm-pack build --target web --out-dir "$OUT" --out-name precis_web

echo "🔨 Building WASM (Node.js target)..."
wasm-pack build --target nodejs --out-dir pkg-node --out-name precis_node

echo "📋 Merging Node.js target artifacts into $OUT..."
cp pkg-node/precis_node.js \
   pkg-node/precis_node.d.ts \
   pkg-node/precis_node_bg.wasm \
   pkg-node/precis_node_bg.wasm.d.ts \
   "$OUT"/
rm -rf pkg-node

echo "📦 Installing TypeScript (if needed)..."
if ! [ -x "node_modules/.bin/tsc" ]; then
    npm install --silent
fi

echo "📝 Compiling TypeScript wrappers..."
npx tsc -p tsconfig.web.json    # -> pkg/precis.js       (ESM, with async init)
npx tsc -p tsconfig.node.json   # -> pkg/precis-node.js  (CJS, auto-initialized)

echo "🔧 Writing published package manifest..."
cp package.json "$OUT"/package.json
[ -f README.md ] && cp README.md "$OUT"/README.md

echo "✅ Build complete!"
echo ""
echo "📦 Publishable package in ./$OUT (run: npm publish ./$OUT)"
echo "  - Browser/bundler entry : precis.js  (import { init, ... }; await init())"
echo "  - Node.js entry         : precis-node.js  (auto-initialized)"
echo "  - Types                 : precis.d.ts"
