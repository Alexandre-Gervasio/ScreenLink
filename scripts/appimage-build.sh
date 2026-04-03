#!/bin/bash
# Create AppImage for Linux (universal, no installation needed)
# Requires: appimagetool or linuxdeployqt

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION=$(grep '^version' "$PROJECT_DIR/core/Cargo.toml" | head -1 | cut -d'"' -f2)
BUILD_DIR="$PROJECT_DIR/dist"
APPIMAGE_DIR="$BUILD_DIR/KVM_Pro-$VERSION.AppDir"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║    Creating KVM Pro AppImage for Linux - v$VERSION       ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# First, build portable Linux binary
echo "Building portable Linux binary..."
"$PROJECT_DIR/scripts/portable-build.sh" linux

SERVER_BIN="$PROJECT_DIR/core/target/x86_64-unknown-linux-musl/portable/kvm-pro-server"
CLIENT_BIN="$PROJECT_DIR/core/target/x86_64-unknown-linux-musl/portable/kvm-pro-client"

if [ ! -f "$SERVER_BIN" ] || [ ! -f "$CLIENT_BIN" ]; then
    echo "Error: Binaries not found after build"
    exit 1
fi

# Create AppDir structure
mkdir -p "$APPIMAGE_DIR/usr/bin"
mkdir -p "$APPIMAGE_DIR/usr/share/applications"
mkdir -p "$APPIMAGE_DIR/usr/share/icons"

echo "Creating AppImage structure..."

# Copy binaries
cp "$SERVER_BIN" "$APPIMAGE_DIR/usr/bin/kvm-pro-server"
cp "$CLIENT_BIN" "$APPIMAGE_DIR/usr/bin/kvm-pro-client"

# Create desktop files
cat > "$APPIMAGE_DIR/usr/share/applications/kvm-pro-server.desktop" << 'EOF'
[Desktop Entry]
Type=Application
Name=KVM Pro Server
Comment=KVM Server - Share your input devices
Exec=kvm-pro-server
Icon=kvm-pro
Categories=System;Utility;
Terminal=true
EOF

cat > "$APPIMAGE_DIR/usr/share/applications/kvm-pro-client.desktop" << 'EOF'
[Desktop Entry]
Type=Application
Name=KVM Pro Client
Comment=KVM Client - Use remote input devices
Exec=kvm-pro-client
Icon=kvm-pro
Categories=System;Utility;
Terminal=true
EOF

# Create AppRun script
cat > "$APPIMAGE_DIR/AppRun" << 'EOF'
#!/bin/bash
APPDIR="$(cd "$(dirname "$0")" && pwd)"
export PATH="$APPDIR/usr/bin:$PATH"
exec "$APPDIR/usr/bin/kvm-pro-server" "$@"
EOF
chmod +x "$APPIMAGE_DIR/AppRun"

# Create metadata
cat > "$APPIMAGE_DIR/metadata.xml" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<component type="console-application">
  <id>io.github.kvm-pro.server</id>
  <name>KVM Pro Server</name>
  <summary>High-performance KVM software</summary>
  <description>
    <p>KVM Pro is a superior alternative to Input Leap and Barrier.</p>
    <p>Control multiple computers from a single mouse and keyboard.</p>
  </description>
  <version>$VERSION</version>
  <url type="homepage">https://github.com/your-repo/kvm-pro</url>
  <license>MIT</license>
  <permissions/>
</component>
EOF

# Check if appimagetool is available
if command -v appimagetool &> /dev/null; then
    echo "Creating AppImage using appimagetool..."
    cd "$BUILD_DIR"
    appimagetool "$APPIMAGE_DIR" "KVM_Pro-$VERSION-x86_64.AppImage"
    
    echo ""
    echo "✅ AppImage created!"
    APPIMAGE="$BUILD_DIR/KVM_Pro-$VERSION-x86_64.AppImage"
    SIZE=$(du -h "$APPIMAGE" | cut -f1)
    
    chmod +x "$APPIMAGE"
    
    echo "  File: $(basename "$APPIMAGE")"
    echo "  Size: $SIZE"
    echo ""
    echo "📦 Universal Linux Package Ready!"
    echo "   Just make it executable and run:"
    echo "   $ chmod +x KVM_Pro-$VERSION-x86_64.AppImage"
    echo "   $ ./KVM_Pro-$VERSION-x86_64.AppImage"
    echo ""
    echo "   NO INSTALLATION NEEDED!"
    
else
    echo "⚠️  appimagetool not found, skipping AppImage creation"
    echo "   Install with: sudo apt install appimagetool"
    echo ""
    echo "Using portable package instead:"
    ls -lh "$BUILD_DIR/kvm-pro-linux-x64-portable-$VERSION.tar.gz"
fi

echo ""
echo "════════════════════════════════════════════════════════════"
