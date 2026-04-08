#!/bin/bash
VERSION="1.0.0"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="$SCRIPT_DIR/dist-portable"

mkdir -p "$DIST_DIR"

# LINUX PORTABLE
LINUX_APPIMAGE="$SCRIPT_DIR/frontend/target/release/bundle/appimage/ScreenLink_${VERSION}_amd64.AppImage"
if [ -f "$LINUX_APPIMAGE" ]; then
    echo "Linux Portable..."
    LINUX_DIR="$DIST_DIR/ScreenLink-${VERSION}-linux-portable"
    rm -rf "$LINUX_DIR"
    mkdir -p "$LINUX_DIR"
    
    cp "$LINUX_APPIMAGE" "$LINUX_DIR/ScreenLink"
    chmod +x "$LINUX_DIR/ScreenLink"
    
    echo "#!/bin/bash" > "$LINUX_DIR/run.sh"
    echo 'DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"' >> "$LINUX_DIR/run.sh"
    echo 'exec "$DIR/ScreenLink" "$@"' >> "$LINUX_DIR/run.sh"
    chmod +x "$LINUX_DIR/run.sh"
    
    echo "ScreenLink - Portable Linux" > "$LINUX_DIR/README.txt"
    echo "" >> "$LINUX_DIR/README.txt"
    echo "Run: ./run.sh or ./ScreenLink" >> "$LINUX_DIR/README.txt"
    
    cd "$DIST_DIR"
    zip -r "ScreenLink-${VERSION}-linux-portable.zip" "ScreenLink-${VERSION}-linux-portable/"
    cd "$SCRIPT_DIR"
    echo "OK: linux-portable.zip"
else
    echo "Missing: $LINUX_APPIMAGE"
fi

# MACOS PORTABLE
MACOS_APP="$SCRIPT_DIR/frontend/target/release/bundle/macos/ScreenLink.app"
if [ -d "$MACOS_APP" ]; then
    echo "Apple macOS Portable..."
    MACOS_DIR="$DIST_DIR/ScreenLink-${VERSION}-macos-portable"
    rm -rf "$MACOS_DIR"
    mkdir -p "$MACOS_DIR"
    
    cp -r "$MACOS_APP" "$MACOS_DIR/ScreenLink.app"
    
    echo "#!/bin/bash" > "$MACOS_DIR/run.sh"
    echo 'DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"' >> "$MACOS_DIR/run.sh"
    echo 'open -a "$DIR/ScreenLink.app"' >> "$MACOS_DIR/run.sh"
    chmod +x "$MACOS_DIR/run.sh"
    
    echo "ScreenLink - Portable macOS" > "$MACOS_DIR/README.txt"
    echo "" >> "$MACOS_DIR/README.txt"
    echo "Run: ./run.sh or double-click ScreenLink.app" >> "$MACOS_DIR/README.txt"
    
    cd "$DIST_DIR"
    zip -r "ScreenLink-${VERSION}-macos-portable.zip" "ScreenLink-${VERSION}-macos-portable/"
    cd "$SCRIPT_DIR"
    echo "OK: macos-portable.zip"
else
    echo "Missing: $MACOS_APP"
fi

# WINDOWS PORTABLE
WINDOWS_EXE="$SCRIPT_DIR/frontend/target/release/ScreenLink.exe"
if [ -f "$WINDOWS_EXE" ]; then
    echo "Windows Portable..."
    WINDOWS_DIR="$DIST_DIR/ScreenLink-${VERSION}-windows-portable"
    rm -rf "$WINDOWS_DIR"
    mkdir -p "$WINDOWS_DIR"
    
    cp "$WINDOWS_EXE" "$WINDOWS_DIR/ScreenLink.exe"
    
    echo "@echo off" > "$WINDOWS_DIR/run.bat"
    echo "%~dp0ScreenLink.exe %*" >> "$WINDOWS_DIR/run.bat"
    
    echo '$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path' > "$WINDOWS_DIR/run.ps1"
    echo '& "$scriptPath\ScreenLink.exe"' >> "$WINDOWS_DIR/run.ps1"
    
    echo "ScreenLink - Portable Windows" > "$WINDOWS_DIR/README.txt"
    echo "" >> "$WINDOWS_DIR/README.txt"
    echo "Run: ScreenLink.exe or run.bat or run.ps1" >> "$WINDOWS_DIR/README.txt"
    
    cd "$DIST_DIR"
    zip -r "ScreenLink-${VERSION}-windows-portable.zip" "ScreenLink-${VERSION}-windows-portable/"
    cd "$SCRIPT_DIR"
    echo "OK: windows-portable.zip"
else
    echo "Missing: $WINDOWS_EXE"
fi
fi

# ============================================================================
# SUMMARY
# ============================================================================

echo ""
echo "✨ Versões Portáveis Criadas:"
echo ""
ls -lh "$DIST_DIR"/*.zip 2>/dev/null || echo "   (Nenhuma versão portável gerada - verifique os executáveis compilados)"
echo ""
echo "📍 Localização: $DIST_DIR/"
echo ""
echo "✅ Pronto para download!"
