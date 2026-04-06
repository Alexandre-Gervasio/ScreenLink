#!/bin/bash

echo "========================================"
echo "KVM Pro - Release Publisher (All Versions)"
echo "========================================"
echo ""

# Check authentication
echo "🔐 Verificando autenticação..."
if ! gh auth status 2>/dev/null | grep -q "Logged in"; then
    echo "❌ Não autenticado no GitHub!"
    echo "Execute: gh auth login"
    exit 1
fi
echo "✅ Autenticação detectada"
echo ""

# v1.0.2 - Web GUI with Start/Stop Controls
echo "📦 Publicando v1.0.2..."
gh release create v1.0.2 \
  --repo "Alexandre-Gervasio/kvm-pro" \
  --title "v1.0.2 - Web GUI with Start/Stop Controls" \
  --notes "## Features
- 🌐 Web-based interface (actix-web)
- 🎛️ Start/Stop controls for Server and Client
- 📱 Access from any browser
- 🔗 Shareable links with IP addresses
- 🖥️ Cross-platform: Linux and Windows

## Improvements
- Replaced FLTK with actix-web for better cross-platform support
- Removed X11 dependencies causing Linux issues
- Simplified deployment with single HTTP server

## Usage
\`\`\`bash
./kvm-pro-ui-linux
# or
kvm-pro-ui.exe
\`\`\`

Then open http://localhost:8080 in your browser." \
  dist/kvm-pro-ui-linux \
  dist/kvm-pro-ui.exe \
  dist/kvm-pro-server-linux \
  dist/kvm-pro-client-linux \
  dist/kvm-pro-server.exe \
  dist/kvm-pro-client.exe 2>/dev/null

echo "https://github.com/Alexandre-Gervasio/kvm-pro/releases/tag/v1.0.2"
echo "✅ v1.0.2 publicada com sucesso!"
echo ""

# v1.0.3 - Automatic IP Detection & Link Generator
echo "📦 Publicando v1.0.3..."
gh release create v1.0.3 \
  --repo "Alexandre-Gervasio/kvm-pro" \
  --title "v1.0.3 - Automatic IP Detection & Link Generator" \
  --notes "## Features
- 🔍 Automatic IP detection (no hardcoding needed!)
- 🔗 Smart link generator showing all available IPs
- 📋 Copy-to-clipboard buttons for easy sharing
- 🌐 Web interface with real-time status
- 💻 Server/Client process management

## What's New in v1.0.3
- Auto-detects local IPs using UDP socket trick
- Shows shareable links for each detected network interface
- One-click copy functionality for links
- Better UX with visual indicators for status

## Usage
\`\`\`bash
./kvm-pro-ui-linux
# or
kvm-pro-ui.exe
\`\`\`

Access from: http://localhost:8080

Share the detected IP with other users!" \
  dist/kvm-pro-ui-linux \
  dist/kvm-pro-ui.exe \
  dist/kvm-pro-server-linux \
  dist/kvm-pro-client-linux \
  dist/kvm-pro-server.exe \
  dist/kvm-pro-client.exe 2>/dev/null

echo "https://github.com/Alexandre-Gervasio/kvm-pro/releases/tag/v1.0.3"
echo "✅ v1.0.3 publicada com sucesso!"
echo ""

# v1.0.4 - P2P Terminal Interface
echo "📦 Publicando v1.0.4..."
gh release create v1.0.4 \
  --repo "Alexandre-Gervasio/kvm-pro" \
  --title "v1.0.4 - P2P Terminal Interface with Auto-Discovery" \
  --notes "## Features
- 🚀 **Unified Single Binary** - No more separate server/client/ui
- 🔎 **P2P Auto-Discovery** - Machines find each other automatically via UDP broadcast
- 💻 **TUI (Terminal User Interface)** - Beautiful terminal interface with colors and navigation
- 🎨 **Rich Terminal UI** - ratatui-based interactive display
- ⚡ **Lightweight** - 1.1MB (Linux), 659KB (Windows)
- 🌐 **Cross-Platform** - Linux (musl) and Windows (GNU)

## What's New in v1.0.4
- No more web dependencies! Removed actix-web, actix-rt
- True P2P architecture - peer discovery via UDP broadcast
- Terminal-based interface - works over SSH or local terminal
- Significantly smaller binaries (1/3 of v1.0.3)
- Better portability - no browser required

## Usage
\`\`\`bash
# Linux
./kvm-pro-linux

# Windows
kvm-pro.exe
\`\`\`

Navigate with:
- **⬆️ ⬇️** Arrow keys to select peers
- **Enter** to connect to a peer
- **Q** to quit

Interface shows:
- 🌐 Your local IP address
- 💻 List of discovered peers on the network
- ✅ Connection status in real-time

## Architecture
- UDP broadcast discovery (port 6969)
- TCP for peer-to-peer communication (port 5000)
- Automatic peer detection - no manual IP entry needed!

## Previous Versions
For users preferring web interface: **v1.0.3** still available" \
  dist/kvm-pro-linux \
  dist/kvm-pro.exe 2>/dev/null

echo "https://github.com/Alexandre-Gervasio/kvm-pro/releases/tag/v1.0.4"
echo "✅ v1.0.4 publicada com sucesso!"
echo ""

echo "========================================"
echo "✅ TODAS AS RELEASES PUBLICADAS!"
echo "========================================"
echo ""
echo "📱 Acesse:"
echo "  https://github.com/Alexandre-Gervasio/kvm-pro/releases"
echo ""
echo "Versões disponíveis: v1.0.2, v1.0.3, v1.0.4"
echo "Escolha a que melhor se adequa ao seu uso!"
