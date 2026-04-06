# 🖥️ ScreenLink - Virtual Extended Monitor

> **Transform your second PC into an extended monitor** — Seamless screen sharing with instant connection. Just share a link and extend your display instantly.

**Status**: 🚧 v0.1.0 — Alpha Development  
**Type**: Virtual Monitor/Screen Sharing  
**Platforms**: Windows, Linux, macOS  
**Architecture**: Tauri (Frontend) + Node.js (Backend)

---

## 🎯 What is ScreenLink?

**The Problem:**
- You have 1 monitor and 2 PCs
- You want to use your 2nd PC as an **extended monitor**, not control it
- No complex setup, no multiple windows

**The Solution:**
```
Monitor 1 (Primary PC)
├── Screen A (native)
└── Screen B (2nd PC - appears as extended display)
```

Purpose-built for **screen extension instead of remote control**.

---

## ⚡ Quick Start

### 🖥️ On Primary PC (The one with monitor):
```bash
npm install
npm run app
# → Opens ScreenLink
# → Shows: "Ready to share" + UUID link
```

### 💻 On Secondary PC (The one to extend):
```bash
npm install
npm run app
# → Paste UUID link from primary
# → Your screen now extends to Primary PC's monitor
```

**Result**: You move your mouse off the end of your monitor → appears on the secondary PC's desktop!

---

## 📦 Project Structure

```
screenlinkroot/
├── frontend/                    # Tauri + React App
│   ├── src-tauri/              # Tauri backend (Rust)
│   │   ├── src/
│   │   │   └── main.rs         # Tauri entry point
│   │   └── tauri.conf.json
│   ├── src/                     # React Frontend
│   │   ├── components/
│   │   │   ├── ScreenView.tsx   # Display remote screen
│   │   │   ├── ShareLink.tsx    # Show/copy UUID link
│   │   │   └── Settings.tsx     # Connection settings
│   │   ├── App.tsx
│   │   ├── App.css
│   │   └── main.tsx
│   ├── package.json
│   └── tauri.conf.json
│
├── backend/                     # Node.js + WebSocket Server
│   ├── src/
│   │   ├── server.ts           # Express + WebSocket
│   │   ├── socket-handler.ts   # WebSocket events
│   │   ├── screen-capture.ts   # H.264 video capture
│   │   ├── link-manager.ts     # UUID link generation
│   │   └── utils.ts
│   ├── package.json
│   ├── tsconfig.json
│   └── .env.example
│
├── shared/                      # Shared types & protocols
│   ├── types.ts                # TypeScript interfaces
│   └── constants.ts            # Port, timeouts, etc
│
├── docs/                        # Documentation
│   ├── ARCHITECTURE.md         # System design
│   ├── PROTOCOL.md             # WebSocket protocol
│   ├── SETUP.md                # Development setup
│   └── STUDY_GUIDE.md          # For learning the codebase
│
├── .gitignore
├── package.json                 # Root workspace config
└── README.md                    # This file
```

---

## 🚀 Key Features

- ✅ **Virtual Extended Monitor** - 2nd PC appears as monitor extension
- ✅ **UUID-based Sharing** - Share via instant links
- ✅ **H.264 Video Stream** - Efficient video compression
- ✅ **Low Latency** - <100ms typical on LAN
- ✅ **Firewall Proof** - Works through NAT/firewalls
- ✅ **Multiple Screens** - Support 2+ displays
- ✅ **Cross-platform** - Windows, Linux, macOS
- ✅ **No Installation** - Portable executable

---

## 🛠️ Tech Stack

### Frontend
- **Tauri** - Cross-platform desktop framework
- **React 18** - UI library
- **TypeScript** - Type safety
- **Vite** - Fast build tool

### Backend
- **Node.js** - JavaScript runtime
- **Express** - Web framework
- **WebSocket** - Real-time communication
- **TypeScript** - Type safety

### Video Encoding
- **libx264** - H.264 video encoding
- **FFmpeg** - Video processing

### Deployment
- GitHub Releases - Binary distribution
- **Size Target**: <100MB per platform
- **Auto-update**: Built-in update checker

---

## 📋 Development Setup

