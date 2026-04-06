# ScreenLink Portable Build Script for Windows PowerShell
param(
    [switch]$CI = $false
)

Write-Host "📦 ScreenLink Portable Build Script (Windows)" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

# Check prerequisites
Write-Host "🔍 Checking prerequisites..." -ForegroundColor Yellow

# Check Node.js
try {
    $nodeVersion = node --version
    Write-Host "✓ Node.js: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Node.js not found. Please install Node.js 18+" -ForegroundColor Red
    exit 1
}

# Check npm
try {
    $npmVersion = npm --version
    Write-Host "✓ npm: v$npmVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ npm not found" -ForegroundColor Red
    exit 1
}

# Check Rust
try {
    $rustVersion = rustc --version
    Write-Host "✓ Rust: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "⚠️  Rust not found. Please install from https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

Write-Host ""

# Install dependencies
Write-Host "📥 Installing dependencies..." -ForegroundColor Yellow
npm ci
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to install dependencies" -ForegroundColor Red
    exit 1
}

# Build backend
Write-Host ""
Write-Host "🔨 Building Backend..." -ForegroundColor Cyan
npm run build:backend
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Backend build failed" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Backend built" -ForegroundColor Green

# Build frontend
Write-Host ""
Write-Host "🔨 Building Frontend..." -ForegroundColor Cyan
npm run build:frontend
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Frontend build failed" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Frontend built" -ForegroundColor Green

# Build Tauri app
Write-Host ""
Write-Host "🔨 Building Tauri App..." -ForegroundColor Cyan
npm run build:tauri

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Tauri build failed" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Tauri app built" -ForegroundColor Green

# Find output files
Write-Host ""
Write-Host "📦 Build Output:" -ForegroundColor Cyan

$exeFile = Get-ChildItem -Path "frontend/target/release/bundle" -Filter "*.exe" -Recurse | Select-Object -First 1
$msiFile = Get-ChildItem -Path "frontend/target/release/bundle" -Filter "*.msi" -Recurse | Select-Object -First 1

if ($exeFile) {
    $size = [math]::Round(($exeFile.Length / 1MB), 2)
    Write-Host "✓ Executable: $($exeFile.FullName)" -ForegroundColor Green
    Write-Host "  Size: $size MB"
}

if ($msiFile) {
    $size = [math]::Round(($msiFile.Length / 1MB), 2)
    Write-Host "✓ Installer: $($msiFile.FullName)" -ForegroundColor Green
    Write-Host "  Size: $size MB"
}

if ($exeFile -or $msiFile) {
    Write-Host ""
    Write-Host "🎉 Build completed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "To run the portable app:"
    if ($exeFile) {
        Write-Host "  & '$($exeFile.FullName)'"
    }
} else {
    Write-Host "⚠️  Could not locate output files" -ForegroundColor Yellow
    Write-Host "Check frontend/target/release/bundle/ for build artifacts"
}

Write-Host ""
