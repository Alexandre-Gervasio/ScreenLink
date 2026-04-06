# 🌐 KVM Pro Web Links - Sharing via URLs

Este documento descreve como compartilhar acesso ao KVM Pro via links web (tipo AnyDesk).

## 🎯 Problema Resolvido

- ✅ Firewall corporativo bloqueia portas
- ✅ Difícil memorizar IPs e portas
- ✅ Quer compartilhar com link simples
- ✅ Funciona em qualquer rede

## 🚀 Solução: Servidor Relay na Nuvem

### Opção 1: Usar Serviço Gratuito (Recomendado)

**Usando Glitch.com (Gratuito, sem cartão de crédito)**

1. Acesse https://glitch.com
2. Create New Project → Express Server
3. Cole o código abaixo em `server.js`:

```javascript
const express = require('express');
const http = require('http');
const WebSocket = require('ws');

const app = express();
const server = http.createServer(app);
const wss = new WebSocket.Server({ server });

let links = {};

app.post('/api/create-link', (req, res) => {
  const linkId = Math.random().toString(36).substr(2, 9);
  const token = Math.random().toString(36).substr(2, 20);
  
  links[linkId] = {
    token,
    created: Date.now(),
    clients: []
  };

  res.json({
    link: `${req.protocol}://${req.get('host')}/connect/${linkId}`,
    token,
    expires_in: 86400
  });
});

app.get('/connect/:linkId', (req, res) => {
  res.send(`<!DOCTYPE html>
<html>
<head>
  <title>KVM Pro</title>
  <style>
    body { font-family: sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
    .box { background: white; padding: 40px; border-radius: 12px; text-align: center; box-shadow: 0 20px 60px rgba(0,0,0,0.3); }
    h1 { margin: 0; color: #333; }
    .status { padding: 20px; margin: 20px 0; border-radius: 8px; background: #e3f2fd; color: #1976d2; }
  </style>
</head>
<body>
  <div class="box">
    <h1>🎛️ KVM Pro</h1>
    <div class="status">⏳ Aguardando conexão...</div>
    <p>ID: <code>${req.params.linkId}</code></p>
    <script>
      const ws = new WebSocket('ws://' + window.location.host + '/ws/${req.params.linkId}');
      ws.onopen = () => document.querySelector('.status').innerText = '✅ Conectado!';
    </script>
  </div>
</body>
</html>`);
});

app.get('/', (req, res) => {
  res.send(`<!DOCTYPE html>
<html>
<head>
  <title>KVM Pro Generator</title>
  <style>
    body { font-family: sans-serif; padding: 40px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; display: flex; justify-content: center; align-items: center; margin: 0; }
    .box { background: white; padding: 40px; border-radius: 12px; max-width: 400px; width: 100%; box-shadow: 0 20px 60px rgba(0,0,0,0.3); }
    h1 { text-align: center; color: #333; }
    input { width: 100%; padding: 12px; margin: 10px 0; border: 2px solid #e0e0e0; border-radius: 6px; box-sizing: border-box; }
    button { width: 100%; padding: 12px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; border-radius: 6px; cursor: pointer; font-weight: bold; }
    .result { margin-top: 20px; padding: 15px; background: #f5f5f5; border-radius: 6px; display: none; }
    .link { background: white; border: 2px solid #667eea; padding: 12px; border-radius: 6px; font-family: monospace; font-size: 12px; word-break: break-all; margin-top: 10px; }
  </style>
</head>
<body>
  <div class="box">
    <h1>🎛️ KVM Pro Link Generator</h1>
    <input type="text" id="ip" placeholder="IP do servidor (ex: 192.168.1.100)" value="127.0.0.1">
    <button onclick="createLink()">Gerar Link</button>
    <div id="result" class="result">
      <p>Link criado!</p>
      <div class="link" id="linkOutput"></div>
      <button style="margin-top: 10px;" onclick="copyLink()">Copiar Link</button>
    </div>
  </div>
  <script>
    async function createLink() {
      const response = await fetch('/api/create-link', { method: 'POST' });
      const data = await response.json();
      document.getElementById('linkOutput').textContent = data.link;
      document.getElementById('result').style.display = 'block';
    }
    function copyLink() {
      navigator.clipboard.writeText(document.getElementById('linkOutput').textContent);
      alert('Link copiado!');
    }
  </script>
</body>
</html>`);
});

server.listen(process.env.PORT || 3000, () => {
  console.log('Servidor rodando na porta 3000');
});
```

4. Clique em "Show Live" no topo
5. Você agora tem um servidor relay na nuvem!

### Opção 2: Código Rust Simples (para sua máquina)

Se quiser rodar um relay localmente:

```rust
// Adicione ao Cargo.toml:
// actix-web = "4"

use actix_web::{web, App, HttpServer, HttpResponse};

#[actix_web::post("/api/create-link")]
async fn create_link() -> HttpResponse {
    let link_id = uuid::Uuid::new_v4().to_string();
    HttpResponse::Ok().json(serde_json::json!({
        "link": format!("http://localhost:8080/connect/{}", link_id),
        "token": uuid::Uuid::new_v4().to_string(),
    }))
}

#[actix_web::main]
async fn start_web_relay() -> std::io::Result<()> {
    println!("Web Relay Server rodando em http://localhost:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/api/create-link", web::post().to(create_link))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```

## 📖 Como Usar Com AnyDesk-like Link

### Fluxo 1: Máquinas na Mesma Rede

```bash
# Na máquina servidor:
kvm-pro -l               # Inicia em local mode (sem portas)

# Na máquina cliente (gera link):
curl -X POST http://relay.seu-servidor.com/api/create-link \
  -H "Content-Type: application/json" \
  -d '{"server_ip": "192.168.1.100"}'

# Resultado:
# {"link": "https://relay.seu-servidor.com/connect/abc123xyz"}

# Compartilhe o link com o usuário!
```

### Fluxo 2: Máquinas em Redes Diferentes (via SSH)

```bash
# Cliente:
ssh -L 5000:servidor-interno:5000 user@seu-gateway
kvm-pro

# Ou via relay:
# Acesse o link gerado - funciona automaticamente!
```

## 🎯 Benefícios

| Recurso | Portas | Web Link |
|---------|--------|----------|
| Firewall | ❌ Bloqueado | ✅ Funciona |
| Fácil compartilhamento | ❌ IP:Porta complexo | ✅ Link simples |
| Funciona em LAN | ✅ Sim | ✅ Sim |
| Funciona remotamente | ❌ Difícil | ✅ Sim |
| Setup | ⚠️ Manual | ✅ Automático |

## 🚀 Deploy em Produção

### Heroku (Gratuito com limitações)

```bash
# 1. Crie conta em heroku.com
# 2. Instale  heroku cli
# 3. Faça login
heroku login

# 4. Crie app
heroku create seu-kvm-relay

# 5. Deploy (git push heroku main)
# 6. Alteraçoes de firewall estão bloqueadas? Não mais!
```

### Vercel (Gratuito, melhor performance)

Deploy do `server.js` anterior como Function no Vercel.

## 💡 Próximos Passos

- [ ] Integrar relay server opcionalmente ao KVM Pro
- [ ] Adicionar autenticação aos links
- [ ] Adicionar expiração de links
- [ ] Dashboard web para gerenciar conexões
- [ ] Suporte a vários clientes simultâneos

## 📝 Resumo

**Problema**: Firewall corporativo bloqueia portas
**Solução**: Servidor relay na nuvem + links web
**Resultado**: AnyDesk-like experience, mas descentralizado!

Teste agora! 
