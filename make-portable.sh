#!/bin/bash

# Script to create portable versions of ScreenLink
# No admin required - just extract and run!

set -e

VERSION="1.0.0"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="$SCRIPT_DIR/dist-portable"

echo "Creating portable versions of ScreenLink v${VERSION}..."
mkdir -p "$DIST_DIR"

# ============================================================================
# LINUX PORTABLE
# ============================================================================

if [ -f "$SCRIPT_DIR/frontend/target/release/bundle/appimage/ScreenLink_${VERSION}_amd64.AppImage" ]; then
    echo ""
    echo "Linux Portable..."
    
    LINUX_DIR="$DIST_DIR/ScreenLink-${VERSION}-linux-portable"
    mkdir -p "$LINUX_DIR"
    
    # Copiar AppImage
    cp "$SCRIPT_DIR/frontend/target/release/bundle/appimage/ScreenLink_${VERSION}_amd64.AppImage" \
       "$LINUX_DIR/ScreenLink"
    chmod +x "$LINUX_DIR/ScreenLink"
    
    # Criar script de execução simples
    cat > "$LINUX_DIR/run.sh" << 'EOF'
#!/bin/bash
# ScreenLink - Portable Version for Linux
# No admin required - just run this script!

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$DIR/ScreenLink" "$@"
EOF
    chmod +x "$LINUX_DIR/run.sh"
    
    # Criar README
    cat > "$LINUX_DIR/README.txt" << 'EOF'
ScreenLink - Portable Version (Linux)
=====================================

NO INSTALLATION REQUIRED!

To run:
  ./run.sh
  
Or directly:
  ./ScreenLink
  
That's it! No admin permissions needed.

Troubleshooting:
- If it doesn't run, try: chmod +x ./ScreenLink && ./ScreenLink
- Some systems may need additional libraries (run.sh handles this)

Enjoy!
EOF
    
    # Criar ZIP
    cd "$DIST_DIR"
    zip -r "ScreenLink-${VERSION}-linux-portable.zip" "ScreenLink-${VERSION}-linux-portable/" > /dev/null
    cd "$SCRIPT_DIR"
    
    echo "OK: ScreenLink-${VERSION}-linux-portable.zip"
fi

# ============================================================================
# MACOS PORTABLE (se disponível)
# ============================================================================

if [ -d "$SCRIPT_DIR/frontend/target/release/bundle/macos/ScreenLink.app" ]; then
    echo ""
    echo "Apple macOS Portable..."
    
    MACOS_DIR="$DIST_DIR/ScreenLink-${VERSION}-macos-portable"
    mkdir -p "$MACOS_DIR"
    
    # Copiar app bundle
    cp -r "$SCRIPT_DIR/frontend/target/release/bundle/macos/ScreenLink.app" "$MACOS_DIR/ScreenLink.app"
    
    # Criar script de execução
    cat > "$MACOS_DIR/run.sh" << 'EOF'
#!/bin/bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
open -a "$DIR/ScreenLink.app"
EOF
    chmod +x "$MACOS_DIR/run.sh"
    
    # Criar README
    cat > "$MACOS_DIR/README.txt" << 'EOF'
ScreenLink - Portable Version (macOS)
====================================

NO INSTALLATION REQUIRED!

To run:
  Double-click ScreenLink.app
  Or: ./run.sh

No admin permissions needed - just run!

Enjoy!
EOF
    
    # Criar ZIP
    cd "$DIST_DIR"
    zip -r "ScreenLink-${VERSION}-macos-portable.zip" "ScreenLink-${VERSION}-macos-portable/" > /dev/null
    cd "$SCRIPT_DIR"
    
    echo "OK: ScreenLink-${VERSION}-macos-portable.zip"
fi

# ============================================================================
# WINDOWS PORTABLE (manual - executável extraído do NSIS/MSI)
# ============================================================================

if [ -f "$SCRIPT_DIR/frontend/target/release/ScreenLink.exe" ]; then
    echo ""
    echo "Windows Portable..."
    
    WINDOWS_DIR="$DIST_DIR/ScreenLink-${VERSION}-windows-portable"
    mkdir -p "$WINDOWS_DIR"
    
    # Copiar executável
    cp "$SCRIPT_DIR/frontend/target/release/ScreenLink.exe" "$WINDOWS_DIR/ScreenLink.exe"
    
    # Criar batch script
    cat > "$WINDOWS_DIR/run.bat" << 'EOF'
@echo off
REM ScreenLink - Portable Version for Windows
REM NO INSTALLATION REQUIRED!
REM Just double-click this file or run: ScreenLink.exe

"%~dp0ScreenLink.exe" %*
EOF
    
    # Criar PowerShell script
    cat > "$WINDOWS_DIR/run.ps1" << 'EOF'
# ScreenLink - Portable Version for Windows
# NO INSTALLATION REQUIRED!

$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
& "$scriptPath\ScreenLink.exe"
EOF
    
    # Criar README
    cat > "$WINDOWS_DIR/README.txt" << 'EOF'
ScreenLink - Portable Version (Windows)
======================================

NO INSTALLATION REQUIRED!

To run:
  1. Double-click ScreenLink.exe
  2. Or double-click run.bat
  3. Or run: ScreenLink.exe

No admin permissions needed!

Troubleshooting:
  If Windows shows "Windows protected your PC":
    1. Click "More info"
    2. Click "Run anyway"
  
  This is normal for unsigned executables.

Enjoy!
EOF
    
    # Criar ZIP
    cd "$DIST_DIR"
    zip -r "ScreenLink-${VERSION}-windows-portable.zip" "ScreenLink-${VERSION}-windows-portable/" > /dev/null
    cd "$SCRIPT_DIR"
    
    echo "OK: ScreenLink-${VERSION}-windows-portable.zip"
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
