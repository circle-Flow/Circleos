#!/usr/bin/env bash
set -e

echo "[CircleOSD] ⚙️ Installing system binaries..."

PREFIX="/usr/local/bin"
ROOT_DIR="$(dirname "$0")/.."

# Build first if not built
if [[ ! -d "$ROOT_DIR/build/release" ]]; then
    bash "$ROOT_DIR/scripts/build.sh"
fi

sudo mkdir -p "$PREFIX"
sudo cp "$ROOT_DIR/build/release/"* "$PREFIX"

echo "[CircleOSD] ✅ Installed binaries to $PREFIX"
echo "[CircleOSD] Try running: circlectl system info"
