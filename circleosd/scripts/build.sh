#!/usr/bin/env bash
set -e

echo "[CircleOSD] 🔧 Building all components..."
cd "$(dirname "$0")/.."

# Build workspace
cargo build --workspace --release

# Copy binaries into build/release
mkdir -p build/release
for bin in core-daemon auth-service plugin-manager circlectl; do
    cp "target/release/$bin" "build/release/" 2>/dev/null || true
done

echo "[CircleOSD] ✅ Build complete! Binaries available in build/release/"
