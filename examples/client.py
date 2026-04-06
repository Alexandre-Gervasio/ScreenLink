#!/usr/bin/env python3

"""
Simple Python client example for ScreenLink

Usage:
    python3 client.py <uuid>

Example:
    python3 client.py 550e8400-e29b-41d4-a716-446655440000
"""

import websocket
import json
import sys
import threading
import time
import requests

API_BASE = 'http://localhost:3001'
WS_BASE = 'ws://localhost:3001'

class ScreenLinkPython:
    def __init__(self, uuid):
        self.uuid = uuid
        self.ws = None
        self.client_id = f'python-client-{int(time.time() * 1000)}'
        self.stats = {
            'framesReceived': 0,
            'latency': 0,
            'bandwidthMbps': 0,
        }
        self.is_connected = False

    def get_link_details(self):
        """Get link details from API"""
        try:
            response = requests.get(f'{API_BASE}/api/links/{self.uuid}')
            response.raise_for_status()
            data = response.json()
            print('✓ Link Details:')
            print(f"  Status: {data['status']}")
            print(f"  Created: {data['createdAt']}")
            print(f"  Expires: {data['expiresAt']}")
            return data
        except requests.exceptions.RequestException as e:
            print(f'✗ Failed to get link details: {e}')
            return None

    def connect(self):
        """Connect to WebSocket"""
        url = f'{WS_BASE}/ws?uuid={self.uuid}'
        print(f'\n🔌 Connecting to {url}...')

        self.ws = websocket.WebSocketApp(
            url,
            on_open=self.on_open,
            on_message=self.on_message,
            on_error=self.on_error,
            on_close=self.on_close,
        )

        # Run WebSocket in a separate thread
        ws_thread = threading.Thread(target=self.ws.run_forever)
        ws_thread.daemon = True
        ws_thread.start()

        # Keep main thread alive
        try:
            while self.ws.keep_running:
                time.sleep(0.1)
        except KeyboardInterrupt:
            print('\n\n👋 Shutting down...')
            self.disconnect()

    def on_open(self, ws):
        """Handle WebSocket connection opened"""
        print('✓ WebSocket connected!')
        self.is_connected = True

        connect_message = {
            'type': 'CONNECT',
            'role': 'secondary',
            'clientId': self.client_id,
            'timestamp': int(time.time() * 1000),
        }

        print('→ Sending CONNECT message...')
        ws.send(json.dumps(connect_message))

        # Start stats polling in background
        threading.Thread(target=self.poll_stats, daemon=True).start()

    def on_message(self, ws, message):
        """Handle incoming WebSocket messages"""
        try:
            if isinstance(message, bytes):
                # Binary video frame
                self.stats['framesReceived'] += 1
                if self.stats['framesReceived'] % 30 == 0:
                    print(f"  Received {self.stats['framesReceived']} frames")
            else:
                # JSON message
                data = json.loads(message)

                if data['type'] == 'CONNECT_ACK':
                    print(f"✓ Connected! Peer: {data['peerId']}")
                    print(f"  Role: {data['role']}")

                elif data['type'] == 'STATS':
                    self.stats['latency'] = data['latency']
                    self.stats['bandwidthMbps'] = data['bandwidthMbps']
                    fps = data.get('fps', '?')
                    latency = data.get('latency', '?')
                    bw = data.get('bandwidthMbps', '?')
                    print(f"  → FPS: {fps}, Latency: {latency}ms, BW: {bw}Mbps")

                elif data['type'] == 'ERROR':
                    print(f"✗ Server Error: {data['message']}")
                    if not data.get('recoverable', False):
                        self.disconnect()

                elif data['type'] == 'DISCONNECT':
                    print(f"✓ Server initiated disconnect: {data['reason']}")
                    self.disconnect()

                else:
                    print(f"? Received message type: {data['type']}")

        except json.JSONDecodeError as e:
            print(f'✗ Failed to parse message: {e}')

    def on_error(self, ws, error):
        """Handle WebSocket errors"""
        print(f'✗ WebSocket Error: {error}')

    def on_close(self, ws, close_status_code, close_msg):
        """Handle WebSocket connection closed"""
        print('✓ WebSocket closed')
        self.is_connected = False
        print('\n📊 Final Statistics:')
        print(f"  Frames Received: {self.stats['framesReceived']}")
        print(f"  Last Latency: {self.stats['latency']}ms")
        print(f"  Last Bandwidth: {self.stats['bandwidthMbps']}Mbps")

    def poll_stats(self):
        """Poll stats from API every 5 seconds"""
        while self.is_connected:
            try:
                response = requests.get(f'{API_BASE}/api/stats', timeout=2)
                if response.status_code == 200:
                    stats = response.json()
                    cpu = stats['cpu'].get('usage', 0)
                    mem = stats['memory'].get('used', 0)
                    print(f"  → CPU: {cpu:.1f}%, Memory: {mem}MB")
            except requests.exceptions.RequestException:
                # Silently ignore if server is down
                pass
            time.sleep(5)

    def disconnect(self):
        """Disconnect WebSocket"""
        if self.ws:
            self.ws.close()
            self.is_connected = False


def main():
    if len(sys.argv) < 2:
        print('Usage: python3 client.py <uuid>')
        print('Example: python3 client.py 550e8400-e29b-41d4-a716-446655440000')
        sys.exit(1)

    uuid = sys.argv[1]
    client = ScreenLinkPython(uuid)

    # Get link details first
    if not client.get_link_details():
        print('Invalid UUID or link expired.')
        sys.exit(1)

    # Connect WebSocket
    client.connect()


if __name__ == '__main__':
    main()
