#!/bin/bash
# Package script for KVM Pro
# Creates distributable packages

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PACKAGE_DIR="$PROJECT_DIR/packages"
BUILD_DIR="$PROJECT_DIR/core/target/release"

mkdir -p "$PACKAGE_DIR"

echo "=== KVM Pro Packaging Script ==="
echo "Package directory: $PACKAGE_DIR"

# Check if binaries exist
if [ ! -f "$BUILD_DIR/kvm-pro-server" ] || [ ! -f "$BUILD_DIR/kvm-pro-client" ]; then
    echo "Error: Binaries not found. Run build.sh first."
    exit 1
fi

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    SUFFIX="linux-x64"
    ARCHIVE_FORMAT="tar.gz"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    OS="windows"
    SUFFIX="windows-x64"
    ARCHIVE_FORMAT="zip"
else
    OS="unknown"
    SUFFIX="unknown"
    ARCHIVE_FORMAT="tar"
fi

# Create package directory
PACKAGE_NAME="kvm-pro-$SUFFIX"
PACKAGE_PATH="$PACKAGE_DIR/$PACKAGE_NAME"
mkdir -p "$PACKAGE_PATH"

echo "Packaging for $OS..."

# Copy binaries
cp "$BUILD_DIR/kvm-pro-server" "$PACKAGE_PATH/" || cp "$BUILD_DIR/kvm-pro-server.exe" "$PACKAGE_PATH/" 2>/dev/null || true
cp "$BUILD_DIR/kvm-pro-client" "$PACKAGE_PATH/" || cp "$BUILD_DIR/kvm-pro-client.exe" "$PACKAGE_PATH/" 2>/dev/null || true

# Copy configuration
cp "$PROJECT_DIR/kvm-pro.toml" "$PACKAGE_PATH/kvm-pro.toml.example" 2>/dev/null || true

# Copy README
cp "$PROJECT_DIR/README.md" "$PACKAGE_PATH/README.md" 2>/dev/null || true

# Copy license
cp "$PROJECT_DIR/LICENSE" "$PACKAGE_PATH/LICENSE" 2>/dev/null || echo "LICENSE file not found"

# Create archive
if [[ "$ARCHIVE_FORMAT" == "tar.gz" ]]; then
    cd "$PACKAGE_DIR"
    tar czf "$PACKAGE_NAME.tar.gz" "$PACKAGE_NAME"
    echo "Package created: $PACKAGE_DIR/$PACKAGE_NAME.tar.gz"
elif [[ "$ARCHIVE_FORMAT" == "zip" ]]; then
    cd "$PACKAGE_DIR"
    zip -r "$PACKAGE_NAME.zip" "$PACKAGE_NAME"
    echo "Package created: $PACKAGE_DIR/$PACKAGE_NAME.zip"
fi

echo "=== Packaging Complete ==="
