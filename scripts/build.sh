#!/bin/bash
# Build script for KVM Pro
# Supports: Linux and Windows cross-compilation

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR/core"

echo "=== KVM Pro Build Script ==="
echo "Project directory: $PROJECT_DIR"

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    TARGETS="x86_64-unknown-linux-gnu"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    OS="windows"
    TARGETS="x86_64-pc-windows-gnu"
else
    OS="unknown"
    TARGETS="unknown"
fi

echo "Detected OS: $OS"

# Build for current platform
echo "Building for current platform..."
cargo build --release

# Optional: Build for Linux if on Linux
if [[ "$OS" == "linux" ]]; then
    echo "Building native Linux binaries..."
    cargo build --release --target x86_64-unknown-linux-gnu
fi

# Optional: Windows cross-compile (if tools available)
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Building for Windows (cross-compile)..."
    cargo build --release --target x86_64-pc-windows-gnu
else
    echo "Windows cross-compile tools not found. Skipping Windows build."
fi

echo "=== Build Complete ==="
echo "Output directory: $PROJECT_DIR/core/target/release/"
ls -lh "$PROJECT_DIR/core/target/release/kvm-pro-server" 2>/dev/null || true
ls -lh "$PROJECT_DIR/core/target/release/kvm-pro-client" 2>/dev/null || true
