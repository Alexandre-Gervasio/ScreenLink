# PowerShell script para criar versões portáveis do ScreenLink

$VERSION = "1.0.0"
$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$DIST_DIR = Join-Path $SCRIPT_DIR "dist-portable"

Write-Host "📦 Criando versões portáveis do ScreenLink v$VERSION..." -ForegroundColor Green

# Criar diretório
if (-not (Test-Path $DIST_DIR)) {
    New-Item -ItemType Directory -Path $DIST_DIR | Out-Null
}

# ============================================================================
# WINDOWS PORTABLE
# ============================================================================

$WIN_PORTABLE = Join-Path $SCRIPT_DIR "frontend/target/release/ScreenLink.exe"

if (Test-Path $WIN_PORTABLE) {
    Write-Host ""
    Write-Host "🪟 Preparando Windows Portable..." -ForegroundColor Cyan
    
    $WINDOWS_DIR = Join-Path $DIST_DIR "ScreenLink-$VERSION-windows-portable"
    
    if (Test-Path $WINDOWS_DIR) {
        Remove-Item $WINDOWS_DIR -Recurse -Force
    }
    New-Item -ItemType Directory -Path $WINDOWS_DIR | Out-Null
    
    # Copiar executável
    Copy-Item $WIN_PORTABLE (Join-Path $WINDOWS_DIR "ScreenLink.exe")
    
    # Criar batch script
    $batLines = @(
        '@echo off',
        'REM ScreenLink - Portable Version for Windows',
        'REM NO INSTALLATION REQUIRED!',
        'REM Just double-click this file or run: ScreenLink.exe',
        '',
        '"%~dp0ScreenLink.exe" %*'
    )
    $batContent = $batLines -join ([System.Environment]::NewLine)
    Set-Content -Path (Join-Path $WINDOWS_DIR "run.bat") -Value $batContent -Encoding ASCII -Force
    
    # Criar PowerShell script
    $psLines = @(
        '# ScreenLink - Portable Version for Windows',
        '# NO INSTALLATION REQUIRED!',
        '',
        '$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path',
        '& "$scriptPath\ScreenLink.exe"'
    )
    $psContent = $psLines -join ([System.Environment]::NewLine)
    Set-Content -Path (Join-Path $WINDOWS_DIR "run.ps1") -Value $psContent -Encoding UTF8 -Force
    
    # Criar README
    $readmeLines = @(
        'ScreenLink - Portable Version (Windows)',
        '======================================',
        '',
        'NO INSTALLATION REQUIRED!',
        '',
        'To run:',
        '  1. Double-click ScreenLink.exe',
        '  2. Or double-click run.bat',
        '  3. Or run via PowerShell: .\run.ps1',
        '',
        'No admin permissions needed!',
        '',
        'Troubleshooting:',
        '  If Windows shows "Windows protected your PC":',
        '    1. Click "More info"',
        '    2. Click "Run anyway"',
        '',
        '  This is normal for unsigned executables.',
        '',
        'Enjoy!'
    )
    $readmeContent = $readmeLines -join ([System.Environment]::NewLine)
    Set-Content -Path (Join-Path $WINDOWS_DIR "README.txt") -Value $readmeContent -Encoding ASCII -Force
    
    # Criar ZIP
    $ZIP_PATH = Join-Path $DIST_DIR "ScreenLink-$VERSION-windows-portable.zip"
    if (Test-Path $ZIP_PATH) {
        Remove-Item $ZIP_PATH
    }
    
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [System.IO.Compression.ZipFile]::CreateFromDirectory($WINDOWS_DIR, $ZIP_PATH)
    
    Write-Host "✅ Windows Portable: ScreenLink-$VERSION-windows-portable.zip"
}

# ============================================================================
# SUMMARY
# ============================================================================

Write-Host ""
Write-Host "✨ Versões Portáveis Criadas:" -ForegroundColor Green
Write-Host ""
Get-ChildItem (Join-Path $DIST_DIR "*.zip") -ErrorAction SilentlyContinue | ForEach-Object {
    Write-Host "  - $($_.Name) ($([math]::Round($_.Length/1MB, 2)) MB)"
}

Write-Host ""
Write-Host "📍 Localização: $DIST_DIR/" -ForegroundColor Yellow
Write-Host ""
Write-Host "✅ Pronto para download!" -ForegroundColor Green
