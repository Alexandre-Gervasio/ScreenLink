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
    
    # Copy AppImage directly (not as "ScreenLink" without extension)
    cp "$LINUX_APPIMAGE" "$LINUX_DIR/ScreenLink.AppImage"
    chmod +x "$LINUX_DIR/ScreenLink.AppImage"
    
    # Copy backend server
    if [ -d "$SCRIPT_DIR/backend" ]; then
        cp -r "$SCRIPT_DIR/backend" "$LINUX_DIR/"
        echo "✓ Backend included"
    fi
    
    # Create a wrapper script that starts backend and app
    echo "#!/bin/bash" > "$LINUX_DIR/run.sh"
    echo 'DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"' >> "$LINUX_DIR/run.sh"
    echo 'APPIMAGE="$DIR/ScreenLink.AppImage"' >> "$LINUX_DIR/run.sh"
    echo '' >> "$LINUX_DIR/run.sh"
    echo '# Start backend server in background if not already running' >> "$LINUX_DIR/run.sh"
    echo 'if [ -d "$DIR/backend" ] && ! nc -z localhost 3001 2>/dev/null; then' >> "$LINUX_DIR/run.sh"
    echo '    cd "$DIR/backend"' >> "$LINUX_DIR/run.sh"
    echo '    npm install --silent 2>/dev/null' >> "$LINUX_DIR/run.sh"
    echo '    npm run start > /dev/null 2>&1 &' >> "$LINUX_DIR/run.sh"
    echo '    sleep 1  # Give server time to start' >> "$LINUX_DIR/run.sh"
    echo '    cd "$DIR"' >> "$LINUX_DIR/run.sh"
    echo 'fi' >> "$LINUX_DIR/run.sh"
    echo '' >> "$LINUX_DIR/run.sh"
    echo '# Ensure executable permission' >> "$LINUX_DIR/run.sh"
    echo 'chmod +x "$APPIMAGE" 2>/dev/null' >> "$LINUX_DIR/run.sh"
    echo '' >> "$LINUX_DIR/run.sh"
    echo '# Run AppImage with FUSE fallback support' >> "$LINUX_DIR/run.sh"
    echo 'export APPIMAGE_EXTRACT_AND_RUN=1  # For systems without FUSE' >> "$LINUX_DIR/run.sh"
    echo 'exec "$APPIMAGE" "$@"' >> "$LINUX_DIR/run.sh"
    chmod +x "$LINUX_DIR/run.sh"
    
    # Create a simple instruction file with permission instructions
    echo "LEIA PRIMEIRO - READ FIRST" > "$LINUX_DIR/LEIA-ME.txt"
    echo "" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "=== EXECUTOR ===" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "Para executar:" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "  ./run.sh" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "Ou diretamente:" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "  chmod +x ScreenLink.AppImage && ./ScreenLink.AppImage" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "=== TROUBLESHOOTING ===" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "If AppImage won't open:" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "  chmod +x ScreenLink.AppImage run.sh" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "If error about FUSE (fusion filesystem):" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "  export APPIMAGE_EXTRACT_AND_RUN=1" >> "$LINUX_DIR/LEIA-ME.txt"
    echo "  ./ScreenLink.AppImage" >> "$LINUX_DIR/LEIA-ME.txt"
    
    # Better README with permission instructions
    echo "ScreenLink - Portable Linux" > "$LINUX_DIR/README.txt"
    echo "" >> "$LINUX_DIR/README.txt"
    echo "=== Como usar ===" >> "$LINUX_DIR/README.txt"
    echo "" >> "$LINUX_DIR/README.txt"
    echo "Opção 1 (Recomendado - Automático):" >> "$LINUX_DIR/README.txt"
    echo "  ./run.sh" >> "$LINUX_DIR/README.txt"
    echo "" >> "$LINUX_DIR/README.txt"
    echo "This starts the backend server automatically!" >> "$LINUX_DIR/README.txt"
    
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
    
    # Copy backend server
    if [ -d "$SCRIPT_DIR/backend" ]; then
        cp -r "$SCRIPT_DIR/backend" "$MACOS_DIR/"
        echo "✓ Backend included"
    fi
    
    # Create run script that starts backend and app
    echo "#!/bin/bash" > "$MACOS_DIR/run.sh"
    echo 'DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"' >> "$MACOS_DIR/run.sh"
    echo '' >> "$MACOS_DIR/run.sh"
    echo '# Start backend server in background if not already running' >> "$MACOS_DIR/run.sh"
    echo 'if [ -d "$DIR/backend" ] && ! nc -z localhost 3001 2>/dev/null; then' >> "$MACOS_DIR/run.sh"
    echo '    cd "$DIR/backend"' >> "$MACOS_DIR/run.sh"
    echo '    npm install --silent 2>/dev/null' >> "$MACOS_DIR/run.sh"
    echo '    npm run start > /dev/null 2>&1 &' >> "$MACOS_DIR/run.sh"
    echo '    sleep 1' >> "$MACOS_DIR/run.sh"
    echo '    cd "$DIR"' >> "$MACOS_DIR/run.sh"
    echo 'fi' >> "$MACOS_DIR/run.sh"
    echo '' >> "$MACOS_DIR/run.sh"
    echo 'open -a "$DIR/ScreenLink.app"' >> "$MACOS_DIR/run.sh"
    chmod +x "$MACOS_DIR/run.sh"
    
    echo "ScreenLink - Portable macOS" > "$MACOS_DIR/README.txt"
    echo "" >> "$MACOS_DIR/README.txt"
    echo "Run: ./run.sh or double-click ScreenLink.app" >> "$MACOS_DIR/README.txt"
    echo "" >> "$MACOS_DIR/README.txt"
    echo "The backend server will start automatically!" >> "$MACOS_DIR/README.txt"
    
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
