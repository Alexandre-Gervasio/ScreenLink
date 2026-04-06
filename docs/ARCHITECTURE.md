# 🏗️ ScreenLink - System Architecture

## High-Level Overview

ScreenLink is a **virtual monitor extension application** that transforms a second PC into an extended display via WebSocket streaming.

```
┌─────────────────────┐         ┌─────────────────────┐
│   PRIMARY PC        │         │  SECONDARY PC       │
│  (With Monitor)     │         │  (Extended Display) │
│                     │         │                     │
│  ┌───────────────┐  │         │  ┌───────────────┐  │
│  │ Tauri + React │──┼─────────┼─→│ Tauri + React │  │
│  │  (UI/Display) │  │         │  │  (Display)    │  │
│  └───────┬───────┘  │         │  └───────┬───────┘  │
│          │          │         │          │          │
│  ┌───────▼───────┐  │         │  ┌───────▼───────┐  │
│  │ Node.js + Exp │  │         │  │ Node.js + Exp │  │
│  │ (3001, 3002)  │◄─┼─────────┼─→│ (3001, 3002)  │  │
│  │               │  │         │  │               │  │
│  │ • REST API    │  │         │  │ • REST API    │  │
│  │ • WebSocket   │  │         │  │ • WebSocket   │  │
│  │ • Screen Cap  │  │         │  │ • Display Mgr │  │
│  └───────────────┘  │         │  └───────────────┘  │
│                     │         │                     │
└─────────────────────┘         └─────────────────────┘
```

---

## Component Architecture

### Frontend (Tauri + React)

**Technology Stack:**
- Tauri 2.0 (Rust-based desktop framework)
- React 18 (UI library)
- TypeScript (type safety)
- Vite (build tool)

**Directory Structure:**
```
frontend/
├── src/
│   ├── App.tsx          # Main component (3 screens)
│   ├── App.css          # Styling
│   ├── main.tsx         # React entry point
│   └── index.css        # Global styles
├── src-tauri/
│   ├── src/
│   │   └── main.rs      # Tauri entry point
│   └── tauri.conf.json  # Tauri configuration
├── vite.config.ts       # Vite configuration
├── tsconfig.json        # TypeScript config
└── package.json         # Dependencies
```

**Three Screens (States):**

1. **Home Screen** (`mode: 'home'`)
   - Two buttons: "I'm Main PC" or "I'm Extended Screen"
   - Entry point for choosing role

2. **Primary Screen** (`mode: 'primary'`)
   - Displays 6-character share code
   - Shows "Copy link" button
   - Displays connection status
   - Shows "Waiting for connection..." state

3. **Secondary Screen** (`mode: 'secondary'`)
   - Input field for share code
   - "Connect" button
   - Video container (displays remote screen)
   - Shows connection status

**State Management:**
```tsx
// App.tsx state:
const [mode, setMode] = useState<'home' | 'primary' | 'secondary'>('home');
const [shareLink, setShareLink] = useState<string>('');
const [shareCode, setShareCode] = useState<string>('');
const [connected, setConnected] = useState(false);
const [remoteScreen, setRemoteScreen] = useState<string>('');
const wsRef = useRef<WebSocket | null>(null);
```

**API Calls:**
```typescript
// Generate share link
POST /api/links/create
→ Returns: { uuid, code, shareUrl }

// Get link status
GET /api/links/:uuid
→ Returns: { uuid, code, active, primaryConnected, secondaryConnected }
```

---

### Backend (Node.js + Express)

**Technology Stack:**
- Node.js 18+ (runtime)
- Express (HTTP server)
- express-ws (WebSocket support)
- TypeScript

**Directory Structure:**
```
backend/
├── src/
│   └── server.ts        # Main server logic
├── dist/                # Compiled JavaScript
├── package.json         # Dependencies
├── tsconfig.json        # TypeScript config
├── .env.example         # Environment template
└── .env                 # (git-ignored)
```

**Server Structure:**

```typescript
// Main components:

1. Express App
   ├── REST Routes
   │   ├── POST /api/links/create
   │   ├── GET /api/links/:uuid
   │   └── GET /health
   └── Error handlers

2. WebSocket Server
   ├── ws://:3002/ws/:uuid/:type/:code
   ├── Message router
   ├── Session manager
   └── Video forwarder

3. In-Memory Store
   ├── ShareLinks Map<uuid, LinkData>
   ├── ActiveSessions Map<sessionId, ClientData>
   └── Cleanup on expire/disconnect
```

