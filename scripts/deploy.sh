#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BINARY="$PROJECT_ROOT/dashboard/target/armv7l-unknown-linux-musleabihf/release/aurora-dashboard"
HOST="${KOBO_HOST:-192.168.2.1}"
PORT="${KOBO_PORT:-22}"
USER="${KOBO_USER:-root}"
REMOTE_DIR="/mnt/onboard/.aurora"

if [[ ! -f "$BINARY" ]]; then
    echo "[!] Binary not found. Run ./scripts/build.sh first."
    exit 1
fi

echo "[*] Creating remote directory..."
ssh -p "$PORT" "$USER@$HOST" "mkdir -p $REMOTE_DIR"

echo "[*] Deploying binary..."
scp -P "$PORT" "$BINARY" "$USER@$HOST:$REMOTE_DIR/aurora-dashboard"

echo "[*] Deploying web assets..."
scp -P "$PORT" -r "$PROJECT_ROOT/dashboard/web/"* "$USER@$HOST:$REMOTE_DIR/web/"

echo "[✓] Deployed! Start with: ssh $USER@$HOST '$REMOTE_DIR/aurora-dashboard'"
