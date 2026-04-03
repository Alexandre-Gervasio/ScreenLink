#!/bin/bash
# Portable Build Script for KVM Pro
# Creates fully static, dependency-free binaries
# Usage: ./scripts/portable-build.sh [linux|windows]

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$PROJECT_DIR/core/target"
DIST_DIR="$PROJECT_DIR/dist"
cd "$PROJECT_DIR/core"

TARGET="${1:-linux}"
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)

echo "╔════════════════════════════════════════════════════════════╗"
echo "║       KVM Pro Portable Build - Version $VERSION               ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

mkdir -p "$DIST_DIR"

case "$TARGET" in
    linux)
        echo "🐧 Building for Linux (musl - fully static)..."
        
        # Check if musl target is installed
        if ! rustup target list | grep -q "x86_64-unknown-linux-musl (installed)"; then
            echo "  Installing musl target..."
            rustup target add x86_64-unknown-linux-musl
        fi
        
        echo "  Building..."
        RUSTFLAGS="-C target-feature=+crt-static" cargo build \
            --release \
            --target x86_64-unknown-linux-musl \
            --profile portable 2>&1 | tail -20
        
        SERVER_BIN="$BUILD_DIR/x86_64-unknown-linux-musl/portable/kvm-pro-server"
        CLIENT_BIN="$BUILD_DIR/x86_64-unknown-linux-musl/portable/kvm-pro-client"
        ARCH="linux-x64"
        ;;
        
    windows)
        echo "🪟 Building for Windows (MinGW - fully static)..."
        
        # Check if windows target is installed
        if ! rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
            echo "  Installing Windows target..."
            rustup target add x86_64-pc-windows-gnu
        fi
        
        echo "  Building..."
        cargo build \
            --release \
            --target x86_64-pc-windows-gnu \
            --profile portable 2>&1 | tail -20
        
        SERVER_BIN="$BUILD_DIR/x86_64-pc-windows-gnu/portable/kvm-pro-server.exe"
        CLIENT_BIN="$BUILD_DIR/x86_64-pc-windows-gnu/portable/kvm-pro-client.exe"
        ARCH="windows-x64"
        ;;
        
    *)
        echo "Unknown target: $TARGET"
        echo "Usage: $0 [linux|windows]"
        exit 1
        ;;
esac

echo ""
echo "✅ Build successful!"
echo ""

# Create portable package
PACKAGE_NAME="kvm-pro-$ARCH-portable-$VERSION"
PACKAGE_DIR="$DIST_DIR/$PACKAGE_NAME"

mkdir -p "$PACKAGE_DIR"

echo "📦 Creating portable package..."
echo "  Package: $PACKAGE_NAME"

# Copy binaries
if [ "$TARGET" = "linux" ]; then
    cp "$SERVER_BIN" "$PACKAGE_DIR/kvm-pro-server"
    cp "$CLIENT_BIN" "$PACKAGE_DIR/kvm-pro-client"
    chmod +x "$PACKAGE_DIR/kvm-pro-server"
    chmod +x "$PACKAGE_DIR/kvm-pro-client"
    
    # Create launcher script
    cat > "$PACKAGE_DIR/run-server.sh" << 'LAUNCHER'
#!/bin/bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$DIR/kvm-pro-server" "$@"
LAUNCHER
    chmod +x "$PACKAGE_DIR/run-server.sh"
    
    cat > "$PACKAGE_DIR/run-client.sh" << 'LAUNCHER'
#!/bin/bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$DIR/kvm-pro-client" "$@"
LAUNCHER
    chmod +x "$PACKAGE_DIR/run-client.sh"
    
elif [ "$TARGET" = "windows" ]; then
    cp "$SERVER_BIN" "$PACKAGE_DIR/kvm-pro-server.exe"
    cp "$CLIENT_BIN" "$PACKAGE_DIR/kvm-pro-client.exe"
    
    # Create launcher batch files
    cat > "$PACKAGE_DIR/run-server.bat" << 'LAUNCHER'
@echo off
cd /d "%~dp0"
kvm-pro-server.exe %*
LAUNCHER
    
    cat > "$PACKAGE_DIR/run-client.bat" << 'LAUNCHER'
@echo off
cd /d "%~dp0"
kvm-pro-client.exe %*
LAUNCHER
fi

# Copy common files
cp "$PROJECT_DIR/README.md" "$PACKAGE_DIR/README.txt" 2>/dev/null || true
cp "$PROJECT_DIR/kvm-pro.toml" "$PACKAGE_DIR/kvm-pro.toml.example" 2>/dev/null || true
cp "$PROJECT_DIR/LICENSE" "$PACKAGE_DIR/LICENSE" 2>/dev/null || true

# Create documentation
cat > "$PACKAGE_DIR/QUICK_START.txt" << 'EOF'
KVM Pro - Portable Version

Quick Start:
============

LINUX:
  Terminal 1 (Server):
    ./run-server.sh

  Terminal 2 (Client):
    ./run-client.sh

WINDOWS:
  Command Prompt 1 (Server):
    run-server.bat

  Command Prompt 2 (Client):
    run-client.bat

Configuration:
===============
Edit kvm-pro.toml.example and rename to kvm-pro.toml

Help:
======
See README.txt for full documentation

No installation required - just run!
EOF

# Get binary sizes
SERVER_SIZE=$(du -h "$PACKAGE_DIR/kvm-pro-server" 2>/dev/null | cut -f1)
CLIENT_SIZE=$(du -h "$PACKAGE_DIR/kvm-pro-client" 2>/dev/null | cut -f1)

echo ""
echo "════════════════════════════════════════════════════════════"
echo ""
echo "📊 Package Information:"
echo "  Location: $DIST_DIR/$PACKAGE_NAME/"
echo "  Server binary size: $SERVER_SIZE"
echo "  Client binary size: $CLIENT_SIZE"
echo ""

# Create archive
cd "$DIST_DIR"

if [ "$TARGET" = "linux" ]; then
    echo "📦 Creating archive..."
    tar czf "$PACKAGE_NAME.tar.gz" "$PACKAGE_NAME"
    ARCHIVE_SIZE=$(du -h "$PACKAGE_NAME.tar.gz" | cut -f1)
    echo "  Archive: $PACKAGE_NAME.tar.gz ($ARCHIVE_SIZE)"
    echo ""
    echo "✅ Ready to distribute!"
    echo "   Just extract and run - NO INSTALLATION NEEDED!"
    echo ""
    echo "Usage:"
    echo "  tar xzf $PACKAGE_NAME.tar.gz"
    echo "  cd $PACKAGE_NAME"
    echo "  ./run-server.sh"
    
elif [ "$TARGET" = "windows" ]; then
    echo "📦 Creating archive..."
    zip -q -r "$PACKAGE_NAME.zip" "$PACKAGE_NAME"
    ARCHIVE_SIZE=$(du -h "$PACKAGE_NAME.zip" | cut -f1)
    echo "  Archive: $PACKAGE_NAME.zip ($ARCHIVE_SIZE)"
    echo ""
    echo "✅ Ready to distribute!"
    echo "   Just extract and run - NO INSTALLATION NEEDED!"
    echo ""
    echo "Usage:"
    echo "  Extract $PACKAGE_NAME.zip"
    echo "  Double-click run-server.bat or run-client.bat"
fi

echo ""
echo "════════════════════════════════════════════════════════════"
