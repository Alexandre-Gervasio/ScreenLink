# API Documentation

Complete API reference for ScreenLink endpoints and WebSocket protocol.

## Table of Contents

- [REST API Endpoints](#rest-api-endpoints)
- [WebSocket Protocol](#websocket-protocol)
- [Data Models](#data-models)
- [Error Handling](#error-handling)
- [Examples](#examples)

---

## REST API Endpoints

### Base URL
```
http://localhost:3001
```

### Server Health

**GET** `/health`
```bash
curl http://localhost:3001/health
```

**Response (200 OK):**
```json
{
  "status": "ok",
  "version": "1.1.0",
  "uptime": 3600,
  "connections": 2
}
```

---

### Link Management

#### Generate Share Link
**POST** `/api/links/generate`

Generate a new shareable UUID link.

```bash
curl -X POST http://localhost:3001/api/links/generate \
  -H "Content-Type: application/json" \
  -d '{}'
```

**Response (201 Created):**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "shortCode": "ABCD-1234",
  "expiresAt": "2026-04-07T14:28:00Z",
  "createdAt": "2026-04-06T14:28:00Z"
}
```

#### List Active Links
**GET** `/api/links`

Get all currently active share links.

```bash
curl http://localhost:3001/api/links
```

**Response (200 OK):**
```json
{
  "links": [
    {
      "uuid": "550e8400-e29b-41d4-a716-446655440000",
      "shortCode": "ABCD-1234",
      "status": "active",
      "connectedClients": 1,
      "createdAt": "2026-04-06T14:28:00Z"
    }
  ],
  "total": 1
}
```

#### Get Link Details
**GET** `/api/links/:uuid`

Get detailed information about a specific link.

```bash
curl http://localhost:3001/api/links/550e8400-e29b-41d4-a716-446655440000
```

**Response (200 OK):**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "shortCode": "ABCD-1234",
  "status": "active",
  "createdAt": "2026-04-06T14:28:00Z",
  "expiresAt": "2026-04-07T14:28:00Z",
  "clients": [
    {
      "id": "client-1",
      "connected": true,
      "role": "primary",
      "address": "192.168.1.100"
    }
  ]
}
```

#### Revoke Link
**DELETE** `/api/links/:uuid`

Immediately deactivate a share link.

```bash
curl -X DELETE http://localhost:3001/api/links/550e8400-e29b-41d4-a716-446655440000
```

**Response (204 No Content):**
```
(no body)
```

---

### Display Information

#### Get Display Properties
**GET** `/api/display`

Get information about the connected display.

```bash
curl http://localhost:3001/api/display
```

**Response (200 OK):**
```json
{
  "primary": {
    "width": 1920,
    "height": 1080,
    "refreshRate": 60,
    "colorDepth": 32,
    "scale": 1.0
  },
  "secondary": null,
  "count": 1
}
```

#### Get Display Capabilities
**GET** `/api/display/capabilities`

Get supported resolutions and frame rates.

```bash
curl http://localhost:3001/api/display/capabilities
```

**Response (200 OK):**
```json
{
  "maxResolution": "4K",
  "supportedFrameRates": [24, 30, 60],
  "supportedCodecs": ["H.264"],
  "hardwareAcceleration": true
}
```

---

### Statistics

#### Get Connection Statistics
**GET** `/api/stats`

Get real-time performance statistics.

```bash
curl http://localhost:3001/api/stats
```

**Response (200 OK):**
```json
{
  "cpu": {
    "usage": 18.5,
    "cores": 8
  },
  "memory": {
    "used": 145,
    "total": 8192
  },
  "network": {
    "bytesIn": 1024000,
    "bytesOut": 2048000,
    "bandwidthMbps": 12.5
  },
  "frames": {
    "sent": 1800,
    "framerate": 30,
    "latency": 45
  }
}
```

---

## WebSocket Protocol

### Connection

**URL:** `ws://localhost:3001/ws?uuid=<UUID>`

```javascript
const ws = new WebSocket('ws://localhost:3001/ws?uuid=550e8400-e29b-41d4-a716-446655440000');

ws.onopen = () => {
  console.log('Connected to ScreenLink');
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  handleMessage(message);
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = () => {
  console.log('Disconnected from ScreenLink');
};
```

### Message Types

#### CONNECT
Initialize connection and set role.

```json
{
  "type": "CONNECT",
  "role": "primary",
  "clientId": "client-123",
  "timestamp": 1712427000000
}
```

**Response:**
```json
{
  "type": "CONNECT_ACK",
  "clientId": "client-123",
  "peerId": "client-456",
  "role": "primary"
}
```

#### VIDEO_FRAME (Binary)
H.264 encoded video stream (sent from primary).

```
Binary WebSocket frame:
[H.264 NAL Unit Header] [H.264 Data]
- Frame type: Keyframe (0x67) or H264 (0x65)
- Timestamp: 8 bytes (milliseconds)
```

#### STATS
Performance metrics.

```json
{
  "type": "STATS",
  "fps": 30,
  "latency": 45,
  "bandwidthMbps": 12.5,
  "cpu": 18.5,
  "memory": 145
}
```

#### INPUT_EVENT (from secondary to primary)
Mouse or keyboard input.

```json
{
  "type": "INPUT_EVENT",
  "subtype": "MOUSE_MOVE",
  "data": {
    "x": 500,
    "y": 300
  },
  "timestamp": 1712427000045
}
```

#### DISCONNECT
Gracefully close connection.

```json
{
  "type": "DISCONNECT",
  "reason": "user_requested"
}
```

#### ERROR
Error notification.

```json
{
  "type": "ERROR",
  "code": "INVALID_UUID",
  "message": "UUID not found or expired",
  "recoverable": false
}
```

---

## Data Models

### UUID Object
```typescript
interface UUID {
  id: string; // UUID v4 format
  shortCode: string; // 8-char code
  createdAt: Date;
  expiresAt: Date;
  primaryClient?: Client;
  secondaryClient?: Client;
  status: 'active' | 'expired' | 'revoked';
}
```

### Client Object
```typescript
interface Client {
  id: string;
  role: 'primary' | 'secondary';
  address: string;
  port: number;
  connected: boolean;
  connectedAt: Date;
  disconnectedAt?: Date;
  stats: ClientStats;
}
```

### ClientStats Object
```typescript
interface ClientStats {
  fps: number;
  latency: number; // milliseconds
  bandwidthMbps: number;
  cpuUsage: number; // percentage
  memoryUsageMB: number;
  framesReceived: number;
  framesDropped: number;
}
```

---

## Error Handling

### Error Response Format

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message",
    "details": {},
    "timestamp": "2026-04-06T14:28:00Z"
  }
}
```

### Common Error Codes

| Code | Status | Meaning |
|------|--------|---------|
| `INVALID_UUID` | 400 | UUID format incorrect or expired |
| `UUID_NOT_FOUND` | 404 | UUID doesn't exist |
| `ALREADY_CONNECTED` | 409 | Connection already established |
| `DISPLAY_ERROR` | 500 | Failed to access display |
| `ENCODER_ERROR` | 500 | Video encoding failed |
| `NETWORK_ERROR` | 503 | Network connectivity issue |

### Error Response Examples

**Invalid UUID:**
```json
{
  "error": {
    "code": "INVALID_UUID",
    "message": "UUID format is invalid. Expected 36 characters.",
    "details": {
      "received": "not-a-uuid"
    }
  }
}
```

**Display Error:**
```json
{
  "error": {
    "code": "DISPLAY_ERROR",
    "message": "Failed to enumerate displays. Check permissions.",
    "details": {
      "platform": "linux",
      "requiredPermissions": ["video"]
    }
  }
}
```

---

## Examples

### Node.js Client Example

```typescript
import WebSocket from 'ws';

class ScreenLinkClient {
  private ws: WebSocket;
  private uuid: string;

  constructor(uuid: string) {
    this.uuid = uuid;
    this.connect();
  }

  private connect() {
    this.ws = new WebSocket(`ws://localhost:3001/ws?uuid=${this.uuid}`);

    this.ws.on('open', () => {
      console.log('Connected');
      this.sendConnect();
    });

    this.ws.on('message', (data) => {
      this.handleMessage(data);
    });

    this.ws.on('error', (error) => {
      console.error('Error:', error.message);
    });
  }

  private sendConnect() {
    const message = {
      type: 'CONNECT',
      role: 'secondary',
      clientId: 'node-client',
    };
    this.ws.send(JSON.stringify(message));
  }

  private handleMessage(data: Buffer | string) {
    if (data instanceof Buffer) {
      // Binary video frame
      console.log('Received video frame:', data.length, 'bytes');
    } else {
      // JSON message
      const message = JSON.parse(data.toString());
      console.log('Received:', message.type);
    }
  }

  disconnect() {
    this.ws.close();
  }
}

// Usage
const client = new ScreenLinkClient('550e8400-e29b-41d4-a716-446655440000');
```

### Python Client Example

```python
import websocket
import json
import time

class ScreenLinkPython:
    def __init__(self, uuid):
        self.uuid = uuid
        self.ws = None
        self.connect()

    def connect(self):
        url = f'ws://localhost:3001/ws?uuid={self.uuid}'
        self.ws = websocket.WebSocketApp(
            url,
            on_open=self.on_open,
            on_message=self.on_message,
            on_error=self.on_error,
            on_close=self.on_close
        )
        self.ws.run_forever()

    def on_open(self, ws):
        print('Connected to ScreenLink')
        message = {
            'type': 'CONNECT',
            'role': 'secondary',
            'clientId': 'python-client'
        }
        ws.send(json.dumps(message))

    def on_message(self, ws, message):
        if isinstance(message, bytes):
            print(f'Received video frame: {len(message)} bytes')
        else:
            data = json.loads(message)
            print(f'Received: {data["type"]}')

    def on_error(self, ws, error):
        print(f'Error: {error}')

    def on_close(self, ws, close_status_code, close_msg):
        print('Disconnected')

# Usage
client = ScreenLinkPython('550e8400-e29b-41d4-a716-446655440000')
```

---

## Rate Limiting

- **API Endpoints**: 100 requests/minute per IP
- **WebSocket**: Unlimited (per-connection)
- **Headers**:
  ```
  X-RateLimit-Limit: 100
  X-RateLimit-Remaining: 85
  X-RateLimit-Reset: 1712427060
  ```

---

## Authentication

Future versions will support:
- API Key authentication
- OAuth2
- JWT tokens

Currently, UUID acts as the authentication token.

---

**Last Updated**: April 6, 2026