**Data Models:**

```typescript
interface ShareLink {
  uuid: string;
  code: string;
  created: Date;
  expires: Date;
  primaryPC: string;
  secondaryPC?: string;
  active: boolean;
}

interface ClientSession {
  type: 'primary' | 'secondary';
  uuid: string;
  ws: WebSocket;
  connected: boolean;
}

interface Message {
  type: 'CONNECTED' | 'VIDEO_FRAME' | 'CONTROL' | 'PEER_DISCONNECTED';
  [key: string]: any;
}
```

---

## Communication Protocol

### Link Generation Flow

```
User clicks "I'm Main PC"
    ↓
Frontend calls: POST /api/links/create
    ↓
Backend generates:
  - UUID (550e8400-...)
  - Code (ABC123)
  - Expiry (24 hours)
    ↓
Returns to Frontend:
  {
    uuid: "550e8400...",
    code: "ABC123",
    shareUrl: "screenlink://connect/ABC123"
  }
    ↓
Frontend displays code + copy button
```

### Connection Handshake

```
PRIMARY PC:
1. Click "I'm Main PC"
2. Get code → Display to user
3. Shares code via chat/QR/link

SECONDARY PC:
1. Click "I'm Extended Screen"
2. Enter code → Click Connect
3. Frontend connects to:
   ws://primary-ip:3002/ws/{uuid}/{type}/{code}

BACKEND:
1. Validates UUID + code + type
2. Creates session
3. Stores in activeSessions
4. Sends CONNECTED message
   
BOTH:
- Exchange VIDEO_FRAME messages
- Monitor peer connection
- Handle disconnect
```

### Message Types

**Text Messages (JSON):**

```json
// Connection established
{
  "type": "CONNECTED",
  "message": "Connected as primary to ABC123",
  "timestamp": "2026-04-06T14:00:00Z"
}

// Keyboard/Mouse input (future)
{
  "type": "INPUT",
  "input": { "x": 100, "y": 200, "button": "left" },
  "timestamp": "2026-04-06T14:00:00:001Z"
}

// Peer disconnected
{
  "type": "PEER_DISCONNECTED",
  "peer": "primary"
}
```

**Binary Messages (Video):**

```
WebSocket.send(h264EncodedFrame)
- Raw H.264 NAL Units
- Binary protocol (no JSON overhead)
- ~30-60 frames per second
- Size: 50KB - 500KB per frame depending on resolution
```

### Session Lifecycle

```
1. CREATED
   - Link generated with 24h expiry
   - Stored in memory

2. PRIMARY CONNECTED
   - Primary PC calls generateShareLink()
   - Gets UUID + code
   - Enters Primary mode

3. BOTH CONNECTED
   - Secondary connects with UUID + code
   - WebSocket handshake complete
   - Video streaming begins

4. STREAMING
   - Primary sends VIDEO_FRAME messages
   - Secondary receives and renders
   - Heartbeat checks (30s intervals)

5. DISCONNECTED
   - Either peer closes connection
   - Notify other peer
   - Clean up session
   - Link still valid for 24h

6. EXPIRED
   - 24h passes
   - Link becomes inactive
   - New connection rejected
```

---

## Data Flow Diagrams

### Video Streaming Pipeline

```
PRIMARY PC:

Screen Capture (every 33ms)
    ↓ (800x600 pixels)
Frame Buffer
    ↓
Node.js Process
    ↓ (spawn ffmpeg)
FFmpeg Encoder
    ↓ (libx264 @ ultrafast)
H.264 Stream
    ↓
NAL Units
    ↓
WebSocket Binary Message
    ↓
Network (1-3 Mbps)
    ↓
SECONDARY PC
    ↓
WebSocket Receive
    ↓
NAL Units Buffer
    ↓
Decoder (ffmpeg or libav)
    ↓
Canvas/Display Element
    ↓
Rendered on Screen
```

### Control Message Flow (Future)

```
SECONDARY PC:
User moves mouse → 100px right
    ↓
React captures EVENT
    ↓
Send INPUT message:
{
  "type": "INPUT",
  "x": 100,
  "y": 50,
  "action": "move"
}
    ↓
PRIMARY PC Backend receives
    ↓
Injects input to OperatingSystem
    ↓
Mouse moves 100px right on Primary
```

