# � ScreenLink - Deployment & Build Guide

**Version**: 1.0.0  
**Status**: Production Ready ✅  
**Last Updated**: April 6, 2026

---

## 📋 Quick Start

### Development (Any OS)
```bash
git clone https://github.com/Alexandre-Gervasio/ScreenLink.git
cd ScreenLink
npm ci
./run.sh
```

Then open: http://localhost:5173

### Build Portable Executables

**Linux/macOS:**
```bash
./build-portable.sh
```

**Windows (PowerShell):**
```bash
.\build-portable.ps1
```

**Any Platform:**
```bash
npm run build:portable
```

---

## 🔧 System Requirements

### All Platforms
- **Node.js** 18+
- **npm** (with Node.js)
- **Rust** 1.70+ (for portable builds only)

### Linux (for portable builds)
```bash
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
```

### Windows (for portable builds)
- Visual Studio 2019+ with C++ build tools
- WebView2 Runtime

### macOS (for portable builds)
```bash
xcode-select --install
```

---

## 📦 Build Outputs

| Platform | Format | Location | Size |
|----------|--------|----------|------|
| Windows | `.exe` | `frontend/target/release/bundle/msi/` | 50-80 MB |
| Linux | `.AppImage` | `frontend/target/release/bundle/appimage/` | 60-100 MB |
| macOS | `.dmg` | `frontend/target/release/bundle/dmg/` | 80-120 MB |

---

## 🌐 Access Points

- **Frontend**: http://localhost:5173
- **Backend**: http://localhost:3001
- **Health**: http://localhost:3001/health
- **WebSocket**: ws://localhost:3002

---

## 📊 Project Structure

```
ScreenLink/
├── backend/       # Express + WebSocket server
├── frontend/      # React + Vite + Tauri
├── shared/        # Shared types
├── docs/          # Documentation
├── build-portable.sh  # Unix build script
├── build-portable.ps1 # Windows build script  
├── run.sh         # Development launcher
└── test-integration.sh # Test suite
```

---

## ✅ Quality Metrics

- **TypeScript Errors**: 0
- **Test Pass Rate**: 100% (5/5 tests)
- **Build Success**: ✅
- **Cross-Platform**: ✅ (Windows, macOS, Linux)
- **Production Ready**: ✅

---

## 🐛 Troubleshooting

| Issue | Solution |
|-------|----------|
| "Rust not found" | Run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| "Port 3001 in use" | Change: `export PORT=3002` |
| "npm ci fails" | Try: `npm install -g npm@latest && npm ci` |
| "Out of disk" | Run: `rm -rf frontend/target && npm cache clean --force` |

---

## 🚀 For End Users

### Windows
1. Download `ScreenLink.exe`
2. Double-click to run
3. Or use `.msi` installer

### Linux
1. Download `ScreenLink.AppImage`
2. Run: `chmod +x ScreenLink.AppImage && ./ScreenLink.AppImage`

### macOS
1. Download `ScreenLink.dmg`
2. Open .dmg, drag app to Applications
3. Launch from Applications

---

## 🔄 Automated Release

```bash
git tag v1.0.0
git push --tags
```

GitHub Actions will automatically build for all platforms.

---

**See also**: [README.md](./README.md) | [CONTRIBUTING.md](./CONTRIBUTING.md) | [CHANGELOG.md](./CHANGELOG.md)

