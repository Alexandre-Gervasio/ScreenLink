# 📚 ScreenLink - Complete Study Guide

**This document is your complete learning resource for the ScreenLink codebase.**

## 🎯 What You'll Learn

By studying this guide, you'll understand:
- Full-stack TypeScript architecture
- Desktop app development with Tauri + React
- Real-time WebSocket communication
- Video streaming fundamentals
- Cross-platform development

---

## 📖 Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Component Deep Dive](#component-deep-dive)
4. [Protocol & Communication](#protocol--communication)
5. [Video Streaming](#video-streaming)
6. [Learning Path](#learning-path)
7. [Common Tasks](#common-tasks)

---

## Project Overview

### What is ScreenLink?

**Before:** You have 1 monitor and 2 PCs. You can only control 1 at a time.

```
PC 1 (Laptop)                PC 2 (Desktop)
┌──────────────┐            ┌──────────────┐
│              │            │              │
│  1 Monitor   │            │  No monitor  │
│              │            │              │
└──────────────┘            └──────────────┘
```

**After ScreenLink:** PC 2 becomes a virtual extended monitor!

```
PC 1 (Laptop)                PC 2 (Desktop - ScreenLink)
┌──────────────┐            ┌──────────────┐
│ Primary PC   │            │ Extended     │
│ + Monitor 1  │ ◄─────────► │ Display      │
│              │  WebSocket  │ (Your apps)  │
└──────────────┘            └──────────────┘
```

### Core Features

1. **Virtual Monitor Extend** - PC 2 acts as Monitor 2
2. **UUID-based Links** - Share via unique codes, not IPs
3. **H.264 Streaming** - Efficient video compression
4. **Bidirectional** - Both PCs see each other's screens
5. **No Installation** - Single portable .exe/.app/.bin

---

## Architecture

### System Design

```
┌─────────────────────────────────────────────────────────────┐
│                    PRIMARY PC (Laptop)                      │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Tauri Window (React UI)                             │  │
│  │  ┌────────────────┐  ┌──────────────────┐           │  │
│  │  │ Share Link UI  │  │ Remote Display   │           │  │
│  │  │ Code: ABC123   │  │ (Secondary PC)   │           │  │
│  │  └────────────────┘  └──────────────────┘           │  │
│  └──────────────────────────────────────────────────────┘  │
│                          ▲                                  │
│                          │                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Node.js Backend (Port 3001 + 3002)                  │  │
│  │  ┌─────────────┐  ┌──────────┐  ┌──────────────┐    │  │
│  │  │ Express API │  │ WebSocket│  │ Screen Cap   │    │  │
│  │  │ /links/     │  │ Handler  │  │ (ffmpeg)     │    │  │
│  │  └─────────────┘  └──────────┘  └──────────────┘    │  │
│  └──────────────────────────────────────────────────────┘  │
│                          ▲                                  │
└──────────────────────────┼──────────────────────────────────┘
                           │
                   WebSocket Binary
                   H.264 Video Stream
                           │
┌──────────────────────────┼──────────────────────────────────┐
│                          ▼                                  │
│                    SECONDARY PC                            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Tauri Window (React UI)                             │  │
│  │  ┌────────────────┐  ┌──────────────────┐           │  │
│  │  │ Connect UI     │  │ Display Renderer │           │  │
│  │  │ Code: ABC123   │  │ (Your Desktop)   │           │  │
│  │  └────────────────┘  └──────────────────┘           │  │
│  └──────────────────────────────────────────────────────┘  │
│                          ▲                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Node.js Backend (Port 3001 + 3002)                  │  │
│  │  ┌─────────────┐  ┌──────────┐  ┌──────────────┐    │  │
│  │  │ Express API │  │ WebSocket│  │ Display Mgmt │    │  │
│  │  │ /health     │  │ Listener │  │ (Display API)│    │  │
│  │  └─────────────┘  └──────────┘  └──────────────┘    │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

```
1. Primary PC Start:
   - Tauri window opens
   - React renders "I'm the Main PC" button
   - User clicks → Node.js backend generates UUID link
   - Displays 6-char code (ABC-123)

2. Secondary PC Start:
   - Tauri window opens
   - User clicks "I'm the Extended Screen"
   - Enters code → Connects to Primary via WebSocket
   - Sends CONNECT message

3. Video Streaming Loop:
   - Primary captures screen every 33ms (30 FPS)
   - Encodes to H.264 via ffmpeg
   - Sends binary WebSocket packets
   - Secondary receives & decodes
   - Displays on screen

4. Mouse/Keyboard (Future):
   - Secondary sends INPUT events
   - Primary receives & injects
   - Bidirectional control possible
```

---

## Component Deep Dive

### Frontend (Tauri + React)

#### **App.tsx** - Main Component

```tsx
// Three modes:
1. 'home'      - Choose role (Primary or Secondary)
2. 'primary'   - Show share link
3. 'secondary' - Connect to primary
```

**Key Functions:**
```tsx
generateShareLink()     // Calls API, gets UUID + code
startPrimaryMode()      // Start sharing screen
connectToSecondary()    // Join existing link
```

**State Management:**
```tsx
mode              // Current mode
shareLink         // Full link URL
shareCode         // 6-char code for sharing
connected         // WebSocket connected?
remoteScreen      // Base64 video data
```

#### **Component Tree**

```
App.tsx
├── HomeScreen
│   ├── Header
│   └── Options (2 buttons)
├── PrimaryScreen
│   ├── Header
│   ├── ShareBox (copy code)
│   └── Status
└── SecondaryScreen
    ├── Header
    ├── ConnectBox (input code)
    └── RemoteDisplay (video)
```

### Backend (Node.js + Express)

#### **server.ts** - Main Server

```typescript
// Three main jobs:
1. REST API        - Generate links, check status
2. WebSocket       - Real-time communication
3. Video Bridge    - Forward H.264 streams
```

**REST Endpoints:**

```
POST /api/links/create
├─ Returns: { uuid, code, shareUrl, expiresIn }
└─ Used by: Primary PC to create link

GET /api/links/:uuid
├─ Returns: { uuid, code, active, primaryConnected, secondaryConnected }
└─ Used by: Status checks

GET /health
├─ Returns: { status, activeSessions }
└─ Used by: Health monitoring
```

**WebSocket Events:**

```
Connected:
Primary   → SHARE_LINK_GENERATED
Secondary → CONNECTED

Video Streaming:
Primary   → VIDEO_FRAME (binary H.264 data)
Secondary ← VIDEO_FRAME (received)

Input (Future):
Secondary → INPUT (mouse/keyboard events)
Primary   ← INPUT (received)

Disconnected:
Either → PEER_DISCONNECTED
```

---

## Protocol & Communication

### Link Structure

```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "code": "ABC123",
  "created": "2026-04-06T14:00:00Z",
  "expires": "2026-04-07T14:00:00Z",
  "primaryPC": "primary-550e8400",
  "secondaryPC": "secondary-550e8400",
  "active": true
}
```

**Link Lifecycle:**
```
Created (24h validity)
   ↓
Primary connects (gets UUID & code)
   ↓
Shares code via chat/QR/link
   ↓
Secondary enters code
   ↓
WebSocket handshake
   ↓
Video streaming begins
   ↓
Connection closes OR expires
```

### WebSocket Message Format

**Text Messages (JSON):**
```json
{
  "type": "CONNECTED|CONTROL|PEER_DISCONNECTED",
  "message": "string",
  "timestamp": "ISO8601",
  "data": {}
}
```

**Binary Messages (Video):**
```
[H.264 NAL Unit Stream]
- H.264 encoded video frame
- Raw binary WebSocket packet
- Sent 30-60 times per second
```

**Message Routing:**
```
Primary sends VIDEO_FRAME
    ↓
Backend receives via WebSocket
    ↓
Backend checks: is Secondary connected?
    ↓
If YES: Forward binary to Secondary
    ↓
Secondary receives VIDEO_FRAME
    ↓
Secondary decodes H.264 → canvas
    ↓
Display updated on screen
```

---

## Video Streaming

### H.264 Encoding Strategy

**Why H.264?**
- ✅ Industry standard (WebRTC, RTMP, HLS)
- ✅ Excellent compression
- ✅ Low latency
- ✅ Widely supported
- ✅ Mature library ecosystem

**Encoding Pipeline:**

```
Screen Capture (30ms interval)
    ↓
ffmpeg process
    ↓
Raw YUV420 (libx264 encodes)
    ↓
H.264 NAL Units
    ↓
WebSocket binary send
    ↓
Secondary receives
    ↓
Decode (ffmpeg or libav)
    ↓
Render to Canvas/Display
```

**Performance Tuning:**

```
FPS:      30 (default) to 60 (if LAN is good)
Preset:   ultrafast (low latency > quality)
Bitrate:  8000k-20000k (adaptive based on network)
Keyframe: Every 2 seconds (I-frame for sync)
```

**Bandwidth Calculation:**
```
1920x1080 @ 30FPS @ 8Mbps ≈ 1 MB/s
24-hour recording ≈ 86 GB (too much!)

With H.264:
1920x1080 @ 30FPS ≈ 1-3 Mbps typical
24-hour ≈ 400 GB - 1 TB (reasonable for SSD)
```

---

## Learning Path

### Week 1: Foundations

**Day 1-2: TypeScript Basics**
```typescript
// Key concepts for this project:
interface  // Define types
type       // Type aliases
async/await // Async operations
```

**Files to study:**
- `backend/src/server.ts` (lines 1-50)
- `frontend/src/App.tsx` (imports & types)

**Day 3-4: Frontend Setup**
- Tauri basics
- React hooks (useState, useRef, useEffect)
- CSS Grid/Flexbox

**Files to study:**
- `frontend/src/App.tsx` (JSX structure)
- `frontend/src/App.css` (layout)

**Day 5-7: Backend Setup**
- Express fundamentals
- WebSocket lifecycle
- REST API patterns

**Files to study:**
- `backend/src/server.ts` (main)

### Week 2: Core Features

**Day 1-3: Link Generation**
```typescript
// Generate UUID + validation code
// Store in memory (later: database)
// Include expiry logic
```

**Files to study:**
- `backend/src/server.ts` (generateShareLink function)

**Day 4-7: WebSocket Communication**
```typescript
// Connect primary + secondary
// Route video frames
// Handle disconnects
```

**Files to study:**
- `backend/src/server.ts` (wsApp.ws route)

### Week 3-4: Video Streaming

**Advanced Topics:**
- H.264 encoding internals
- ffmpeg integration
- Performance optimization
- Error handling

---

## Common Tasks

### How to Add a New Feature?

**Example: Add desktop notifications**

1. **Frontend (React):**
```tsx
// frontend/src/App.tsx
const showNotification = (message: string) => {
  // Call Tauri notification API
  // Show toast in UI
};
```

2. **Backend (Express):**
```typescript
// backend/src/server.ts
ws.on('message', (data) => {
  if (message.type === 'NOTIFY') {
    // Log notification
    console.log('Notification:', message.text);
  }
});
```

3. **Frontend → Backend:**
```tsx
// In App.tsx, when user clicks button:
ws.send(JSON.stringify({
  type: 'NOTIFY',
  text: 'Hello from Primary!',
}));
```

### How to Debug?

**Browser DevTools (Frontend):**
```bash
# In Tauri dev mode, press F12
# See Console, Network, React DevTools
```

**Node.js Debugger (Backend):**
```bash
# Terminal
node --inspect-brk dist/server.js

# Chrome
chrome://inspect
```

**WebSocket Debugging:**
```bash
# Use 'websocat' tool
websocat ws://localhost:3002

# Send test messages:
{"type":"PING"}
```

### How to Build/Deploy?

**Development:**
```bash
npm run dev              # Both frontend + backend
npm run app             # Just frontend
cd backend && npm run dev # Just backend
```

**Production:**
```bash
npm run build                    # Compile everything
# Output:
# - frontend/target/release/   (Tauri bundles)
# - backend/dist/               (Node.js server)
```

**Shipping Binary:**
```bash
# Create installer for each platform
# Windows: .msi, .exe
# macOS: .dmg, .app
# Linux: .AppImage, .deb
```

---

## 🎓 Next Steps

1. **Read** this guide 2-3 times
2. **Study** code files in order:
   - `backend/src/server.ts` (understand flow)
   - `frontend/src/App.tsx` (understand UI)
   - `frontend/src/App.css` (understand layout)
3. **Modify** features (add buttons, colors, etc)
4. **Build** new features (notifications, settings, etc)
5. **Deploy** your version

---

## 📞 Reference

- **Tauri Docs**: https://tauri.app/docs
- **React Docs**: https://react.dev
- **WebSocket MDN**: https://developer.mozilla.org/en-US/docs/Web/API/WebSocket
- **H.264 Overview**: https://en.wikipedia.org/wiki/Advanced_Video_Coding
- **FFmpeg**: https://ffmpeg.org/

---

**Happy Learning!** 🚀
