#!/usr/bin/env bash
# Build the explorer as static files in web/crates/app/dist (see README.md).
#   web/build.sh              production build (LTO, wasm-opt)
#   PUBLIC_URL=/repo/ web/build.sh   when the site is served from a sub-path
set -euo pipefail
cd "$(dirname "$0")"

python3 ../db/tools/webexport.py                       # the dataset, from db/dump/
(cd bridge && npm ci --no-audit --no-fund && npm run build)
(cd crates/app && trunk build --release --cargo-profile dist ${PUBLIC_URL:+--public-url "$PUBLIC_URL"})

# The worker's wasm is loaded by kbworker_loader.js (no integrity hash), so it can be optimised after the build.
wasm-opt -Oz --strip-debug --strip-producers --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext --enable-mutable-globals \
  --enable-reference-types crates/app/dist/kbworker_bg.wasm -o crates/app/dist/kbworker_bg.wasm

du -sh crates/app/dist
ls -la crates/app/dist | grep -E "wasm|js$"
