#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOST="${KOBO_HOST:-192.168.2.1}"
PORT="${KOBO_PORT:-22}"
USER="${KOBO_USER:-root}"

case "${1:-}" in
    logs)
        ssh -p "$PORT" "$USER@$HOST" "tail -f /var/log/aurora/*.log"
        ;;
    shell)
        ssh -p "$PORT" "$USER@$HOST"
        ;;
    restart)
        ssh -p "$PORT" "$USER@$HOST" "killall aurora-dashboard; /mnt/onboard/.aurora/aurora-dashboard &"
        ;;
    *)
        echo "Usage: $0 {logs|shell|restart}"
        exit 1
        ;;
esac
