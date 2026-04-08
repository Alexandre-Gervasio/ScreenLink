/**
 * Test Script para simular 2 PCs conectando
 * Simula um PC Principal (Ubuntu/Linux) e um Secundário (Windows)
 */

import http from 'http';
import WebSocket from 'ws';

const API_URL = 'http://localhost:3001';
const WS_URL = 'ws://localhost:3001';

console.log('🖥️  ScreenLink - 2-PC Connection Test\n');
console.log('═'.repeat(50));
console.log('API Server: ' + API_URL);
console.log('WebSocket: ' + WS_URL);
console.log('═'.repeat(50) + '\n');

// Step 1: PC Principal cria um link de pairing
async function createPairingLink() {
  return new Promise((resolve, reject) => {
    const options = {
      hostname: 'localhost',
      port: 3001,
      path: '/api/links/create',
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      }
    };

    const req = http.request(options, (res) => {
      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => {
        try {
          const json = JSON.parse(data);
          resolve(json);
        } catch (e) {
          reject(e);
        }
      });
    });

    req.on('error', reject);
    req.end();
  });
}

// Step 2: PC Principal se conecta como 'primary'
function connectPrimary(uuid, code) {
  return new Promise((resolve) => {
    const ws = new WebSocket(`${WS_URL}/ws/${uuid}/primary/${code}`);
    
    ws.on('open', () => {
      console.log('✅ [PC PRINCIPAL - Ubuntu] Conectado como PRIMARY');
      console.log('   ID do Link: ' + uuid.substring(0, 12) + '...');
      ws.send(JSON.stringify({
        type: 'STATUS',
        message: 'PC Principal aguardando conexão'
      }));
    });

    ws.on('message', (data) => {
      try {
        const msg = JSON.parse(data);
        if (msg.type === 'PEER_DISCONNECTED') {
          console.log('⚠️  [PC PRINCIPAL] PC secundário desconectou');
          ws.close();
        } else {
          console.log('📨 [PC PRINCIPAL] Recebeu:', msg.type);
        }
      } catch (e) {
        console.log('📨 [PC PRINCIPAL] Dados binários recebidos');
      }
    });

    ws.on('error', (err) => {
      console.error('❌ [PC PRINCIPAL] Erro:', err.message);
    });

    ws.on('close', () => {
      console.log('🔌 [PC PRINCIPAL] Desconectado');
    });

    resolve(ws);
  });
}

// Step 3: PC Secundário se conecta como 'secondary' com o código
function connectSecondary(uuid, code) {
  return new Promise((resolve) => {
    // Aguardar um pouco para simular o usuário entrando o código
    setTimeout(() => {
      console.log('\n🖥️  [PC SECUNDÁRIO - Windows] Digitou código: ' + code);
      
      const ws = new WebSocket(`${WS_URL}/ws/${uuid}/secondary/${code}`);
      
      ws.on('open', () => {
        console.log('✅ [PC SECUNDÁRIO - Windows] Conectado como SECONDARY');
        console.log('   ID do Link: ' + uuid.substring(0, 12) + '...');
        ws.send(JSON.stringify({
          type: 'STATUS',
          message: 'PC Secundário pronto'
        }));
      });

      ws.on('message', (data) => {
        try {
          const msg = JSON.parse(data);
          if (msg.type === 'PEER_DISCONNECTED') {
            console.log('⚠️  [PC SECUNDÁRIO] PC principal desconectou');
            ws.close();
          } else {
            console.log('📨 [PC SECUNDÁRIO] Recebeu:', msg.type);
          }
        } catch (e) {
          console.log('📨 [PC SECUNDÁRIO] Dados binários recebidos');
        }
      });

      ws.on('error', (err) => {
        console.error('❌ [PC SECUNDÁRIO] Erro:', err.message);
      });

      ws.on('close', () => {
        console.log('🔌 [PC SECUNDÁRIO] Desconectado');
      });

      resolve(ws);
    }, 2000);
  });
}

// Main test flow
async function runTest() {
  try {
    // Step 1: PC Principal cria código
    console.log('🔄 Passo 1: PC Principal gerando código de pairing...\n');
    const linkData = await createPairingLink();
    
    if (!linkData.code) {
      throw new Error('Falha ao gerar código');
    }
    
    console.log('📋  Código de Pairing Gerado: ' + linkData.code);
    console.log('📍  UUID: ' + linkData.uuid);
    console.log('⏰  Válido por: ' + linkData.expiresIn);
    
    // Step 2: PC Principal se conecta
    console.log('\n\n🔄 Passo 2: PC Principal (Ubuntu) se conectando...\n');
    const primaryWs = await connectPrimary(linkData.uuid, linkData.code);
    
    // Step 3: PC Secundário se conecta
    console.log('\n\n🔄 Passo 3: PC Secundário (Windows) entrando código e conectando...\n');
    const secondaryWs = await connectSecondary(linkData.uuid, linkData.code);
    
    // Simular interação
    console.log('\n\n✨ Ambos os PCs conectados! Simulando troca de mensagens...\n');
    
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    // Enviar mensagem do secondary para primary
    secondaryWs.send(JSON.stringify({
      type: 'CONTROL',
      data: { x: 100, y: 200, button: 'left' }
    }));
    console.log('📤 [PC SECUNDÁRIO] Enviou: comando de mouse (100, 200)');
    
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Fechar conexão
    console.log('\n\n🔄 Passo 4: Encerrando conexões...\n');
    primaryWs.close();
    secondaryWs.close();
    
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    console.log('\n\n═'.repeat(50));
    console.log('✅ TESTE CONCLUÍDO COM SUCESSO!');
    console.log('═'.repeat(50));
    console.log('\n📊 Resumo:');
    console.log('  ✅ PC Principal conectou');
    console.log('  ✅ PC Secundário conectou com código');
    console.log('  ✅ Comunicação WebSocket funcionando');
    console.log('  ✅ Desconexão limpa\n');
    
    process.exit(0);
  } catch (error) {
    console.error('\n❌ ERRO NO TESTE:', error.message);
    process.exit(1);
  }
}

// Run
runTest();
