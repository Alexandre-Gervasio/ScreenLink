# 🎛️ KVM Pro Control Panel GUI

## Overview | Visão Geral

Web-based graphical interface for controlling KVM Pro server and client instances.

Interface gráfica baseada em web para controlar instâncias do servidor e cliente KVM Pro.

## Features

- ✅ Beautiful, responsive web interface
- ✅ Start/Stop server button
- ✅ Connect/Disconnect client button  
- ✅ Real-time status updates
- ✅ Bilingual support (English/Portuguese)
- ✅ Cross-platform (Linux & Windows)
- ✅ No additional dependencies required

## Usage

### Linux
```bash
./dist/kvm-pro-ui-linux
```

### Windows
```cmd
dist\kvm-pro-ui.exe
```

### Access GUI
1. Open your web browser
2. Navigate to: **http://127.0.0.1:8080**
3. Click buttons to control server and client

## Interface

```
┌─────────────────────────────────────┐
│         🎛️ KVM Pro v1.0.1            │
│                                       │
│  📡 Servidor (Server)                │
│  [🟢 Ligar] [🔴 Desligar]           │
│                                       │
│  💻 Cliente (Client)                 │
│  [🟢 Conectar] [🔴 Desconectar]     │
│                                       │
│  Status: ✅ Pronto para usar         │
└─────────────────────────────────────┘
```

## Requirements

- Rust 1.70+ (for compilation only)
- Web browser (any modern browser)
- Network connectivity

## Building from Source

```bash
cd /path/to/kvm-pro
cargo build --release --bin kvm-pro-ui

# For Windows cross-compilation from Linux
cargo build --release --target x86_64-pc-windows-gnu --bin kvm-pro-ui
```

## API Endpoints

- `GET /` - Main UI page
- `POST /api/server/start` - Start server
- `POST /api/server/stop` - Stop server
- `POST /api/client/start` - Start client
- `POST /api/client/stop` - Stop client

## Technologies

- **Framework**: Actix-web 4.x
- **Language**: Rust 1.70+
- **Frontend**: HTML5 + CSS3 + Vanilla JavaScript

## Troubleshooting

**Port 8080 already in use?**
Modify the port in `core/src/ui.rs` line 21 and recompile.

**Server/Client fails to start?**
Ensure `kvm-pro-server` and `kvm-pro-client` binaries are in your PATH or same directory.

**Browser shows connection refused?**
- Check firewall settings
- Ensure UI binary is running: `./dist/kvm-pro-ui-linux`
- Try opening http://localhost:8080 instead

## License

MIT License - See LICENSE file

---

## Português

### Como Usar

1. Execute o programa GUI
2. Abra seu navegador em: **http://127.0.0.1:8080**
3. Clique nos botões para controlar servidor e cliente

### Requisitos

- Rust 1.70+ (apenas para compilação)
- Navegador web moderno
- Conexão de rede

### Troubleshooting

**Porta 8080 já em uso?**
Modifique a porta em `core/src/ui.rs` linha 21 e recompile.

**Servidor/Cliente não inicia?**
Certifique-se que os binários `kvm-pro-server` e `kvm-pro-client` estão disponíveis.

**Navegador mostra conexão recusada?**
- Verifique as configurações de firewall
- Certifique-se que o programa GUI está em execução
- Tente abrir http://localhost:8080

