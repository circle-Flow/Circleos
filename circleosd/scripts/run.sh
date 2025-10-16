#!/usr/bin/env bash
set -e

echo "[CircleOSD] 🚀 Starting system..."

ROOT_DIR="$(dirname "$0")/.."
cd "$ROOT_DIR"

# Ensure directories exist
mkdir -p var/log var/run var/sessions

# Start core-daemon (supervises services)
if [[ ! -f build/release/core-daemon ]]; then
    echo "Core-daemon not built. Run scripts/build.sh first."
    exit 1
fi

LOGFILE="var/log/circleosd.log"
./build/release/core-daemon >> "$LOGFILE" 2>&1 &

sleep 1
echo "[CircleOSD] 🟢 Core daemon started. Logs -> $LOGFILE"
echo "[CircleOSD] Use 'circlectl system status' to check system health."
