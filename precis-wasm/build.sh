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
# Publish a clean manifest: strip the dev-only `scripts` and `devDependencies`
# (they have no meaning for consumers and would make `npm publish ./pkg` try to
# run a `prepublishOnly` whose build script is not shipped).
node -e '
  const p = require("./package.json");
  delete p.scripts;
  delete p.devDependencies;
  require("fs").writeFileSync("'"$OUT"'/package.json", JSON.stringify(p, null, 2) + "\n");
'
[ -f README.md ] && cp README.md "$OUT"/README.md
# Ship the license text inside the package (the crate itself has none, so
# fall back to the workspace-root LICENSE).
if [ -f LICENSE ]; then
    cp LICENSE "$OUT"/LICENSE
elif [ -f ../LICENSE ]; then
    cp ../LICENSE "$OUT"/LICENSE
fi

echo "✅ Build complete!"
echo ""
echo "📦 Publishable package in ./$OUT (run: npm publish ./$OUT)"
echo "  - Browser/bundler entry : precis.js  (import { init, ... }; await init())"
echo "  - Node.js entry         : precis-node.js  (auto-initialized)"
echo "  - Types                 : precis.d.ts"
