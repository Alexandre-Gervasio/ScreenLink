# Examples

Complete examples for integrating with ScreenLink.

## Table of Contents

- [Node.js Client](#nodejs-client)
- [Python Client](#python-client)
- [Browser Client](#browser-client)
- [Common Patterns](#common-patterns)

## Node.js Client

Simple Node.js client that connects to ScreenLink and receives video frames.

**File**: `client.js`

**Required packages**:
```bash
npm install ws axios
```

**Usage**:
```bash
node client.js 550e8400-e29b-41d4-a716-446655440000
```

**Features**:
- Connects to WebSocket with UUID
- Sends CONNECT message
- Receives video frames
- Polls server statistics
- Handles errors and graceful shutdown

**Key Points**:
```javascript
// Connect to WebSocket
const ws = new WebSocket(`ws://localhost:3001/ws?uuid=${uuid}`);

// Send connect message
ws.send(JSON.stringify({
  type: 'CONNECT',
  role: 'secondary',
  clientId: 'my-client'
}));

// Receive messages
ws.on('message', (data) => {
  if (data instanceof Buffer) {
    // Video frame (binary)
  } else {
    // JSON message
  }
});
```

---

## Python Client

Simple Python client using WebSocket.

**File**: `client.py`

**Required packages**:
```bash
pip install websocket-client requests
```

**Usage**:
```bash
python3 client.py 550e8400-e29b-41d4-a716-446655440000
```

**Features**:
- WebSocket connection with UUID
- Async message handling
- Statistics polling
- Thread-safe operations
- Error handling

**Key Points**:
```python
# Connect to WebSocket
url = f'ws://localhost:3001/ws?uuid={uuid}'
ws = websocket.WebSocketApp(
    url,
    on_open=on_open,
    on_message=on_message
)
ws.run_forever()

# Send message
message = {'type': 'CONNECT', 'role': 'secondary'}
ws.send(json.dumps(message))
```

---

## Browser Client

Interactive HTML+JavaScript client that runs in any web browser.

**File**: `browser-client.html`

**Usage**:
1. Open `browser-client.html` in your web browser
2. Enter your UUID
3. Click "Connect"
4. Watch statistics in real-time

**Features**:
- Beautiful UI with gradients
- Real-time statistics display
- Status log for debugging
- Responsive design (mobile-friendly)
- No dependencies required

**Key Points**:
```javascript
// Browser WebSocket API
const ws = new WebSocket(`ws://localhost:3001/ws?uuid=${uuid}`);

ws.onopen = () => {
  ws.send(JSON.stringify(message));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  // Handle message
};
```

---

## Common Patterns

### Error Handling

```javascript
// Check if error is recoverable
ws.on('message', (data) => {
  const message = JSON.parse(data);
  
  if (message.type === 'ERROR') {
    if (message.recoverable) {
      console.log('Recoverable error, reconnecting...');
      reconnect();
    } else {
      console.log('Fatal error, shutting down');
      process.exit(1);
    }
  }
});
```

### Reconnection Logic

```javascript
let retries = 0;
const MAX_RETRIES = 5;

function connect() {
  try {
    ws = new WebSocket(url);
    ws.onopen = () => { retries = 0; };
    ws.onclose = () => {
      if (retries < MAX_RETRIES) {
        retries++;
        setTimeout(connect, 1000 * retries);
      }
    };
  } catch (error) {
    console.error(error);
  }
}
```

### Performance Monitoring

```javascript
// Track performance metrics
const metrics = {
  fps: 0,
  latency: 0,
  bandwidth: 0,
};

ws.on('message', (data) => {
  const message = JSON.parse(data);
  
  if (message.type === 'STATS') {
    metrics.fps = message.fps;
    metrics.latency = message.latency;
    metrics.bandwidth = message.bandwidthMbps;
    
    if (metrics.latency > 200) {
      console.warn('High latency detected');
    }
  }
});
```

### Graceful Shutdown

```javascript
// Handle process termination
process.on('SIGINT', () => {
  console.log('Shutting down...');
  
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify({
      type: 'DISCONNECT',
      reason: 'client_shutdown'
    }));
  }
  
  process.exit(0);
});
```

### REST API Integration

```javascript
// Generate new link
const response = await fetch('http://localhost:3001/api/links/generate', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' }
});

const { uuid } = await response.json();
console.log('New UUID:', uuid);

// Connect with that UUID
const ws = new WebSocket(`ws://localhost:3001/ws?uuid=${uuid}`);
```

---

## Testing Your Connection

### Using curl

```bash
# Check server health
curl http://localhost:3001/health

# Generate new link
curl -X POST http://localhost:3001/api/links/generate

# Get link details
curl http://localhost:3001/api/links/550e8400-e29b-41d4-a716-446655440000

# Get statistics
curl http://localhost:3001/api/stats
```

### Using WebSocket CLI tools

```bash
# Using wscat (npm install -g wscat)
wscat -c "ws://localhost:3001/ws?uuid=550e8400-e29b-41d4-a716-446655440000"

# Then type JSON messages:
# {"type":"CONNECT","role":"secondary","clientId":"test"}
```

---

## Tips & Best Practices

💡 **Connection Tips**:
- Always validate UUID format before connecting
- Implement exponential backoff for reconnects
- Set appropriate timeouts for WebSocket messages
- Log connection state transitions for debugging

⚡ **Performance Tips**:
- Monitor latency and adjust quality if needed
- Batch small messages when possible
- Use binary frames for video data
- Clean up resources on disconnect

🔒 **Security Tips**:
- Validate all incoming data
- Never expose UUIDs in client-side code
- Use TLS/SSL in production (wss://)
- Implement rate limiting for API calls
- Sanitize user inputs

---

**Last Updated**: April 6, 2026
