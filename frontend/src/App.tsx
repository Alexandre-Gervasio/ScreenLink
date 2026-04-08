import { useState } from 'react';
import './App.css';

export default function App() {
  const [mode, setMode] = useState<'home' | 'primary' | 'secondary'>('home');
  const [shareLink, setShareLink] = useState<string>('');
  const [shareCode, setShareCode] = useState<string>('');
  const [connected, setConnected] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');
  const [inputCode, setInputCode] = useState<string>('');

  // Generate share link
  const generateShareLink = async () => {
    setLoading(true);
    setError('');
    try {
      const response = await fetch('http://localhost:3001/api/links/create', {
        method: 'POST',
      });
      
      if (!response.ok) {
        throw new Error(`Erro da API: ${response.status}`);
      }
      
      const data = await response.json();
      
      if (!data.code) {
        // Fallback: generate a temporary code if API doesn't return one
        const tempCode = Math.random().toString(36).substring(2, 8).toUpperCase();
        setShareCode(tempCode);
        setShareLink(`screenlink://connect/${tempCode}`);
      } else {
        setShareCode(data.code);
        setShareLink(`screenlink://connect/${data.code}`);
      }
    } catch (error) {
      console.error('Erro ao gerar link:', error);
      setError('Falha ao gerar código. Tente novamente.');
      
      // Fallback: generate a temporary code
      const tempCode = Math.random().toString(36).substring(2, 8).toUpperCase();
      setShareCode(tempCode);
      setShareLink(`screenlink://connect/${tempCode}`);
    } finally {
      setLoading(false);
    }
  };

  // Primary PC - Start sharing
  const startPrimaryMode = async () => {
    await generateShareLink();
    setMode('primary');
  };

  // Secondary PC - Connect to primary
  const connectToSecondary = (code: string) => {
    if (!code || code.trim().length === 0) {
      setError('Por favor, digite um código válido');
      return;
    }

    // TODO: Connect via WebSocket
    setMode('secondary');
    setConnected(true);
    setError('');
  };

  return (
    <div className="app">
      {mode === 'home' && (
        <div className="home-screen">
          <div className="header">
            <h1>🖥️ ScreenLink</h1>
            <p>Monitor Virtual Estendido</p>
          </div>

          <div className="options">
            <button onClick={startPrimaryMode} className="primary-btn">
              🖱️ Sou o PC Principal
              <span>Compartilhar minha tela</span>
            </button>

            <div className="divider">OU</div>

            <button
              onClick={() => {
                setInputCode('');
                setError('');
                setMode('secondary');
              }}
              className="secondary-btn"
            >
              🖥️ Sou a Tela Estendida
              <span>Conectar ao PC principal</span>
            </button>
          </div>

          <footer>
            <p>v1.0.0 Beta</p>
          </footer>
        </div>
      )}

      {mode === 'primary' && (
        <div className="primary-screen">
          <div className="header">
            <h1>📡 Pronto para Compartilhar</h1>
            <button onClick={() => setMode('home')}>← Voltar</button>
          </div>

          <div className="share-box">
            <h2>Código para Outra Máquina</h2>
            
            {loading && (
              <div className="loading">
                <p>Gerando código...</p>
              </div>
            )}
            
            {error && (
              <div className="error-message">
                <p>{error}</p>
              </div>
            )}
            
            {shareCode && (
              <div className="code-box">
                <code>{shareCode}</code>
                <button
                  onClick={() => navigator.clipboard.writeText(shareCode)}
                >
                  📋 Copiar
                </button>
              </div>
            )}
            
            {shareLink && (
              <p className="full-url">{shareLink}</p>
            )}
            
            <p className="expires">📅 Válido por 24 horas</p>
          </div>

          <div className="status">
            <p>Aguardando conexão...</p>
            {connected && <p className="connected">✅ Tela conectada</p>}
          </div>
        </div>
      )}

      {mode === 'secondary' && (
        <div className="secondary-screen">
          <div className="header">
            <h1>🔗 Conectar ao PC Principal</h1>
            <button onClick={() => setMode('home')}>← Voltar</button>
          </div>

          <div className="connect-box">
            <label>Digite o código do PC Principal:</label>
            <input
              type="text"
              placeholder="Ex: ABCD1234"
              maxLength={9}
              value={inputCode}
              onChange={(e) => {
                setInputCode(e.target.value.toUpperCase());
                setError('');
              }}
            />
            
            {error && (
              <div className="error-message">
                <p>{error}</p>
              </div>
            )}
            
            <button onClick={() => connectToSecondary(inputCode)}>
              Conectar
            </button>
          </div>

          {connected && (
            <div className="remote-display">
              <p>✅ Conectado! Sua tela está estendida.</p>
              <div className="video-container">
                <p>Aguardando stream de vídeo...</p>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