---

## Error Handling Strategy

### Connection Errors

```
Invalid Code:
  ↓ Backend validates
  ↓ ws.close(4001, "Invalid code")
  ↓ Frontend shows error

Link Expired:
  ↓ Backend checks timestamp
  ↓ ws.close(4002, "Link expired")
  ↓ Frontend suggests new link

Network Disconnected:
  ↓ WebSocket onerror event
  ↓ Frontend reconnects with exponential backoff
  ↓ Shows "Reconnecting..." UI
```

### Recovery Mechanisms

```
1. Heartbeat System
   - Every 30 seconds
   - Send ping-like message
   - Detect stale connections
   - Auto-reconnect if needed

2. Buffer Management
   - Store I-frames (keyframes)
   - Memory limit: 50MB max
   - Drop old frames if buffer full

3. Graceful Degradation
   - If video quality drops, auto-downscale
   - If bandwidth low, reduce FPS
   - Show UI indicators of issues
```

---

## Performance Considerations

### Memory Usage

```
Primary PC:
- Tauri window: ~50MB
- Node.js process: ~100MB
- Screen buffer: ~20MB
- Total: ~170MB typical

Secondary PC:
- Tauri window: ~50MB
- Node.js process: ~100MB
- Video buffer: ~50MB
- Canvas cache: ~10MB
- Total: ~210MB typical
```

### Network Usage

```
Resolution: 1920x1080
FPS: 30
Bitrate: 8Mbps typical

Calculation:
- 8 Mbps = 1 MB/sec
- Per hour: 3,600 MB = 3.6 GB
- 24 hours: 86.4 GB

Optimization:
- H.264 compression ratio: 50:1
- Uncompressed would be: ~4,320 GB/day
- Actual: ~86 GB/day = 95% saved!
```

### Latency Budget

```
Total E2E Latency Target: 100ms (acceptable for mouse)

Breakdown:
- Screen capture: 5ms
- H.264 encode: 20ms
- Network transmission: 30ms (LAN)
- WebSocket recv: 5ms
- H.264 decode: 20ms
- Render to display: 10ms
- Display refresh: 16ms (60Hz)
_________________________
Total: ~106ms (reasonable)
```

---

## Scaling Considerations

### Current Limitations

```
✅ Single link per Primary PC
✅ One Secondary PC per Primary
❌ No multi-screen support
❌ No audio
❌ No recording
❌ In-memory store (lost on restart)
```

### Future Improvements

```
v0.2:
- Multiple secondary displays per primary
- Audio streaming (WebAudio API)
- Session recording
- Database for history

v1.0:
- Redundant servers (HA)
- Relay servers for NAT traversal
- Mobile app support
- Cloud synchronization
```

---

## Security Architecture

### Current

```
✅ 24hour link expiry
✅ Random UUID + code
✅ WebSocket over TLS (local dev: plaintext)
❌ No authentication
❌ No encryption (full screen visible)
```

### Roadmap

```
v1.0:
- TLS/SSL enforcement
- Per-link PIN codes
- Screen redaction zones
- Audit logging
- Session timeout from inactivity
```

---

## Deployment Model

### Single Machine (Development)

```
One PC runs:
- Primary Frontend + Backend
- Secondary Frontend + Backend (simulated)
- All on localhost:3001/3002
```

### LAN Network (Local Use)

```
PC A (Primary):
- Frontend @ localhost:5173
- Backend @ 0.0.0.0:3001

PC B (Secondary):
- Frontend @ localhost:5173
- Connects to: ws://PC-A-IP:3002/...
```

### Internet (Future Relay)

```
PC A ↘
      → Relay Server → ← PC B
PC C ↗

Relay at:
- publicip.com:3002
- Forwards all WebSocket traffic
- Handles NAT piercing
```

---

## File Organization Best Practices

```
When adding new features:

1. Keep TypeScript strict mode ON
2. Add types for new data structures
3. Update both frontend AND backend
4. Document in PROTOCOL.md if protocol changes
5. Add error handling for network failures
6. Test on both Primary and Secondary
7. Measure performance impact
```

---

**Last Updated**: April 6, 2026  
**Version**: 0.1.0
