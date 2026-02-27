#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
STATIC_DEMO="$SCRIPT_DIR/../static/demo"
PROFILE="release"

if [ "$1" = "--debug" ]; then
    PROFILE="dev"
    echo "Building in DEBUG mode"
fi

mkdir -p "$STATIC_DEMO"

# 1. Build the JS bundle
echo "Building JS bundle..."
bash "$SCRIPT_DIR/buildbundle.sh"

# 2. Build CodeMirror bundle (if not already built)
if [ ! -f "$STATIC_DEMO/codemirror-bundle.js" ]; then
    echo "Building CodeMirror bundle..."
    bash "$SCRIPT_DIR/build-codemirror.sh"
fi

# 3. Copy font assets (macroquad loads relative to the page URL)
echo "Copying assets..."
mkdir -p "$STATIC_DEMO/assets/fonts"
cp "$SCRIPT_DIR/assets/fonts/lexend.ttf" "$STATIC_DEMO/assets/fonts/lexend.ttf"

# 4. Build WASM
echo "Building WASM ($PROFILE)..."
if [ "$PROFILE" = "dev" ]; then
    cargo build --target wasm32-unknown-unknown \
        --manifest-path "$SCRIPT_DIR/Cargo.toml"
    WASM_IN="$SCRIPT_DIR/target/wasm32-unknown-unknown/debug/interactive-examples.wasm"
else
    cargo build --release --target wasm32-unknown-unknown \
        --manifest-path "$SCRIPT_DIR/Cargo.toml"
    WASM_IN="$SCRIPT_DIR/target/wasm32-unknown-unknown/release/interactive-examples.wasm"
fi

WASM_OUT="$STATIC_DEMO/app-v1.wasm"

# 5. Optimize with wasm-opt (release only)
if [ "$PROFILE" = "release" ] && command -v wasm-opt &> /dev/null; then
    echo "Optimizing with wasm-opt..."
    wasm-opt -Os --all-features "$WASM_IN" -o "$WASM_OUT"
else
    cp "$WASM_IN" "$WASM_OUT"
fi

echo "WASM: $(du -h "$WASM_OUT" | cut -f1)"
echo "Done."
