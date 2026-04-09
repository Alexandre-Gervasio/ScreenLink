# Minimal Windows portable creator - no complex strings, no arrays
$VERSION = "1.0.0"
$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$DIST_DIR = Join-Path $SCRIPT_DIR "dist-portable"
$WIN_PORTABLE = Join-Path $SCRIPT_DIR "frontend/target/release/ScreenLink.exe"
$BACKEND_DIR = Join-Path $SCRIPT_DIR "backend"

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
    
    # Copy backend if exists
    if (Test-Path $BACKEND_DIR) {
        Copy-Item $BACKEND_DIR (Join-Path $WINDOWS_DIR "backend") -Recurse
    }
    
    # Create run.bat with backend startup
    $batPath = Join-Path $WINDOWS_DIR "run.bat"
    "@echo off" | Out-File $batPath -Encoding ASCII -Force
    "REM ScreenLink Portable" | Out-File $batPath -Encoding ASCII -Append
    "REM Start backend server if available" | Out-File $batPath -Encoding ASCII -Append
    "if exist `"%~dp0backend`" (" | Out-File $batPath -Encoding ASCII -Append
    "    cd /d `"%~dp0backend`"" | Out-File $batPath -Encoding ASCII -Append
    "    npm install --silent >nul 2>&1" | Out-File $batPath -Encoding ASCII -Append
    "    START /B npm run start >nul 2>&1" | Out-File $batPath -Encoding ASCII -Append
    "    timeout /t 1 /nobreak >nul" | Out-File $batPath -Encoding ASCII -Append
    "    cd /d `"%~dp0.`"" | Out-File $batPath -Encoding ASCII -Append
    ")" | Out-File $batPath -Encoding ASCII -Append
    "`"%~dp0ScreenLink.exe`" %*" | Out-File $batPath -Encoding ASCII -Append
    
    # Create run.ps1
    $psPath = Join-Path $WINDOWS_DIR "run.ps1"
    "`$scriptPath = Split-Path -Parent `$MyInvocation.MyCommand.Path" | Out-File $psPath -Encoding UTF8 -Force
    "`$backendPath = Join-Path `$scriptPath 'backend'" | Out-File $psPath -Encoding UTF8 -Append
    "if (Test-Path `$backendPath) {" | Out-File $psPath -Encoding UTF8 -Append
    "    Push-Location `$backendPath" | Out-File $psPath -Encoding UTF8 -Append
    "    npm install --silent 2>null" | Out-File $psPath -Encoding UTF8 -Append
    "    Start-Job -ScriptBlock { npm run start } -WorkingDirectory `$args[0] -ArgumentList `$pwd | Out-Null" | Out-File $psPath -Encoding UTF8 -Append
    "    Start-Sleep -Milliseconds 500" | Out-File $psPath -Encoding UTF8 -Append
    "    Pop-Location" | Out-File $psPath -Encoding UTF8 -Append
    "}" | Out-File $psPath -Encoding UTF8 -Append
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
    "  2. Or double-click run.bat (starts backend automatically)" | Out-File $readmePath -Encoding ASCII -Append
    "  3. Or run via PowerShell: .\run.ps1" | Out-File $readmePath -Encoding ASCII -Append
    "" | Out-File $readmePath -Encoding ASCII -Append
    "The backend server will start automatically!" | Out-File $readmePath -Encoding ASCII -Append
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
