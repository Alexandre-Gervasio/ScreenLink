#!/bin/bash
set -e

REPO="Alexandre-Gervasio/kvm-pro"
DIST_DIR="dist"

# Cores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}KVM Pro - Release Publisher${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Verificar autenticação
if ! gh auth status > /dev/null 2>&1; then
    echo -e "${RED}❌ Não autenticado no GitHub${NC}"
    echo ""
    echo "Execute: gh auth login"
    echo ""
    echo "Escolha:"
    echo "  1. GitHub.com"
    echo "  2. HTTPS"
    echo "  3. SSH"
    echo ""
    exit 1
fi

echo -e "${GREEN}✅ Autenticação detectada${NC}\n"

# Publicar v1.0.2
echo -e "${BLUE}📦 Publicando v1.0.2...${NC}"
gh release create v1.0.2 \
  --repo "$REPO" \
  --title "v1.0.2 - Web GUI with Start/Stop Controls" \
  --notes "🎛️ v1.0.2 - Web GUI Control Panel

**NEW IN v1.0.2:**
✅ Web-based GUI with actix-web server
✅ Beautiful interface with start/stop buttons
✅ Portuguese and English text
✅ Control server and client from one interface
✅ Access via http://127.0.0.1:8080
✅ Works on both Linux and Windows

**BINARIES INCLUDED:**
- kvm-pro-ui-linux (2.8M) - Linux GUI
- kvm-pro-ui.exe (2.5M) - Windows GUI
- kvm-pro-server-linux (510K) - Server
- kvm-pro-client-linux (489K) - Client
- kvm-pro-server.exe (431K) - Server
- kvm-pro-client.exe (412K) - Client

**USAGE:**
\`\`\`bash
# Linux
./kvm-pro-ui-linux
# Open: http://127.0.0.1:8080

# Windows
kvm-pro-ui.exe
# Open: http://127.0.0.1:8080
\`\`\`" \
  --prerelease=false \
  "$DIST_DIR/kvm-pro-ui-linux" \
  "$DIST_DIR/kvm-pro-ui.exe" \
  "$DIST_DIR/kvm-pro-server-linux" \
  "$DIST_DIR/kvm-pro-client-linux" \
  "$DIST_DIR/kvm-pro-server.exe" \
  "$DIST_DIR/kvm-pro-client.exe"

echo -e "${GREEN}✅ v1.0.2 publicada com sucesso!${NC}\n"

# Publicar v1.0.3
echo -e "${BLUE}📦 Publicando v1.0.3...${NC}"
gh release create v1.0.3 \
  --repo "$REPO" \
  --title "v1.0.3 - Automatic IP Detection & Link Generator" \
  --notes "🔗 v1.0.3 - Automatic IP Detection & Link Generator

**NEW IN v1.0.3:**
✅ Auto-detect all available local IPs
✅ Display sharable URLs for all network interfaces
✅ Copy-to-clipboard buttons for easy sharing
✅ /api/info endpoint for programmatic access
✅ Server listens on 0.0.0.0 (all interfaces, not just localhost)
✅ Show multiple access URLs on startup

**UI ENHANCEMENTS:**
• Link sharing section with copy buttons
• Better visual design
• Mobile friendly layout
• Version info displayed clearly

**WHY THIS MATTERS:**
- No more hardcoded localhost (127.0.0.1)
- Easy to share access with other devices on network
- Automatic IP detection works across Linux/Windows
- Perfect for LAN sharing

**BINARIES INCLUDED:**
- kvm-pro-ui-linux (2.8M) - Linux GUI
- kvm-pro-ui.exe (2.5M) - Windows GUI
- kvm-pro-server-linux (510K) - Server
- kvm-pro-client-linux (489K) - Client
- kvm-pro-server.exe (431K) - Server
- kvm-pro-client.exe (412K) - Client

**API ENDPOINT:**
\`\`\`bash
GET /api/info
\`\`\`

Response:
\`\`\`json
{
  \"version\": \"1.0.3\",
  \"name\": \"KVM Pro Control Panel\",
  \"local_ips\": [\"192.168.1.100\", \"127.0.0.1\"],
  \"access_urls\": [\"http://192.168.1.100:8080\", \"http://127.0.0.1:8080\"],
  \"port\": 8080
}
\`\`\`" \
  --prerelease=false \
  "$DIST_DIR/kvm-pro-ui-linux" \
  "$DIST_DIR/kvm-pro-ui.exe" \
  "$DIST_DIR/kvm-pro-server-linux" \
  "$DIST_DIR/kvm-pro-client-linux" \
  "$DIST_DIR/kvm-pro-server.exe" \
  "$DIST_DIR/kvm-pro-client.exe"

echo -e "${GREEN}✅ v1.0.3 publicada com sucesso!${NC}\n"

echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✅ TODAS AS RELEASES PUBLICADAS!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "📱 Acesse:"
echo "  https://github.com/$REPO/releases"
echo ""

