#!/bin/bash
# KVM Pro Server Launcher with Auto-Update
# This script checks for updates and then runs the server

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GITHUB_REPO="${GITHUB_REPO:-yourusername/kvm-pro}"

# Run auto-update check
echo "Checking for updates..."
if [ -f "$SCRIPT_DIR/auto-update.sh" ]; then
    GITHUB_REPO="$GITHUB_REPO" bash "$SCRIPT_DIR/auto-update.sh" "$SCRIPT_DIR" || true
fi

# Run the server
exec "$SCRIPT_DIR/kvm-pro-server" "$@"
