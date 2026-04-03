#!/bin/bash
# Master Release Build Script for KVM Pro
# Creates all portable distributions

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION=$(grep '^version' "$PROJECT_DIR/core/Cargo.toml" | head -1 | cut -d'"' -f2)

clear
cat << "EOF"
╔════════════════════════════════════════════════════════════╗
║                KVM Pro Release Builder                    ║
║          Creating Portable Distributions v0.1.0           ║
╚════════════════════════════════════════════════════════════╝
EOF

echo ""
echo "Project Version: $VERSION"
echo "Build Date: $(date)"
echo ""

# Create dist directory
mkdir -p "$PROJECT_DIR/dist"

echo "What would you like to build?"
echo ""
echo "1. Linux portable (tar.gz + AppImage)"
echo "2. Windows portable (ZIP)"
echo "3. Both (Linux + Windows)"
echo "4. Full release packages"
echo ""
read -p "Enter your choice (1-4): " choice

case $choice in
    1)
        echo ""
        echo "Building Linux packages..."
        bash "$PROJECT_DIR/scripts/portable-build.sh" linux
        
        if command -v appimagetool &> /dev/null; then
            echo ""
            bash "$PROJECT_DIR/scripts/appimage-build.sh"
        fi
        ;;
    2)
        echo ""
        echo "Building Windows package..."
        bash "$PROJECT_DIR/scripts/windows-build.sh"
        ;;
    3)
        echo ""
        echo "Building all packages..."
        bash "$PROJECT_DIR/scripts/portable-build.sh" linux
        bash "$PROJECT_DIR/scripts/portable-build.sh" windows
        
        if command -v appimagetool &> /dev/null; then
            bash "$PROJECT_DIR/scripts/appimage-build.sh"
        fi
        ;;
    4)
        echo ""
        echo "Creating full release..."
        bash "$PROJECT_DIR/scripts/portable-build.sh" linux
        bash "$PROJECT_DIR/scripts/portable-build.sh" windows
        if command -v appimagetool &> /dev/null; then
            bash "$PROJECT_DIR/scripts/appimage-build.sh"
        fi
        
        # Create releases summary
        cat > "$PROJECT_DIR/dist/RELEASES.txt" << RELEASES
KVM Pro Release $VERSION - $(date)

FILES:
======

Linux:
  kvm-pro-linux-x64-portable-$VERSION.tar.gz
    - Fully static portable binary
    - Extract and run
    - No dependencies

  KVM_Pro-$VERSION-x86_64.AppImage (if created)
    - Universal Linux package
    - Click to run
    - Works on all Linux distributions

Windows:
  kvm-pro-windows-x64-portable-$VERSION.zip
    - Portable ZIP package
    - Extract and run
    - No installation required
    - Works on Windows 10/11

INSTALLATION:
==============

Linux:
  tar xzf kvm-pro-linux-x64-portable-*.tar.gz
  cd kvm-pro-linux-*
  ./run-server.sh

Windows:
  Extract ZIP file
  Double-click run-server.bat

FEATURES:
==========
✓ No installation required
✓ No dependencies
✓ Fully portable
✓ Works on any compatible system
✓ Can run from USB drive

SYSTEM REQUIREMENTS:
====================

Linux:
  - Any x86_64 Linux (glibc or musl)
  - No additional libraries needed

Windows:
  - Windows 10 or Windows 11
  - x86_64 architecture
  - 100 MB free disk space

RELEASES
        
        echo ""
        echo "✅ Full release package prepared!"
        cat "$PROJECT_DIR/dist/RELEASES.txt"
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "════════════════════════════════════════════════════════════"
echo ""
echo "📦 Build Complete!"
echo ""
echo "Packages available in: $PROJECT_DIR/dist/"
echo ""
ls -lh "$PROJECT_DIR/dist/"
echo ""
echo "════════════════════════════════════════════════════════════"