### Prerequisites
```bash
# Node.js 18+
node --version

# Rust (for Tauri)
rustc --version

# FFmpeg (for video encoding)
ffmpeg -version
```

### Install Dependencies
```bash
# Root
npm install

# Frontend
cd frontend && npm install && cd ..

# Backend
cd backend && npm install && cd ..
```

### Run Development
```bash
# Terminal 1: Start backend
cd backend
npm run dev

# Terminal 2: Start frontend (Tauri dev)
cd frontend
npm run tauri dev
```

### Build Release
```bash
# Build everything
npm run build

# Output: dist/
# - frontend/bundle/ (web assets)
# - frontend/target/release/ (Tauri executables)
# - backend/dist/ (compiled backend)
```

---

## 🔌 Protocol Overview

### Connection Flow

```
Primary PC                          Secondary PC
┌─────────────┐                    ┌──────────────┐
│   Tauri     │                    │   Tauri      │
│   Frontend  │                    │   Frontend   │
└──────┬──────┘                    └──────┬───────┘
       │ WebSocket                        │
       ├──────────────────────────────────┤
       │                                  │
     Node.js Backend (Primary)      Node.js Backend (Secondary)
     ┌─────────────────────┐        ┌──────────────────────┐
     │ Screen Capture      │        │ Screen Renderer      │
     │ H.264 Encoder       │        │ Display Manager       │
     │ WebSocket Server    │        │ WebSocket Client     │
     └─────────────────────┘        └──────────────────────┘
```

### Messages

**Share Link Generation:**
```json
{
  "type": "SHARE_LINK",
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "code": "ABCD-1234",
  "expires": "2026-04-07T14:28:00Z"
}
```

**Video Stream:**
```
WebSocket Binary: [H.264 NAL Units]
- Keyframes: Every 2 seconds
- Frame rate: 30-60 FPS
- Resolution: Matches remote display
- Bitrate: Adaptive (5-20 Mbps)
```

---

## 📖 Documentation

- **[ARCHITECTURE.md](./docs/ARCHITECTURE.md)** - System design & flow
- **[PROTOCOL.md](./docs/PROTOCOL.md)** - WebSocket protocol details
- **[SETUP.md](./docs/SETUP.md)** - Development environment
- **[STUDY_GUIDE.md](./docs/STUDY_GUIDE.md)** - Learning guide for codebase

---

## 🎓 For Learning

This project teaches:
- ✅ Desktop app development (Tauri + React)
- ✅ Real-time communication (WebSocket)
- ✅ Video encoding & streaming (H.264)
- ✅ Cross-platform development
- ✅ Full-stack TypeScript architecture

See **[STUDY_GUIDE.md](./docs/STUDY_GUIDE.md)** for detailed learning path.

---

## 📊 Performance Targets

| Metric | Target |
|--------|--------|
| Latency | <100ms (LAN) |
| FPS | 30-60 |
| Bandwidth | 5-20 Mbps |
| CPU Usage | <20% (primary), <30% (secondary) |
| Memory | <200MB per instance |
| Binary Size | <100MB |

---

## 🚧 Roadmap

### v0.1 (Current)
- [ ] Basic screen sharing
- [ ] UUID link generation
- [ ] H.264 video stream
- [ ] Cross-platform build

### v0.2
- [ ] Mouse/keyboard sync (optional)
- [ ] Multiple monitors
- [ ] Audio sharing
- [ ] Recording capability

### v1.0
- [ ] Production ready
- [ ] Auto-update system
- [ ] Web dashboard
- [ ] Mobile app support

---

## 🤝 Contributing

Clone, contribute, and submit PRs!

```bash
git clone https://github.com/Alexandre-Gervasio/screenlink.git
cd screenlink
npm install
npm run dev
```

---

## 📄 License

MIT License - Free for commercial use with attribution.

---

**Made for productivity** 🎯 | **Performance first** ⚡ | **Open source** 🚀

**GitHub**: [Alexandre-Gervasio/screenlink](https://github.com/Alexandre-Gervasio/screenlink)  
**Version**: 0.1.0 (Alpha)  
**Last Updated**: April 6, 2026
