import express from 'express';
import expressWs from 'express-ws';
import { Router } from 'express';
import { v4 as uuidv4 } from 'uuid';
import dotenv from 'dotenv';
import type { Express, Request } from 'express';
import type { WebSocket } from 'ws';

dotenv.config();

const PORT = process.env.PORT || 3001;
const WS_PORT = process.env.WS_PORT || 3002;

const app: Express = express();
const { app: wsApp } = expressWs(app);

// Middleware
app.use(express.json());

// Types
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

// In-memory store
const shareLinks: Map<string, ShareLink> = new Map();
const activeSessions: Map<string, ClientSession> = new Map();

// Generate unique link
function generateShareLink(): ShareLink {
  const uuid = uuidv4();
  const code = Math.random().toString(36).substring(2, 8).toUpperCase();
  const now = new Date();
  const expires = new Date(now.getTime() + 24 * 60 * 60 * 1000); // 24 hours

  const link: ShareLink = {
    uuid,
    code,
    created: now,
    expires,
    primaryPC: 'primary-' + uuid.substring(0, 8),
    active: true,
  };

  shareLinks.set(uuid, link);
  return link;
}

// REST API Routes
const router = Router();

// Create new share link
router.post('/api/links/create', (req: Request, res) => {
  const link = generateShareLink();
  res.json({
    success: true,
    uuid: link.uuid,
    code: link.code,
    shareUrl: `screenlink://connect/${link.code}`,
    expiresIn: '24 hours',
  });
});

// Get link info
router.get('/api/links/:uuid', (req: Request, res) => {
  const link = shareLinks.get(req.params.uuid);

  if (!link) {
    return res.status(404).json({ error: 'Link not found' });
  }

  if (new Date() > link.expires) {
    link.active = false;
    res.status(410).json({ error: 'Link expired' });
  }

  res.json({
    uuid: link.uuid,
    code: link.code,
    active: link.active,
    created: link.created,
    expiresAt: link.expires,
    primaryConnected: !!link.primaryPC,
    secondaryConnected: !!link.secondaryPC,
  });
});

// WebSocket handler
wsApp.ws('/ws/:uuid/:type/:code', (ws: WebSocket, req: Request) => {
  const { uuid, type, code } = req.params;
  const link = shareLinks.get(uuid);

  // Validate
  if (!link || link.code !== code) {
    ws.close(4001, 'Invalid UUID or code');
    return;
  }

  if (new Date() > link.expires) {
    ws.close(4002, 'Link expired');
    return;
  }

  if (type !== 'primary' && type !== 'secondary') {
    ws.close(4003, 'Invalid type');
    return;
  }

  const sessionId = `${uuid}-${type}`;
  const session: ClientSession = {
    type: type as 'primary' | 'secondary',
    uuid,
    ws,
    connected: true,
  };

  activeSessions.set(sessionId, session);

  console.log(`✅ ${type.toUpperCase()} connected: ${uuid}`);

  // Send greeting
  ws.send(JSON.stringify({
    type: 'CONNECTED',
    message: `Connected as ${type} to ${code}`,
    timestamp: new Date().toISOString(),
  }));

  // Handle messages
  ws.on('message', (data: any) => {
    try {
      const message = typeof data === 'string' ? JSON.parse(data) : data;

      // Route based on message type
      if (message.type === 'VIDEO_FRAME') {
        // Forward video to other peer
        const otherType = type === 'primary' ? 'secondary' : 'primary';
        const otherSession = activeSessions.get(`${uuid}-${otherType}`);

        if (otherSession?.connected) {
          otherSession.ws.send(data);
        }
      } else if (message.type === 'CONTROL') {
        // Forward mouse/keyboard control
        const otherType = type === 'primary' ? 'secondary' : 'primary';
        const otherSession = activeSessions.get(`${uuid}-${otherType}`);

        if (otherSession?.connected) {
          otherSession.ws.send(JSON.stringify({
            type: 'INPUT',
            ...message.data,
          }));
        }
      }
    } catch (error) {
      console.error('Message error:', error);
    }
  });

  // Handle disconnect
  ws.on('close', () => {
    activeSessions.delete(sessionId);
    console.log(`❌ ${type.toUpperCase()} disconnected: ${uuid}`);

    // Notify other peer
    const otherType = type === 'primary' ? 'secondary' : 'primary';
    const otherSession = activeSessions.get(`${uuid}-${otherType}`);
    if (otherSession?.connected) {
      otherSession.ws.send(JSON.stringify({
        type: 'PEER_DISCONNECTED',
        peer: type,
      }));
    }
  });

  ws.on('error', (error: Error) => {
    console.error(`WebSocket error [${sessionId}]:`, error.message);
  });
});

app.use(router);

// Health check
app.get('/health', (req, res) => {
  res.json({
    status: 'ok',
    timestamp: new Date().toISOString(),
    activeSessions: activeSessions.size,
    activeLinks: shareLinks.size,
  });
});

// Start server
app.listen(PORT, () => {
  console.log(`
╔════════════════════════════════════════╗
║        🖥️  ScreenLink Backend v0.1.0   ║
║      Virtual Extended Monitor Server   ║
╚════════════════════════════════════════╝

✅ HTTP Server: http://0.0.0.0:${PORT}
✅ WebSocket: ws://0.0.0.0:${WS_PORT}
📊 Health: http://localhost:${PORT}/health

Ready for connections! 🚀
  `);
});

// Graceful shutdown
process.on('SIGINT', () => {
  console.log('\n👋 Shutting down gracefully...');
  process.exit(0);
});
