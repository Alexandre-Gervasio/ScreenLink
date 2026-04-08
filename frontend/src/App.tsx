import { useState, useRef } from 'react';
import './App.css';

interface ConnectionState {
  uuid: string;
  type: 'primary' | 'secondary';
  code: string;
}

export default function App() {
  const [mode, setMode] = useState<'home' | 'primary' | 'secondary'>('home');
  const [shareLink, setShareLink] = useState<string>('');
  const [shareCode, setShareCode] = useState<string>('');
  const [connected, setConnected] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');
  const [inputCode, setInputCode] = useState<string>('');
  const [statusMessage, setStatusMessage] = useState<string>('');
  const [connectionState, setConnectionState] = useState<ConnectionState | null>(null);
  const wsRef = useRef<WebSocket | null>(null);

  // Generate share link and connect primary
  const generateShareLink = async () => {
    setLoading(true);
    setError('');
    setStatusMessage('Gerando código de pairing...');
    
    try {
      // Try to connect to backend, fallback to localhost
      const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:3001';
      
      const response = await fetch(`${apiUrl}/api/links/create`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
      });
      
      if (!response.ok) {
        throw new Error(`Erro da API: ${response.status}`);
      }
      
      const data = await response.json();
      setShareCode(data.code);
      setShareLink(`screenlink://connect/${data.code}`);
      
      setStatusMessage('Código gerado! Conectando...');
      
      // Connect as primary
      connectToPeer('primary', data.uuid, data.code);
    } catch (error) {
      console.error('Erro ao gerar link:', error);
      setError('⚠️ Servidor backend não respondeu. Usando modo offline (sem conexão real)');
      
      // Fallback: generate demo code
      const tempCode = Math.random().toString(36).substring(2, 8).toUpperCase();
      setShareCode(tempCode);
      setShareLink(`screenlink://connect/${tempCode}`);
      setStatusMessage('Modo demo - aguardando conexão de outro PC...');
    } finally {
      setLoading(false);
    }
  };

  // Connect to peer via WebSocket
  const connectToPeer = (type: 'primary' | 'secondary', uuid: string, code: string) => {
    try {
      const wsUrl = import.meta.env.VITE_WS_URL || `ws://localhost:3001`;
      const wsPath = `/ws/${uuid}/${type}/${code}`;
      const fullUrl = `${wsUrl}${wsPath}`;
      
      console.log(`Conectando como ${type} a: ${fullUrl}`);
      setStatusMessage(`Conectando como ${type}...`);
      
      const ws = new WebSocket(fullUrl);
      
      ws.onopen = () => {
        console.log(`✅ WebSocket conectado como ${type}`);
        wsRef.current = ws;
        setConnectionState({ uuid, type, code });
        setStatusMessage(`${type === 'primary' ? 'PC Principal' : 'Tela Estendida'} pronto! Aguardando conexão...`);
      };
      
      ws.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data);
          console.log('Mensagem recebida:', message);
          
          if (message.type === 'CONNECTED') {
            setConnected(true);
            setStatusMessage(`✅ Conectado com sucesso!`);
          } else if (message.type === 'PEER_DISCONNECTED') {
            setConnected(false);
            setStatusMessage(`⚠️ Outro PC desconectou`);
          }
        } catch (e) {
          console.log('Dados binários recebidos (vídeo/áudio)');
        }
      };
      
      ws.onerror = (event) => {
        console.error('WebSocket erro:', event);
        setError('❌ Erro de conexão WebSocket');
        setStatusMessage('Falha ao conectar. Verifique se o backend está rodando.');
      };
      
      ws.onclose = () => {
        console.log(`WebSocket fechado para ${type}`);
        wsRef.current = null;
        setConnected(false);
        if (type === 'primary') {
          setStatusMessage('Desconectado');
        }
      };
    } catch (error) {
      console.error('Erro ao conectar:', error);
      setError(`Erro ao conectar: ${error}`);
    }
  };

  // Primary PC - Start sharing
  const startPrimaryMode = async () => {
    await generateShareLink();
    setMode('primary');
  };

  // Secondary PC - Connect to primary
  const connectToSecondary = async (code: string) => {
    if (!code || code.trim().length === 0) {
      setError('Por favor, digite um código válido');
      return;
    }

    setLoading(true);
    setError('');
    setStatusMessage('Verificando código...');
    
    try {
      const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:3001';
      
      // First, get the UUID for this code
      const linksResponse = await fetch(`${apiUrl}/api/links`, {
        method: 'GET',
      });
      
      if (!linksResponse.ok) {
        throw new Error('Não foi possível buscar links');
      }
      
      const linksData = await linksResponse.json();
      const matchingLink = linksData.links?.find((link: any) => link.code === code.toUpperCase());
      
      if (!matchingLink) {
        throw new Error('Código inválido ou expirado');
      }
      
      setStatusMessage('Conectando ao PC Principal...');
      setMode('secondary');
      
      // Connect as secondary
      connectToPeer('secondary', matchingLink.uuid, code.toUpperCase());
    } catch (error) {
      console.error('Erro ao conectar:', error);
      setError(`❌ ${error}`);
      setStatusMessage('Falha na conexão');
      
      // Fallback: connect anyway in demo mode
      setMode('secondary');
      const demoUuid = `demo-${code}`;
      setStatusMessage('Modo demo - tentando conectar...');
      connectToPeer('secondary', demoUuid, code.toUpperCase());
    } finally {
      setLoading(false);
    }
  };

  // Disconnect
  const disconnect = () => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
    setConnected(false);
    setConnectionState(null);
    setStatusMessage('');
    setMode('home');
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
            <h1>📡 {connected ? '✅ Conectado' : '🔄 Compartilhando'}</h1>
            <button onClick={disconnect} className="disconnect-btn">
              {connected ? '🔌 Desconectar' : '← Voltar'}
            </button>
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
                  className="copy-btn"
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

          <div className="status-box">
            <h3>📊 Status da Conexão</h3>
            <div className={`status-indicator ${connected ? 'connected' : 'waiting'}`}>
              <span className="status-dot"></span>
              <p>{statusMessage || (connected ? '✅ Tela conectada' : 'Aguardando conexão...')}</p>
            </div>
            {connected && (
              <p className="connection-details">
                ID: {connectionState?.uuid?.substring(0, 8)}... | Tipo: PC Principal
              </p>
            )}
          </div>
        </div>
      )}

      {mode === 'secondary' && (
        <div className="secondary-screen">
          <div className="header">
            <h1>🔗 {connected ? '✅ Conectado' : '🔄 Conectar'}</h1>
            <button onClick={disconnect} className="disconnect-btn">
              {connected ? '🔌 Desconectar' : '← Voltar'}
            </button>
          </div>

          {!connected ? (
            <div className="connect-box">
              <label>Digite o código do PC Principal:</label>
              <div className="input-group">
                <input
                  type="text"
                  placeholder="Ex: ABCD1234"
                  maxLength={9}
                  value={inputCode}
                  onChange={(e) => {
                    setInputCode(e.target.value.toUpperCase());
                    setError('');
                  }}
                  disabled={loading}
                />
                <button onClick={() => connectToSecondary(inputCode)} disabled={loading}>
                  {loading ? '🔄 Conectando...' : 'Conectar'}
                </button>
              </div>
              
              {error && (
                <div className="error-message">
                  <p>{error}</p>
                </div>
              )}
              
              {statusMessage && (
                <div className="status-message">
                  <p>{statusMessage}</p>
                </div>
              )}
            </div>
          ) : (
            <div className="remote-display">
              <div className={`status-indicator ${connected ? 'connected' : 'waiting'}`}>
                <span className="status-dot"></span>
                <p>{statusMessage || '✅ Conectado com sucesso!'}</p>
              </div>
              
              {connectionState && (
                <p className="connection-details">
                  ID: {connectionState.uuid?.substring(0, 8)}... | Código: {connectionState.code}
                </p>
              )}
              
              <div className="video-container">
                <p>📺 Aguardando stream de vídeo...</p>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
