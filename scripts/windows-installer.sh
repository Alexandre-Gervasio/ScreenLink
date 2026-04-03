#!/bin/bash
# One-Click Windows Portable Installer Creator
# For Windows users who want to create a nice installer

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION=$(grep '^version' "$PROJECT_DIR/core/Cargo.toml" | head -1 | cut -d'"' -f2)

cat << 'EOF'
╔════════════════════════════════════════════════════════════╗
║   Windows Portable Package Creator (NSIS Installer)       ║
║              For Advanced Windows Users                   ║
╚════════════════════════════════════════════════════════════╝
EOF

echo ""
echo "This script creates a Windows installer using NSIS."
echo ""

# Check if NSIS is installed
if ! command -v makensis &> /dev/null; then
    echo "⚠️  NSIS is not installed."
    echo ""
    echo "To create Windows installers:"
    echo "1. Install NSIS from: https://nsis.sourceforge.io/"
    echo "2. Run this script again"
    echo ""
    echo "For now, using portable ZIP instead..."
    bash "$PROJECT_DIR/scripts/windows-build.sh"
    exit 0
fi

echo "✅ NSIS found"
echo ""

# Create temporary NSIS installer script
NSIS_SCRIPT="$PROJECT_DIR/core/target/kvm-pro-installer.nsi"

mkdir -p "$(dirname "$NSIS_SCRIPT")"

cat > "$NSIS_SCRIPT" << 'NSIS_EOF'
; KVM Pro Installer Script (NSIS)
; Portable application - no system dependencies

!include "MUI2.nsh"

; Basic settings
Name "KVM Pro"
OutFile "..\dist\kvm-pro-installer-${VERSION}.exe"
InstallDir "$PROGRAMFILES\KVM Pro"
RequestExecutionLevel user

; UI Settings
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

; Installation section
Section "Install"
    SetOutPath "$INSTDIR"
    
    ; Copy binaries
    File "kvm-pro-server.exe"
    File "kvm-pro-client.exe"
    
    ; Copy docs
    File "..\..\kvm-pro.toml"
    File "..\..\README.md"
    File "..\..\LICENSE"
    
    ; Create shortcuts
    CreateDirectory "$SMPROGRAMS\KVM Pro"
    CreateShortcut "$SMPROGRAMS\KVM Pro\Server.lnk" "$INSTDIR\kvm-pro-server.exe"
    CreateShortcut "$SMPROGRAMS\KVM Pro\Client.lnk" "$INSTDIR\kvm-pro-client.exe"
    CreateShortcut "$SMPROGRAMS\KVM Pro\Uninstall.lnk" "$INSTDIR\uninstall.exe"
    
    ; Create uninstaller
    WriteUninstaller "$INSTDIR\uninstall.exe"
SectionEnd

; Uninstall section
Section "Uninstall"
    Delete "$INSTDIR\kvm-pro-server.exe"
    Delete "$INSTDIR\kvm-pro-client.exe"
    RMDir "$INSTDIR"
    Delete "$SMPROGRAMS\KVM Pro\*"
    RMDir "$SMPROGRAMS\KVM Pro"
SectionEnd
NSIS_EOF

echo "Creating Windows installer..."
echo "This may take a moment..."

# Replace VERSION in script
sed -i "s/\${VERSION}/$VERSION/g" "$NSIS_SCRIPT"

# Build first
"$PROJECT_DIR/scripts/windows-build.sh"

# Run NSIS
makensis "$NSIS_SCRIPT"

echo ""
echo "✅ Windows installer created!"
echo "   File: $PROJECT_DIR/dist/kvm-pro-installer-$VERSION.exe"
echo ""
echo "Users can now:"
echo "1. Download kvm-pro-installer-$VERSION.exe"
echo "2. Click to install"
echo "3. Start from Start Menu"
echo ""
