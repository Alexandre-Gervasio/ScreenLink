#!/bin/bash
# Auto-Update System for KVM Pro
# Checks GitHub for new releases and updates automatically

set -e

# Configuration
GITHUB_REPO="${GITHUB_REPO:-yourusername/kvm-pro}"
INSTALL_DIR="${1:-.}"
LOG_FILE="${INSTALL_DIR}/.kvm-pro-update.log"
MARKER_FILE="${INSTALL_DIR}/.last-update-check"
CHECK_INTERVAL_DAYS=1

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

# Check if we should check for updates
should_check_updates() {
    if [ ! -f "$MARKER_FILE" ]; then
        return 0  # First time, should check
    fi

    local last_check=$(stat -f%m "$MARKER_FILE" 2>/dev/null || stat -c%Y "$MARKER_FILE" 2>/dev/null || echo 0)
    local current_time=$(date +%s)
    local diff=$((current_time - last_check))
    local interval=$((CHECK_INTERVAL_DAYS * 86400))

    [ $diff -ge $interval ]
}

# Get current version
get_current_version() {
    if command -v ./kvm-pro-server &> /dev/null; then
        ./kvm-pro-server --version 2>/dev/null | head -1 || echo "unknown"
    else
        echo "unknown"
    fi
}

# Get latest version from GitHub API
get_latest_version() {
    local api_url="https://api.github.com/repos/$GITHUB_REPO/releases/latest"
    
    if command -v curl &> /dev/null; then
        curl -s "$api_url" | grep -o '"tag_name": "[^"]*"' | head -1 | cut -d'"' -f4 || echo ""
    elif command -v wget &> /dev/null; then
        wget -q -O- "$api_url" | grep -o '"tag_name": "[^"]*"' | head -1 | cut -d'"' -f4 || echo ""
    else
        echo ""
    fi
}

# Download latest release
download_update() {
    local version=$1
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch="x86_64"
    
    if [[ "$os" == "linux" ]]; then
        local package="kvm-pro-linux-x64-portable-${version}.tar.gz"
        local download_url="https://github.com/$GITHUB_REPO/releases/download/$version/$package"
    elif [[ "$os" == "darwin" ]]; then
        local package="kvm-pro-macos-x64-portable-${version}.tar.gz"
        local download_url="https://github.com/$GITHUB_REPO/releases/download/$version/$package"
    elif [[ "$os" == "mingw"* ]] || [[ "$os" == "msys"* ]]; then
        local package="kvm-pro-windows-x64-portable-${version}.zip"
        local download_url="https://github.com/$GITHUB_REPO/releases/download/$version/$package"
    else
        return 1
    fi

    log "Downloading: $download_url"
    
    if command -v curl &> /dev/null; then
        curl -L -o "/tmp/$package" "$download_url" || return 1
    elif command -v wget &> /dev/null; then
        wget -O "/tmp/$package" "$download_url" || return 1
    else
        return 1
    fi

    echo "/tmp/$package"
}

# Install update
install_update() {
    local package=$1
    local backup_dir="${INSTALL_DIR}/.backup-$(date +%s)"
    
    log "Installing update from: $package"
    
    # Backup current installation
    mkdir -p "$backup_dir"
    cp "$INSTALL_DIR"/kvm-pro-* "$backup_dir/" 2>/dev/null || true
    
    # Extract new version
    if [[ "$package" == *.tar.gz ]]; then
        tar xzf "$package" --strip-components=1 -C "$INSTALL_DIR" || {
            log "ERROR: Failed to extract update. Restoring backup..."
            cp "$backup_dir"/kvm-pro-* "$INSTALL_DIR/"
            return 1
        }
    elif [[ "$package" == *.zip ]]; then
        unzip -o "$package" -d "$INSTALL_DIR" || {
            log "ERROR: Failed to extract update. Restoring backup..."
            cp "$backup_dir"/kvm-pro-* "$INSTALL_DIR/"
            return 1
        }
    fi

    chmod +x "$INSTALL_DIR"/kvm-pro-* 2>/dev/null || true
    
    log "✅ Update installed successfully"
    echo "✅ Update installed successfully"
    
    # Clean up
    rm -f "$package"
    
    return 0
}

# Main update function
check_and_update() {
    if ! should_check_updates; then
        return 0
    fi

    log "Checking for updates..."
    
    local current=$(get_current_version)
    local latest=$(get_latest_version)
    
    if [ -z "$latest" ]; then
        log "Could not determine latest version"
        touch "$MARKER_FILE"
        return 0
    fi

    log "Current version: $current"
    log "Latest version: $latest"

    # Compare versions (simple string comparison - works for semantic versioning)
    if [ "$current" != "$latest" ] && [ ! -z "$current" ] && [ ! -z "$latest" ]; then
        echo ""
        echo -e "${BLUE}═══════════════════════════════════════════${NC}"
        echo -e "${YELLOW}🔄 Update Available!${NC}"
        echo -e "${BLUE}═══════════════════════════════════════════${NC}"
        echo "Current version: $current"
        echo "New version: $latest"
        echo -e "${BLUE}═══════════════════════════════════════════${NC}"
        echo ""
        
        read -p "Would you like to update now? (y/N): " -n 1 -r
        echo ""
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            local package=$(download_update "$latest") || {
                log "ERROR: Failed to download update"
                return 1
            }
            
            install_update "$package" || return 1
            
            log "Update completed. Please restart the application."
            echo ""
            echo -e "${GREEN}✅ Update completed!${NC}"
            echo "Please restart the application to use the new version."
            echo ""
        else
            log "User declined update"
        fi
    fi

    touch "$MARKER_FILE"
    return 0
}

# Run update check
check_and_update
