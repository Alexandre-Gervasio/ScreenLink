@echo off
REM KVM Pro Auto-Update Script for Windows
REM Downloads and installs updates from GitHub releases

setlocal enabledelayedexpansion

REM Configuration
set "CHECK_INTERVAL_DAYS=1"
set "GITHUB_REPO=%GITHUB_REPO%"
set "LOG_FILE=%USERPROFILE%\.kvm-pro-update.log"
set "TEMP_DIR=%TEMP%\kvm-pro-update"

if "%GITHUB_REPO%"=="" (
    echo Error: GITHUB_REPO environment variable not set
    exit /b 1
)

REM Parse arguments
set "INSTALL_DIR=%1"
if "!INSTALL_DIR!"=="" (
    set "INSTALL_DIR=%CD%"
)

REM ========== Logging function ==========
setlocal enabledelayedexpansion
(
    echo [%date% %time%] Auto-update started
    echo Repository: %GITHUB_REPO%
    echo Install directory: %INSTALL_DIR%
) >> "%LOG_FILE%"

REM ========== Check if we should update ==========
set "CHECK_FILE=%INSTALL_DIR%\.last-update-check"

if exist "%CHECK_FILE%" (
    REM Parse last check date (simplified - assumes YYYY-MM-DD format)
    for /f %%i in (%CHECK_FILE%) do set "LAST_CHECK=%%i"
    REM For simplicity, always check if this runs (you can enhance this)
    echo [%date% %time%] Last check: !LAST_CHECK! >> "%LOG_FILE%"
) else (
    echo [%date% %time%] First check, proceeding >> "%LOG_FILE%"
)

REM ========== Get current version ==========
if not exist "%INSTALL_DIR%\kvm-pro-server.exe" (
    echo [%date% %time%] Error: kvm-pro-server.exe not found >> "%LOG_FILE%"
    exit /b 1
)

REM Try to get version from binary
set "CURRENT_VERSION=unknown"
for /f "tokens=*" %%i in ('%INSTALL_DIR%\kvm-pro-server.exe --version 2^>nul ^| findstr /r ".*"') do (
    set "CURRENT_VERSION=%%i"
)

echo [%date% %time%] Current version: !CURRENT_VERSION! >> "%LOG_FILE%"

REM ========== Get latest version from GitHub ==========
echo [%date% %time%] Checking GitHub for latest release... >> "%LOG_FILE%"

REM Create temp directory
if not exist "%TEMP_DIR%" mkdir "%TEMP_DIR%"

REM Download latest release info using curl or wget
set "LATEST_RELEASE_URL=https://api.github.com/repos/%GITHUB_REPO%/releases/latest"

if exist "C:\Windows\System32\curl.exe" (
    curl -s -L "%LATEST_RELEASE_URL%" > "%TEMP_DIR%\release.json" 2>nul
) else (
    REM Windows 10 has built-in Invoke-WebRequest
    powershell -Command "try { (Invoke-WebRequest -Uri '%LATEST_RELEASE_URL%' -UseBasicParsing).Content | Out-File '%TEMP_DIR%\release.json' } catch { }"
)

if not exist "%TEMP_DIR%\release.json" (
    echo [%date% %time%] Could not fetch latest release info >> "%LOG_FILE%"
    exit /b 0
)

REM Extract version tag and download URL (simplified parsing)
setlocal enabledelayedexpansion
for /f "tokens=2 delims=/" %%i in ('findstr "tag_name" "%TEMP_DIR%\release.json"') do (
    set "LATEST_VERSION=%%i"
    set "LATEST_VERSION=!LATEST_VERSION:"=!
    set "LATEST_VERSION=!LATEST_VERSION:,=!"
)

echo [%date% %time%] Latest version available: !LATEST_VERSION! >> "%LOG_FILE%"

REM ========== Compare versions ==========
if "!LATEST_VERSION!"=="!CURRENT_VERSION!" (
    echo [%date% %time%] Already on latest version >> "%LOG_FILE%"
    REM Update check timestamp
    echo !date! > "%CHECK_FILE%"
    exit /b 0
)

echo [%date% %time%] Update available: !CURRENT_VERSION! ^-^> !LATEST_VERSION! >> "%LOG_FILE%"

REM ========== Ask user ==========
setlocal disabledelayedexpansion
echo.
echo ============================================
echo       KVM Pro - Update Available
echo ============================================
echo.
echo Current version: %CURRENT_VERSION%
echo Latest version:  !LATEST_VERSION!
echo.
set /p "UPDATE_CHOICE=Would you like to update now? (y/n) "

if /i not "!UPDATE_CHOICE!"=="y" (
    echo [%date% %time%] User declined update >> "%LOG_FILE%"
    exit /b 0
)

REM ========== Download update ==========
echo Downloading update...

REM Extract download URL for Windows ZIP
REM This is simplified - in real scenario, parse JSON properly
set "DOWNLOAD_URL=https://github.com/%GITHUB_REPO%/releases/download/%LATEST_VERSION%/kvm-pro-windows.zip"

echo [%date% %time%] Downloading from: !DOWNLOAD_URL! >> "%LOG_FILE%"

if exist "C:\Windows\System32\curl.exe" (
    curl -L -o "%TEMP_DIR%\update.zip" "%DOWNLOAD_URL%" 2>nul
) else (
    powershell -Command "try { Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%TEMP_DIR%\update.zip' } catch { }"
)

if not exist "%TEMP_DIR%\update.zip" (
    echo [%date% %time%] Download failed >> "%LOG_FILE%"
    echo Error: Could not download update
    exit /b 1
)

echo [%date% %time%] Download complete >> "%LOG_FILE%"

REM ========== Backup current version ==========
echo Backing up current version...
set "BACKUP_DIR=%INSTALL_DIR%\backup-!CURRENT_VERSION!"

if not exist "%BACKUP_DIR%" mkdir "%BACKUP_DIR%"
copy "%INSTALL_DIR%\kvm-pro-server.exe" "%BACKUP_DIR%\" 2>nul
copy "%INSTALL_DIR%\kvm-pro-client.exe" "%BACKUP_DIR%\" 2>nul

echo [%date% %time%] Backup created at: !BACKUP_DIR! >> "%LOG_FILE%"

REM ========== Install update ==========
echo Installing update...

REM Extract to temp location
powershell -Command "Expand-Archive -Path '%TEMP_DIR%\update.zip' -DestinationPath '%TEMP_DIR%\extracted' -Force" 2>nul

REM Move new binaries
if exist "%TEMP_DIR%\extracted\kvm-pro-server.exe" (
    move /y "%TEMP_DIR%\extracted\kvm-pro-server.exe" "%INSTALL_DIR%\kvm-pro-server.exe"
)

if exist "%TEMP_DIR%\extracted\kvm-pro-client.exe" (
    move /y "%TEMP_DIR%\extracted\kvm-pro-client.exe" "%INSTALL_DIR%\kvm-pro-client.exe"
)

echo [%date% %time%] Update installed successfully >> "%LOG_FILE%"
echo.
echo ============================================
echo Update completed successfully!
echo Please restart the application.
echo ============================================
echo.

REM Update check timestamp
echo !date! > "%CHECK_FILE%"

REM Cleanup
rmdir /s /q "%TEMP_DIR%" 2>nul

exit /b 0
