#!/bin/bash
# Setup script for KVM Pro development dependencies

set -e

echo "=== KVM Pro Dependency Setup ==="

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    if [ -f /etc/debian_version ]; then
        DISTRO="debian"
    elif [ -f /etc/redhat-release ]; then
        DISTRO="redhat"
    else
        DISTRO="unknown"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    OS="unknown"
fi

echo "Detected OS: $OS ($DISTRO)"

case "$OS" in
    linux)
        if [[ "$DISTRO" == "debian" ]]; then
            echo "Installing dependencies for Debian/Ubuntu..."
            sudo apt-get update
            sudo apt-get install -y \
                build-essential \
                libevdev-dev \
                pkg-config \
                libssl-dev
        elif [[ "$DISTRO" == "redhat" ]]; then
            echo "Installing dependencies for RedHat/Fedora..."
            sudo dnf install -y \
                gcc \
                libevdev-devel \
                pkg-config \
                openssl-devel
        fi
        ;;
    macos)
        echo "Installing dependencies for macOS..."
        brew install libevdev
        ;;
    *)
        echo "Cannot identify OS. Please install dependencies manually."
        echo "Required packages:"
        echo "  - libevdev-dev (or libevdev-devel for Fedora)"
        echo "  - build-essentials"
        echo "  - openssl-dev"
        exit 1
        ;;
esac

echo "Installing Rust (if not already installed)..."
if ! command -v cargo &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed: $(rustc --version)"
fi

echo "Updating Rust..."
rustup update

echo "Adding input group (Linux)..."
if [[ "$OS" == "linux" ]]; then
    if ! groups | grep -q input; then
        sudo usermod -a -G input $USER
        echo "Added user to 'input' group. You may need to log out and back in."
    fi
fi

echo "=== Setup Complete ==="
echo ""
echo "Next steps:"
echo "  1. cd core"
echo "  2. cargo build --release"
echo "  3. sudo ./target/release/kvm-pro-server"
