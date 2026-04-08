# Minimal Windows portable creator - no complex strings, no arrays
$VERSION = "1.0.0"
$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$DIST_DIR = Join-Path $SCRIPT_DIR "dist-portable"
$WIN_PORTABLE = Join-Path $SCRIPT_DIR "frontend/target/release/ScreenLink.exe"

# Create dist dir if needed
if (-not (Test-Path $DIST_DIR)) {
    New-Item -ItemType Directory -Path $DIST_DIR | Out-Null
}

# Only process if executable exists
if (Test-Path $WIN_PORTABLE) {
    $WINDOWS_DIR = Join-Path $DIST_DIR "ScreenLink-$VERSION-windows-portable"
    
    # Clean and create directory
    if (Test-Path $WINDOWS_DIR) {
        Remove-Item $WINDOWS_DIR -Recurse -Force
    }
    New-Item -ItemType Directory -Path $WINDOWS_DIR | Out-Null
    
    # Copy executable
    Copy-Item $WIN_PORTABLE (Join-Path $WINDOWS_DIR "ScreenLink.exe")
    
    # Create run.bat - simple echo approach
    $batPath = Join-Path $WINDOWS_DIR "run.bat"
    "@echo off" | Out-File $batPath -Encoding ASCII -Force
    "REM ScreenLink Portable" | Out-File $batPath -Encoding ASCII -Append
    "`"%~dp0ScreenLink.exe`" %*" | Out-File $batPath -Encoding ASCII -Append
    
    # Create run.ps1
    $psPath = Join-Path $WINDOWS_DIR "run.ps1"
    "`$scriptPath = Split-Path -Parent `$MyInvocation.MyCommand.Path" | Out-File $psPath -Encoding UTF8 -Force
    "& `"`$scriptPath\ScreenLink.exe`"" | Out-File $psPath -Encoding UTF8 -Append
    
    # Create README.txt
    $readmePath = Join-Path $WINDOWS_DIR "README.txt"
    "ScreenLink - Portable Version (Windows)" | Out-File $readmePath -Encoding ASCII -Force
    "======================================" | Out-File $readmePath -Encoding ASCII -Append
    "" | Out-File $readmePath -Encoding ASCII -Append
    "NO INSTALLATION REQUIRED!" | Out-File $readmePath -Encoding ASCII -Append
    "" | Out-File $readmePath -Encoding ASCII -Append
    "To run:" | Out-File $readmePath -Encoding ASCII -Append
    "  1. Double-click ScreenLink.exe" | Out-File $readmePath -Encoding ASCII -Append
    "  2. Or double-click run.bat" | Out-File $readmePath -Encoding ASCII -Append
    "  3. Or run via PowerShell: .\run.ps1" | Out-File $readmePath -Encoding ASCII -Append
    "" | Out-File $readmePath -Encoding ASCII -Append
    "No admin permissions needed!" | Out-File $readmePath -Encoding ASCII -Append
    
    # Create ZIP
    $ZIP_PATH = Join-Path $DIST_DIR "ScreenLink-$VERSION-windows-portable.zip"
    if (Test-Path $ZIP_PATH) {
        Remove-Item $ZIP_PATH
    }
    
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [System.IO.Compression.ZipFile]::CreateFromDirectory($WINDOWS_DIR, $ZIP_PATH)
}
