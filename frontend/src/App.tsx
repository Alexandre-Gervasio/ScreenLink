import { useState } from 'react';
import './App.css';

export default function App() {
  const [mode, setMode] = useState<'home' | 'primary' | 'secondary'>('home');
  const [shareLink, setShareLink] = useState<string>('');
  const [shareCode, setShareCode] = useState<string>('');
  const [connected, setConnected] = useState(false);

  // Generate share link
  const generateShareLink = async () => {
    try {
      const response = await fetch('http://localhost:3001/api/links/create', {
        method: 'POST',
      });
      const data = await response.json();
      setShareCode(data.code);
      setShareLink(`screenlink://connect/${data.code}`);
    } catch (error) {
      console.error('Error generating link:', error);
    }
  };

  // Primary PC - Start sharing
  const startPrimaryMode = async () => {
    await generateShareLink();
    setMode('primary');
  };

  // Secondary PC - Connect to primary
  const connectToSecondary = (code: string) => {
    if (!code) {
      alert('Please enter a valid share code');
      return;
    }

    // TODO: Connect via WebSocket
    setMode('secondary');
    setConnected(true);
  };

  return (
    <div className="app">
      {mode === 'home' && (
        <div className="home-screen">
          <div className="header">
            <h1>🖥️ ScreenLink</h1>
            <p>Virtual Extended Monitor</p>
          </div>

          <div className="options">
            <button onClick={startPrimaryMode} className="primary-btn">
              🖱️ I'm the Main PC
              <span>Share my screen</span>
            </button>

            <div className="divider">OR</div>

            <button
              onClick={() => setMode('secondary')}
              className="secondary-btn"
            >
              🖥️ I'm the Extended Screen
              <span>Connect to main PC</span>
            </button>
          </div>

          <footer>
            <p>v0.1.0 Alpha</p>
          </footer>
        </div>
      )}

      {mode === 'primary' && (
        <div className="primary-screen">
          <div className="header">
            <h1>📡 Ready to Share</h1>
            <button onClick={() => setMode('home')}>← Back</button>
          </div>

          <div className="share-box">
            <h2>Share This Link</h2>
            <div className="code-box">
              <code>{shareCode}</code>
              <button
                onClick={() => navigator.clipboard.writeText(shareCode)}
              >
                📋 Copy
              </button>
            </div>
            <p className="full-url">{shareLink}</p>
            <p className="expires">Expires in 24 hours</p>
          </div>

          <div className="status">
            <p>Waiting for connection...</p>
            {connected && <p className="connected">✅ Display connected</p>}
          </div>
        </div>
      )}

      {mode === 'secondary' && (
        <div className="secondary-screen">
          <div className="header">
            <h1>🔗 Connect to Main PC</h1>
            <button onClick={() => setMode('home')}>← Back</button>
          </div>

          <div className="connect-box">
            <input
              type="text"
              placeholder="Enter share code (e.g., ABCD-1234)"
              maxLength={9}
              onChange={(e) => setShareCode(e.target.value.toUpperCase())}
            />
            <button onClick={() => connectToSecondary(shareCode)}>
              Connect
            </button>
          </div>

          {connected && (
            <div className="remote-display">
              <p>Connected! Your desktop is now extended.</p>
              <div className="video-container">
                <p>Waiting for video stream...</p>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
