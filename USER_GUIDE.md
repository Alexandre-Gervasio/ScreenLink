# 🚀 KVM Pro - User Guide (End User)

**Welcome to KVM Pro!** This is a portable application that requires **NO installation whatsoever**.

## 📥 Linux Installation (Choose One)

### Option 1: Using the AppImage (Easiest)

```bash
# Download KVM_Pro-VERSION-x86_64.AppImage
chmod +x KVM_Pro-*.AppImage
./KVM_Pro-*.AppImage  # Server starts automatically
```

It's that simple! The AppImage is completely self-contained.

### Option 2: Using the Portable Package

```bash
# Download kvm-pro-linux-x64-portable-VERSION.tar.gz
tar xzf kvm-pro-linux-x64-portable-*.tar.gz
cd kvm-pro-linux-*

# Start server
./run-server.sh

# (In another terminal)
./run-client.sh
```

## 💻 Windows Installation

```bash
# 1. Download kvm-pro-windows-x64-portable-VERSION.zip
# 2. Right-click → Extract All
# 3. Double-click FIRST_RUN.bat
# 4. Choose what you want to do
```

That's it! No Administrator privileges needed.

## 🎮 Usage

### Server (The computer you want to control FROM)

**Linux:**
```bash
./run-server.sh
```

**Windows:**
```
Double-click run-server.bat
```

You'll see something like:
```
[INFO] Starting KVM Pro Server
[INFO] KVM Pro server listening on 0.0.0.0:5000
```

### Client (The computer you want to control)

**Linux:**
```bash
./run-client.sh
```

**Windows:**
```
Double-click run-client.bat
```

You'll see:
```
[INFO] Starting KVM Pro Client
[INFO] Connected to server at 127.0.0.1:5000
```

Now move your mouse over the edge and it will appear on the other computer!

## ⚙️ Configuration (Advanced)

If you need to change the port or other settings:

1. Find `kvm-pro.toml.example` in the folder
2. Copy it: `cp kvm-pro.toml.example kvm-pro.toml`
3. Edit with your favorite text editor
4. Save and restart

## 🆘 Troubleshooting

### "Port already in use"
**Solution:** Edit `kvm-pro.toml` and change the port from 5000 to something else like 5001.

### Server runs but client can't connect
**Solution:** Check your firewall allows port 5000 (or your custom port).

### On Linux: "Permission denied" errors
**Solution:** Run this once:
```bash
sudo usermod -a -G input $USER
# Logout and login
```

### Need to run from a specific directory
**Solution:** Create a symlink or alias:
```bash
# Linux
ln -s /path/to/kvm-pro-server ~/kvm-server

# Then run:
~/kvm-server
```

## 📊 System Requirements

### Linux
- Any recent Linux distribution x86_64
- No additional libraries needed (fully static)
- Terminal to run commands

### Windows
- Windows 10 or 11 (x86_64)
- No Administrator required
- Can run from USB stick

## 💾 Disk/Memory Usage

- **Disk Space:** ~10-20 MB per binary
- **Memory:** ~50-100 MB when running
- **CPU:** Minimal (only when capturing/sending events)

## 🔌 Network Requirements

- Works on any local network
- Minimum bandwidth: 1 Mbps
- Typical latency: < 50ms (depending on network)

## 🚪 Firewall

You may need to allow the KVM Pro application through your firewall:

**Windows Firewall:** Should prompt on first run

**Linux:** If you use UFW:
```bash
sudo ufw allow 5000/tcp
```

## 🔒 Security Note

Version 0.1.0 does NOT have encryption. For production use:
- Run on trusted networks only
- Or wait for TLS version in a future update

## 📁 What's in the folder?

```
kvm-pro-server       - Server executable (or .exe on Windows)
kvm-pro-client       - Client executable
run-server.sh/bat    - Easy launcher for server
run-client.sh/bat    - Easy launcher for client
kvm-pro.toml.example - Configuration template
README.txt           - This file
LICENSE              - MIT license
```

## ✨ Features

✅ Keyboard sharing between computers
✅ Mouse movement between computers
✅ Mouse click/button support
✅ Scroll wheel support
✅ Low latency (< 50ms typical)
✅ Lightweight (no bloat)
✅ Works over network
✅ Fully portable

## 🔮 Future Features

- Clipboard sync
- Screen sharing
- Multi-monitor support
- TLS encryption
- Web UI

## 📞 Getting Help

1. Check the README in the folder
2. Try the troubleshooting section above
3. Check system firewall settings
4. Ensure both computers are on same network

## 🙏 Feedback

Have ideas or found a bug? Let me know!

---

**That's it! Enjoy KVM Pro!** 🎉

**No installation. Just download and run.**
