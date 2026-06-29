#!/usr/bin/env bash
set -euo pipefail

# Download and extract the ARM cross-compiler toolchain for Kobo Clara BW.
# Uses the armhf (armv7l) musl-based toolchain by default.

TOOLCHAIN_DIR="$(cd "$(dirname "$0")/.." && pwd)/cross-toolchain"
TOOLCHAIN_URL="${TOOLCHAIN_URL:-https://musl.cc/armv7l-linux-musleabihf-cross.tgz}"
TOOLCHAIN_FILE="armv7l-linux-musleabihf-cross.tgz"

if [[ -d "$TOOLCHAIN_DIR/armv7l-linux-musleabihf-cross" ]]; then
    echo "[✓] Toolchain already exists at $TOOLCHAIN_DIR"
    exit 0
fi

echo "[*] Downloading toolchain..."
mkdir -p "$TOOLCHAIN_DIR"
wget -q "$TOOLCHAIN_URL" -O "$TOOLCHAIN_DIR/$TOOLCHAIN_FILE"

echo "[*] Extracting..."
tar xf "$TOOLCHAIN_DIR/$TOOLCHAIN_FILE" -C "$TOOLCHAIN_DIR"

echo "[*] Cleaning up..."
rm "$TOOLCHAIN_DIR/$TOOLCHAIN_FILE"

echo "[✓] Toolchain installed at $TOOLCHAIN_DIR/armv7l-linux-musleabihf-cross"
