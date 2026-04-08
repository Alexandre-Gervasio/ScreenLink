# 🖥️ Como Usar ScreenLink

> Baixe, instale e use em seu sistema operacional

---

## 📥 Download

**Acesse: https://github.com/Alexandre-Gervasio/ScreenLink/releases**

### Escolha seu Sistema Operacional:

#### 🟢 Instaladores (Recomendado)

| Sistema | Arquivo | 
|---------|---------|
| 🪟 **Windows** | `ScreenLink_1.0.0_x64_en-US.msi` |
| 🍎 **macOS** | `ScreenLink_1.0.0_aarch64.dmg` |
| 🐧 **Linux** | `ScreenLink_1.0.0_amd64.AppImage` |

#### 🟡 Portáveis (Sem instalação - Nenhuma permissão de admin)

| Sistema | Arquivo | 
|---------|---------|
| 🪟 **Windows Portátil** | `ScreenLink-1.0.0-windows-portable.zip` |
| 🍎 **macOS Portátil** | `ScreenLink-1.0.0-macos-portable.zip` |
| 🐧 **Linux Portátil** | `ScreenLink-1.0.0-linux-portable.zip` |

---

## 💾 Instalação

### Windows
1. Baixe `.msi`
2. Clique 2x para instalar
3. Procure "ScreenLink" no Menu Iniciar

### macOS
1. Baixe `.dmg`
2. Arraste para "Applications"
3. Procure "ScreenLink" no Spotlight (Cmd + Espaço)

### Linux
```bash
chmod +x ScreenLink_1.0.0_amd64.AppImage
./ScreenLink_1.0.0_amd64.AppImage
```

#### ⚠️ Se o AppImage não abrir no Linux:

**Problema**: Erro de permissão ou FUSE não disponível

**Solução 1 - Extrair e executar:**
```bash
chmod +x ScreenLink_1.0.0_amd64.AppImage
./ScreenLink_1.0.0_amd64.AppImage --appimage-extract
cd squashfs-root
./AppRun
```

**Solução 2 - Usar modo FUSE fallback:**
```bash
export APPIMAGE_EXTRACT_AND_RUN=1
chmod +x ScreenLink_1.0.0_amd64.AppImage
./ScreenLink_1.0.0_amd64.AppImage
```

**Solução 3 - Usar versão portátil (não precisa de FUSE):**
1. Descompacte `ScreenLink-1.0.0-linux-portable.zip`
2. Execute `./run.sh`

---

## 📦 Versão Portátil (Sem Instalação)

### Windows Portátil
1. Descompacte `ScreenLink-1.0.0-windows-portable.zip`
2. Execute `ScreenLink.exe` ou `run.bat`

### macOS Portátil
1. Descompacte `ScreenLink-1.0.0-macos-portable.zip`
2. Execute `./run.sh` ou clique em `ScreenLink.app`

### Linux Portátil
1. Descompacte `ScreenLink-1.0.0-linux-portable.zip`
2. Execute `./run.sh`

```bash
unzip ScreenLink-1.0.0-linux-portable.zip
cd ScreenLink-1.0.0-linux-portable
./run.sh
```

Se receber erro de permissão:
```bash
chmod +x run.sh ScreenLink.AppImage
./run.sh
```

---

## 🚀 Como Usar

### PC Primário (com monitor):
1. Abra ScreenLink
2. Copie o código mostrado
3. Envie para o outro PC

### PC Secundário (sem monitor):
1. Abra ScreenLink
2. Cole o código
3. Clique "Conectar"

### Use!
- Mova o mouse para a borda direita do monitor
- Ele aparecerá no PC secundário como monitor estendido

---

## 🔗 Requisitos

✓ Ambos os PCs na mesma rede  
✓ Conexão estável (10+ Mbps)

---

## ⚙️ Configurações

**Qualidade (se lento):**
- Menu ⚙️ → Qualidade Baixa (720p)
- [Aplicar]

**Desconectar:**
- Clique [Desconectar]

---

## 🆘 Problemas?

**Não conecta:**
- Verifique se estão na mesma rede
- Permita ScreenLink no firewall

**macOS não abre:**
- Settings → Security → "Open Anyway"

**Latência alta:**
- Use Ethernet em vez de WiFi
- Reduza qualidade para 720p

---

**Pronto para usar!** 🎉
