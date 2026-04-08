#!/bin/bash
# ScreenLink - Portable Version for Linux
# No admin required - just run this script!

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$DIR/ScreenLink" "$@"
