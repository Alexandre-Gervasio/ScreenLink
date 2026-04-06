#!/usr/bin/env node

/**
 * Simple Node.js client example for ScreenLink
 * 
 * Usage:
 *   node client.js <uuid>
 * 
 * Example:
 *   node client.js 550e8400-e29b-41d4-a716-446655440000
 */

import WebSocket from 'ws';
import axios from 'axios';

const API_BASE = 'http://localhost:3001';
const WS_BASE = 'ws://localhost:3001';

class ScreenLinkClient {
  constructor(uuid) {
    this.uuid = uuid;
    this.ws = null;
    this.clientId = `client-${Date.now()}`;
    this.stats = {
      framesReceived: 0,
      latency: 0,
      bandwidthMbps: 0,
    };
  }

  /**
   * Get link details from API
   */
  async getLinkDetails() {
    try {
      const response = await axios.get(`${API_BASE}/api/links/${this.uuid}`);
      console.log('✓ Link Details:', {
        status: response.data.status,
        created: response.data.createdAt,
        expires: response.data.expiresAt,
      });
      return response.data;
    } catch (error) {
      console.error('✗ Failed to get link details:', error.response?.data || error.message);
      return null;
    }
  }

  /**
   * Connect to WebSocket
   */
  connect() {
    const url = `${WS_BASE}/ws?uuid=${this.uuid}`;
    console.log(`\n🔌 Connecting to ${url}...`);

    this.ws = new WebSocket(url);

    this.ws.on('open', () => this.onOpen());
    this.ws.on('message', (data) => this.onMessage(data));
    this.ws.on('error', (error) => this.onError(error));
    this.ws.on('close', () => this.onClose());
  }

  onOpen() {
    console.log('✓ WebSocket connected!');

    const connectMessage = {
      type: 'CONNECT',
      role: 'secondary',
      clientId: this.clientId,
      timestamp: Date.now(),
    };

    console.log('→ Sending CONNECT message...');
    this.ws.send(JSON.stringify(connectMessage));
  }

  onMessage(data) {
    if (typeof data === 'object' && !(data instanceof Buffer)) {
      // Binary video frame
      this.stats.framesReceived++;
      if (this.stats.framesReceived % 30 === 0) {
        console.log(`  Received ${this.stats.framesReceived} frames`);
      }
    } else {
      // JSON message
      try {
        const message = JSON.parse(data.toString());

        switch (message.type) {
          case 'CONNECT_ACK':
            console.log('✓ Connected! Peer:', message.peerId);
            console.log('  Role:', message.role);
            this.startStatsPolling();
            break;

          case 'STATS':
            this.stats.latency = message.latency;
            this.stats.bandwidthMbps = message.bandwidthMbps;
            console.log(`  → FPS: ${message.fps}, Latency: ${message.latency}ms, BW: ${message.bandwidthMbps}Mbps`);
            break;

          case 'ERROR':
            console.error('✗ Server Error:', message.message);
            if (!message.recoverable) {
              this.disconnect();
            }
            break;

          case 'DISCONNECT':
            console.log('✓ Server initiated disconnect:', message.reason);
            this.disconnect();
            break;

          default:
            console.log(`? Received message type: ${message.type}`);
        }
      } catch (error) {
        console.error('✗ Failed to parse message:', error.message);
      }
    }
  }

  onError(error) {
    console.error('✗ WebSocket Error:', error.message);
  }

  onClose() {
    console.log('✓ WebSocket closed');
    console.log('\n📊 Final Statistics:');
    console.log(`  Frames Received: ${this.stats.framesReceived}`);
    console.log(`  Last Latency: ${this.stats.latency}ms`);
    console.log(`  Last Bandwidth: ${this.stats.bandwidthMbps}Mbps`);
  }

  startStatsPolling() {
    setInterval(async () => {
      try {
        const response = await axios.get(`${API_BASE}/api/stats`);
        const stats = response.data;
        console.log(`  → CPU: ${stats.cpu.usage.toFixed(1)}%, Memory: ${stats.memory.used}MB`);
      } catch (error) {
        // Silently ignore if server is down
      }
    }, 5000);
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
    }
  }
}

// Main
async function main() {
  const uuid = process.argv[2];

  if (!uuid) {
    console.error('Usage: node client.js <uuid>');
    console.error('Example: node client.js 550e8400-e29b-41d4-a716-446655440000');
    process.exit(1);
  }

  const client = new ScreenLinkClient(uuid);

  // Get link details first
  const linkDetails = await client.getLinkDetails();
  if (!linkDetails) {
    console.error('Invalid UUID or link expired.');
    process.exit(1);
  }

  // Connect WebSocket
  client.connect();

  // Graceful shutdown
  process.on('SIGINT', () => {
    console.log('\n\n👋 Shutting down...');
    client.disconnect();
    process.exit(0);
  });
}

main().catch((error) => {
  console.error('Fatal error:', error);
  process.exit(1);
});
