#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
STATIC_DEMO="$SCRIPT_DIR/../static/demo"

mkdir -p "$STATIC_DEMO"

# Install CodeMirror deps if not present
if [ ! -d "$SCRIPT_DIR/node_modules" ]; then
    echo "Installing CodeMirror dependencies..."
    cd "$SCRIPT_DIR" && npm install --silent
fi

# Bundle into a single IIFE file
echo "Bundling CodeMirror..."
npx esbuild "$SCRIPT_DIR/codemirror-setup.js" \
    --bundle \
    --format=iife \
    --global-name=CM \
    --minify \
    --outfile="$STATIC_DEMO/codemirror-bundle.js"

echo "CodeMirror: $(du -h "$STATIC_DEMO/codemirror-bundle.js" | cut -f1)"
