# ScreenLink Portable Builds 🎉

This guide explains how to build ScreenLink as portable executables for Windows, macOS, and Linux.

---

## 📋 Requirements

### All Platforms
- **Node.js** 18+ ([download](https://nodejs.org))
- **npm** (comes with Node.js)
- **Rust** 1.70+ ([install](https://rustup.rs))

### Linux-specific
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
```

### Windows-specific
- Visual Studio 2019+ with C++ build tools
- (Or Visual Studio Community with "Desktop development with C++")

### macOS-specific
- Xcode Command Line Tools
```bash
xcode-select --install
```

---

## 🚀 Quick Start

### On Linux/macOS
```bash
./build-portable.sh
```

### On Windows (PowerShell)
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
.\build-portable.ps1
```

---

## 📦 Build Output

### Windows (.exe)
- **Output**: `frontend/target/release/bundle/msi/*.exe`
- **Installer**: `frontend/target/release/bundle/msi/*.msi`
- **Size**: ~50-80 MB

### Linux (.AppImage)
- **Output**: `frontend/target/release/bundle/appimage/*.AppImage`
- **Size**: ~60-100 MB
- **Portable**: Works on any Linux distribution
- **Run**: `./ScreenLink-*.AppImage`

### macOS (.dmg)
- **Output**: `frontend/target/release/bundle/dmg/*.dmg`
- **Size**: ~80-120 MB
- **Install**: Drag app to Applications folder

---

## 🔧 Manual Build Steps

If the scripts don't work, build manually:

```bash
# 1. Install dependencies
npm ci

# 2. Build backend
npm run build:backend

# 3. Build frontend
npm run build:frontend

# 4. Build portable executables with Tauri
npm run build:tauri
# or
npm run app:build

# 5. Find output
# Windows: frontend/target/release/bundle/msi/
# Linux:   frontend/target/release/bundle/appimage/
# macOS:   frontend/target/release/bundle/dmg/
```

---

## 🐛 Troubleshooting

### "cargo not found"
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
```

### "Could not find webview2"  (Windows)
Download and install WebView2 Runtime:
https://developer.microsoft.com/en-us/microsoft-edge/webview2/

### "libgtk-3-dev not found" (Linux)
```bash
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev
```

### "Build timeout"
Increase timeout or try again:
```bash
npm run app:build -- --ci
```

### Out of disk space
Tauri builds require ~2-3 GB free space. Clean up:
```bash
rm -rf frontend/target
npm cache clean --force
```

---

## 📤 Distribution

### For Windows Users
1. Share the `.exe` file (portable standalone)
2. Or the `.msi` installer for easier installation

### For Linux Users
1. Share the `.AppImage` file
2. Users run: `chmod +x ScreenLink*.AppImage && ./ScreenLink*.AppImage`

### For macOS Users
1. Share the `.dmg` file
2. Users drag app to Applications folder

---

## 🔒 Code Signing (Optional)

### Windows
Set environment variables before building:
```bash
set CSC_LINK=certificate.pfx
set CSC_KEY_PASSWORD=your_password
npm run app:build
```

### macOS
Set environment variables:
```bash
export APPLE_CERTIFICATE=certificate.p12
export APPLE_CERTIFICATE_PASSWORD=your_password
export APPLE_ID=your_email@example.com
export APPLE_PASSWORD=your_app_password
npm run app:build
```

---

## 📊 Build Configuration

ScreenLink uses Tauri 2.0 with these bundled formats:

| Platform | Format | Notes |
|----------|--------|-------|
| Windows  | .exe   | Portable, no installation needed |
| Windows  | .msi   | Installer for easier setup |
| Linux    | .AppImage | AppImage format, runs anywhere |
| Linux    | .deb   | Debian package |
| macOS    | .dmg   | Disk image, standard macOS delivery |

---

## 🚀 GitHub Actions CI/CD

ScreenLink automatically builds portable executables on each release:

```bash
git tag v1.0.0
git push origin --tags
```

GitHub Actions will:
1. Build on Windows (produces .exe/.msi)
2. Build on macOS (produces .dmg)
3. Build on Linux (produces .AppImage/.deb)
4. Upload all to the release page

---

## 📝 Version Management

Update version in these files before building:
- `package.json` - `"version": "1.0.0"`
- `frontend/tauri.conf.json` - `"version": "1.0.0"`
- Git tag: `git tag v1.0.0`

---

## 💡 Tips & Best Practices

1. **Always build on the target platform** for best results
2. **Use GitHub Actions** for multi-platform builds
3. **Test thoroughly** before distributing
4. **Keep builds updated** with Tauri updates
5. **Sign code** for production releases

---

## 📚 Resources

- [Tauri Docs](https://tauri.app)
- [Rust Installation](https://rustup.rs)
- [Node.js](https://nodejs.org)
- [WebView2](https://developer.microsoft.com/microsoft-edge/webview2)

---

**Questions?** Check the main [README.md](./README.md) or [DEPLOYMENT.md](./DEPLOYMENT.md)

