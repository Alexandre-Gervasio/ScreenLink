# 🚀 KVM Pro - Quick Start Guide

Bem-vindo ao KVM Pro! Aqui estão as instruções para começar.

## 👥 For End Users (No installation needed!)

**Just download and run:**

### Linux
```bash
tar xzf kvm-pro-linux-x64-portable-*.tar.gz
cd kvm-pro-linux-*
./run-server.sh
```

### Windows
```
Extract ZIP file
Double-click run-server.bat
```

**That's it!** See [USER_GUIDE.md](USER_GUIDE.md) for detailed instructions.

---

## 👨‍💻 For Developers (Setting up development environment)

### 1️⃣ Install Rust (first time only)

#### Linux/macOS:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Windows:
Download: https://rustup.rs/

Then in PowerShell:
```powershell
rustup update
```

### 2️⃣ Clone the Project

```bash
cd ~/Documentos/DEV/kvm-pro
```

### 3️⃣ Install Dependencies

#### Linux (Debian/Ubuntu):
```bash
chmod +x scripts/setup.sh
./scripts/setup.sh
```

#### Linux (Fedora):
```bash
sudo dnf install libevdev-devel pkg-config openssl-devel
```

#### macOS:
```bash
brew install libevdev
```

#### Windows:
Use Visual Studio Build Tools or MinGW

### 4️⃣ Build for Development

```bash
cd core
cargo build --release
```

### 5️⃣ Run Locally

#### Terminal 1 - SERVIDOR:
```bash
RUST_LOG=info ./target/release/kvm-pro-server
```

#### Terminal 2 - CLIENTE:
```bash
RUST_LOG=info ./target/release/kvm-pro-client
```

---

## 📦 Build Portable Packages (Distribution)

### For Linux (fully static, no dependencies)

```bash
chmod +x scripts/portable-build.sh
./scripts/portable-build.sh linux
```

Creates: `dist/kvm-pro-linux-x64-portable-VERSION.tar.gz`

### For Windows (portable ZIP)

```bash
chmod +x scripts/windows-build.sh
./scripts/windows-build.sh
```

Creates: `dist/kvm-pro-windows-x64-portable-VERSION.zip`

### Build Everything at Once

```bash
chmod +x scripts/release-build.sh
./scripts/release-build.sh
```

Choose option 4 for full release.

---

## 📖 Documentation

- **README.md**: Documentação completa
- **USER_GUIDE.md**: Para usuários finais
- **CONTRIBUTING.md**: Como contribuir
- **IMPLEMENTATION_SUMMARY.md**: O que foi implementado
- **TODO.md**: Roadmap e próximos passos

---

## 🔧 Troubleshooting

### Development Issues

**"Cargo command not found"**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**"libevdev not found" (Linux)**
```bash
# Ubuntu/Debian
sudo apt-get install libevdev-dev

# Fedora
sudo dnf install libevdev-devel
```

**"Permission denied /dev/input/event0"**
```bash
sudo usermod -a -G input $USER
# Log out and log back in
```

### Distribution Issues

**Binary doesn't run on other computers**
- Use the portable build scripts (they create static binaries)
- Download packages from the dist/ folder

**"Port already in use"**
- Edit `kvm-pro.toml` and change port number

---

## 📊 Project Structure

```
core/src/
├── main.rs           ← Server
├── client.rs         ← Client
├── input/            ← Capture/inject events
├── network/          ← TCP/UDP
├── security/         ← TLS
├── config/           ← Configuration
├── plugins/          ← Plugin system
└── ...

scripts/
├── build.sh          ← Development build
├── portable-build.sh ← Static portable build
├── release-build.sh  ← Full release
└── setup.sh          ← Dev dependencies
```

---

## 🎯 For Quick Release Build

```bash
# One-command portable build
bash scripts/release-build.sh

# Choose option 4 (Full release packages)

# Result: dist/ folder with all packages ready to distribute
```

Users can then download and run - **NO INSTALLATION NEEDED!**

---

## 💡 Key Concepts

### Development Build
- Depends on system libraries
- Smaller binary size
- Faster compilation
- Good for testing

### Portable Build (Release)
- Fully static - no dependencies
- Works anywhere
- Larger binary size
- Ready to distribute

---

## 🚀 Next Steps

**Development:** Continue in CONTRIBUTING.md

**Release:** Use scripts/release-build.sh

**Share:** Upload dist/ files to your users

---

**Happy coding!** 🎉

Questions? Check [README.md](README.md) or [USER_GUIDE.md](USER_GUIDE.md)
