#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TOOLCHAIN_DIR="$PROJECT_ROOT/cross-toolchain/armv7l-linux-musleabihf-cross"
export PATH="$TOOLCHAIN_DIR/bin:$PATH"

echo "[*] Building dashboard backend..."
(cd "$PROJECT_ROOT/dashboard" && CARGO_TARGET_ARMV7L_UNKNOWN_LINUX_MUSLEABIHF_LINKER=armv7l-linux-musleabihf-gcc cargo build --target armv7l-unknown-linux-musleabihf --release)

echo "[*] Build complete! Binary: dashboard/target/armv7l-unknown-linux-musleabihf/release/aurora-dashboard"
