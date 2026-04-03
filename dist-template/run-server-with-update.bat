@echo off
REM KVM Pro Server Launcher with Auto-Update (Windows)
REM This batch file checks for updates and then runs the server

setlocal enabledelayedexpansion

cd /d "%~dp0"

REM Set GitHub repo (override with environment variable if needed)
if not defined GITHUB_REPO (
    set "GITHUB_REPO=yourusername/kvm-pro"
)

echo Checking for updates...
if exist "auto-update.bat" (
    REM Call auto-update quietly
    call auto-update.bat "%CD%" 2>nul || (
        echo Note: Could not check for updates, continuing anyway...
    )
)

REM Run the server
echo Starting KVM Pro Server...
cmd /c kvm-pro-server.exe %*
