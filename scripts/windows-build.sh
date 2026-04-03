#!/bin/bash
# Windows Installer Builder for KVM Pro
# Creates a portable ZIP or NSIS installer

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION=$(grep '^version' "$PROJECT_DIR/core/Cargo.toml" | head -1 | cut -d'"' -f2)
BUILD_DIR="$PROJECT_DIR/dist"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║   Building Windows Portable Package - KVM Pro v$VERSION   ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Build portable Windows binary
echo "Building portable Windows binary..."
"$PROJECT_DIR/scripts/portable-build.sh" windows

PACKAGE_NAME="kvm-pro-windows-x64-portable-$VERSION"
PACKAGE_DIR="$BUILD_DIR/$PACKAGE_NAME"

if [ ! -d "$PACKAGE_DIR" ]; then
    echo "Error: Package not found"
    exit 1
fi

# Create additional Windows-specific files

# Create a PowerShell launch script (more user-friendly)
cat > "$PACKAGE_DIR/launch-server.ps1" << 'EOF'
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
& "$scriptDir\kvm-pro-server.exe"
EOF

cat > "$PACKAGE_DIR/launch-client.ps1" << 'EOF'
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
& "$scriptDir\kvm-pro-client.exe"
EOF

# Create shortcuts (requires Windows-specific tools, but document it)
cat > "$PACKAGE_DIR/CREATE_SHORTCUTS.bat" << 'EOF'
@echo off
REM This script creates desktop shortcuts for easy access

setlocal enabledelayedexpansion

REM Get the directory where this script is located
set SCRIPT_DIR=%~dp0

REM Use PowerShell to create shortcuts
powershell -ExecutionPolicy Bypass -Command ^
    "$ws = New-Object -ComObject WScript.Shell;" ^
    "$shortcut = $ws.CreateShortcut('%USERPROFILE%\Desktop\KVM Pro Server.lnk');" ^
    "$shortcut.TargetPath = '%SCRIPT_DIR%kvm-pro-server.exe';" ^
    "$shortcut.WorkingDirectory = '%SCRIPT_DIR%';" ^
    "$shortcut.Description = 'KVM Pro Server - Share your input devices';" ^
    "$shortcut.Save();" ^
    "$shortcut2 = $ws.CreateShortcut('%USERPROFILE%\Desktop\KVM Pro Client.lnk');" ^
    "$shortcut2.TargetPath = '%SCRIPT_DIR%kvm-pro-client.exe';" ^
    "$shortcut2.WorkingDirectory = '%SCRIPT_DIR%';" ^
    "$shortcut2.Description = 'KVM Pro Client - Use remote input devices';" ^
    "$shortcut2.Save();"

echo Shortcuts created on Desktop!
pause
EOF

# Create unzip and run script
cat > "$PACKAGE_DIR/FIRST_RUN.bat" << 'EOF'
@echo off
echo.
echo Welcome to KVM Pro!
echo.
echo This is a portable application - no installation needed!
echo.
echo Choose what you want to do:
echo.
echo 1. Run Server (share your input devices)
echo 2. Run Client (use remote input devices)
echo 3. Create Desktop Shortcuts
echo.
set /p choice="Enter your choice (1-3): "

if "%choice%"=="1" (
    kvm-pro-server.exe
) else if "%choice%"=="2" (
    kvm-pro-client.exe
) else if "%choice%"=="3" (
    call CREATE_SHORTCUTS.bat
) else (
    echo Invalid choice
    pause
)
EOF

# Create comprehensive README for Windows
cat > "$PACKAGE_DIR/README_WINDOWS.txt" << 'EOF'
KVM Pro - Windows Portable Edition

FEATURES:
- No installation required
- No administrator privileges needed
- Portable - can run from USB drive
- Works on Windows 10/11 (x64)

QUICK START:
1. Extract this ZIP file to any folder
2. Double-click FIRST_RUN.bat
3. Choose Server or Client
4. That's it!

CONFIGURATION:
Edit kvm-pro.toml.example for advanced settings

MANUAL RUN:
- run-server.bat    - Start as server
- run-client.bat    - Start as client

TROUBLESHOOTING:
- If run-server.bat doesn't work:
  1. Open Command Prompt (cmd.exe)
  2. Navigate to this folder
  3. Type: kvm-pro-server.exe

- If ports are already in use:
  Edit kvm-pro.toml and change the port number

FOLDER STRUCTURE:
- kvm-pro-server.exe     - Server executable
- kvm-pro-client.exe     - Client executable
- kvm-pro.toml.example   - Configuration template
- run-server.bat         - Easy server launcher
- run-client.bat         - Easy client launcher
- FIRST_RUN.bat          - Interactive launcher
- CREATE_SHORTCUTS.bat   - Create desktop shortcuts

For more information, see README.txt

EOF

# Verify package contents
echo "Package contents:"
ls -lh "$PACKAGE_DIR"

echo ""
echo "════════════════════════════════════════════════════════════"
echo ""
echo "✅ Windows Portable Package Created!"
echo ""
echo "📦 Location: $BUILD_DIR/$PACKAGE_NAME.zip"
echo ""
echo "📋 What to do next:"
echo "1. Share/distribute: $PACKAGE_NAME.zip"
echo "2. Users extract the ZIP"
echo "3. Users double-click FIRST_RUN.bat"
echo "4. Done - no installation needed!"
echo ""
echo "════════════════════════════════════════════════════════════"
