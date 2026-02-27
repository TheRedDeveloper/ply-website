#!/bin/bash
set -e

# Minimal JS bundle for the interactive demo.
# Only includes the core miniquad GL loader + ply_demo plugin.
# No audio, net, sapp-jsutils, or accessibility.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
STATIC_DEMO="$SCRIPT_DIR/../static/demo"

mkdir -p "$SCRIPT_DIR/.build"

# Core miniquad GL loader
curl -s https://raw.githubusercontent.com/TheRedDeveloper/miniquad-fix/refs/heads/main/js/gl.js \
    -o "$SCRIPT_DIR/.build/gl.js"

function wrap_js {
    echo "(function () {" >> "$SCRIPT_DIR/.build/bundle.js"
    cat "$1" >> "$SCRIPT_DIR/.build/bundle.js"
    echo "}());" >> "$SCRIPT_DIR/.build/bundle.js"
}

cat "$SCRIPT_DIR/.build/gl.js" > "$SCRIPT_DIR/.build/bundle.js"
wrap_js "$STATIC_DEMO/ply_demo.js"

npx minify@9.2.0 "$SCRIPT_DIR/.build/bundle.js" > "$STATIC_DEMO/ply_bundle.js"

rm -rf "$SCRIPT_DIR/.build"

echo "Bundle: $(du -h "$STATIC_DEMO/ply_bundle.js" | cut -f1)"
