# Guia de Deployment e Releases do KVM Pro

Este guia explica como publicar versões do KVM Pro e configurar auto-updates para usuários.

## Pré-requisitos

- Conta GitHub com repositório em `github.com/seu-usuario/kvm-pro`
- Git instalado e configurado
- Rust 1.70+ instalado
- (Opcional) Actions habilitadas no repositório

## Fluxo de Release Automático com GitHub Actions

### 1. Configuração Inicial (uma vez)

```bash
# Clonar repositório
git clone https://github.com/seu-usuario/kvm-pro.git
cd kvm-pro

# Verificar que os workflows estão no lugar
ls -la .github/workflows/
# Deve mostrar: ci.yml e release.yml
```

### 2. Fazer uma Release

#### Passo 1: Preparar Mudanças

```bash
# Fazer commits normalmente
git add .
git commit -m "Feature: adicionar suporte a X"
git commit -m "Fix: corrigir bug Y"

# O workflow CI roda automaticamente, verificando:
# - Testes (cargo test)
# - Formatação (cargo fmt)
# - Lints (cargo clippy)
```

#### Passo 2: Criar Tag de Release

```bash
# Semântica: v{MAJOR}.{MINOR}.{PATCH}
git tag -a v1.0.0 -m "Release v1.0.0 - Initial release"

# Se quiser atualizar a tag depois
git tag -f -a v1.0.0 -m "Release v1.0.0 - Initial release"
git push -f origin v1.0.0
```

#### Passo 3: Push para GitHub

```bash
# Push código
git push origin main

# Push tag
git push origin v1.0.0

# Ou push tudo de uma vez
git push origin main --tags
```

#### Passo 4: Workflow Roda Automaticamente

O GitHub Actions agora:
1. ✅ Compila para Linux (musl target - estático, sem dependências)
2. ✅ Compila para Windows (MinGW - estático, sem DLLs)
3. ✅ (Opcional) Cria AppImage para Linux universal
4. ✅ Cria release no GitHub com binários anexados
5. ✅ Notifica usuários via auto-update

**Tempo total**: ~10-15 minutos

### 3. Resultado

Usuários verão na próxima inicialização:

```
============================================
       KVM Pro - Update Available
============================================

Current version: v0.9.0
Latest version:  v1.0.0

Would you like to update now? (y/n)
```

Se escolherem `y`:
- Download: ~10-20 segundos (depende conexão)
- Instalação: ~5 segundos
- Pronto para usar

## Versões do Release

### Release Estável

Para releases normais (recomendado):

```bash
git tag -a v1.2.3 -m "Release v1.2.3"
git push origin v1.2.3
```

Resultado: Versão normal no GitHub, sem marcação especial

### Release Beta

Para versões de teste:

```bash
git tag -a v1.2.3-beta.1 -m "Beta release v1.2.3-beta.1"
git push origin v1.2.3-beta.1
```

No GitHub Actions, isso criará release marcada como "Pre-release"

### Release Candidate

Para candidatos a versão final:

```bash
git tag -a v1.2.3-rc.1 -m "Release candidate v1.2.3-rc.1"
git push origin v1.2.3-rc.1
```

## Workflow Manual (sem GitHub Actions)

Se preferir não usar Actions:

### 1. Compilar Localmente

```bash
# Linux
./scripts/portable-build.sh

# Windows (em WSL/Linux)
./scripts/windows-build.sh

# Resultado em dist/
# - kvm-pro-linux.tar.gz (~10-15MB)
# - kvm-pro-windows.zip (~20-30MB)
# - kvm-pro-linux.AppImage (~20MB) opcional
```

### 2. Criar Release Manualmente

```bash
# Em GitHub.com:
# 1. Ir para Releases
# 2. Click "Create a new release"
# 3. Tag: v1.2.3
# 4. Title: KVM Pro v1.2.3
# 5. Upload: kvm-pro-linux.tar.gz, kvm-pro-windows.zip, kvm-pro-linux.AppImage
# 6. Publish
```

### 3. Desabilitar Auto-Update de Pre-Release

Se sua versão for beta/rc, editar `scripts/auto-update.sh`:

```bash
# Adicionar ao topo:
ALLOW_PRERELEASE=false  # true para beta, false para estável
```

## Estrutura de Release Esperada

O GitHub Actions criará automaticamente:

```
Release v1.2.3
├── kvm-pro-linux.tar.gz
│   ├── kvm-pro-server
│   ├── kvm-pro-client
│   ├── run-server.sh
│   ├── run-client.sh
│   ├── auto-update.sh
│   ├── kvm-pro.toml
│   └── LICENSE
├── kvm-pro-windows.zip
│   ├── kvm-pro-server.exe
│   ├── kvm-pro-client.exe
│   ├── run-server-with-update.bat
│   ├── run-client.bat
│   ├── auto-update.bat
│   ├── kvm-pro.toml
│   ├── README.md
│   └── LICENSE
└── kvm-pro-x86_64.AppImage
    └── (executável único, sem arquivos)
```

## Configurar Auto-Update

### Para Usuários Receberem Updates

A variável de ambiente deve estar configurada:

```bash
export GITHUB_REPO="seu-usuario/kvm-pro"
./run-server.sh
```

Ou editar os scripts:

**run-server.sh (Linux):**
```bash
GITHUB_REPO="${GITHUB_REPO:-seu-usuario/kvm-pro}"
```

**run-server-with-update.bat (Windows):**
```batch
if not defined GITHUB_REPO (
    set "GITHUB_REPO=seu-usuario/kvm-pro"
)
```

### Customizar Intervalo de Verificação

**auto-update.sh (Linux):**
```bash
CHECK_INTERVAL_DAYS=1  # Default
CHECK_INTERVAL_DAYS=7  # Semanal
CHECK_INTERVAL_DAYS=30 # Mensal
```

**auto-update.bat (Windows):**
```batch
set "CHECK_INTERVAL_DAYS=1"
```

## Monitorar Updates

### Logs de Update

Usuários podem verificar log de atualizações:

```bash
# Linux/macOS
tail -f ~/.kvm-pro-update.log

# Windows
type %USERPROFILE%\.kvm-pro-update.log
```

### Estatísticas (opcional)

Para coletar dados de downloads no GitHub:

```bash
# Ver download count
curl -s https://api.github.com/repos/seu-usuario/kvm-pro/releases | \
  jq -r '.[0] | "\(.name): \(.assets[].download_count) downloads"'
```

## Publicar em Outros Canais

### Releases no GitHub (automático via Actions)
✅ Já configurado

### AUR (Arch User Repository)
Criar PKGBUILD:

```bash
pkgname=kvm-pro
pkgver=1.0.0
pkgrel=1
pkgdesc="KVM - Keyboard Video Mouse Pro"
url="https://github.com/seu-usuario/kvm-pro"
```

### Homebrew (macOS, Linux)
Criar formula:

```ruby
class KvmPro < Formula
  desc "KVM - Keyboard Video Mouse Pro"
  homepage "https://github.com/seu-usuario/kvm-pro"
  url "https://github.com/seu-usuario/kvm-pro/releases/download/v1.0.0/kvm-pro-linux.tar.gz"
  sha256 "hash_do_arquivo"
end
```

### Scoop (Windows)
Criar manifest:

```json
{
  "version": "1.0.0",
  "url": "https://github.com/seu-usuario/kvm-pro/releases/download/v1.0.0/kvm-pro-windows.zip",
  "bin": "kvm-pro-server.exe"
}
```

## Troubleshooting

### Workflow não segue
```bash
# Verificar permissões
git push -v origin main --tags

# Verificar formatação de tag
git tag -l
# Deve mostrar: v1.2.3 (não v_1.2.3 ou VERSION_1.2.3)
```

### Build falha no Actions
1. Clicar em "Actions" no GitHub
2. Ir para workflow "Release Build"
3. Ver log completo
4. Verificar erros (geralmente: arquivos faltando, permissões)

### Users não recebem update
1. Verifique se GITHUB_REPO está correto no script
2. Verifique se GitHub repo é público (privado não funciona)
3. Verifique log: `~/.kvm-pro-update.log`

### Update trava
```bash
# Limpar cache de update
rm ~/.kvm-pro-update.log ~/.last-update-check

# Tenta novamente
./run-server.sh
```

## Checklist para Release

- [ ] Todos os testes passando (`cargo test`)
- [ ] Sem warnings lint (`cargo clippy`)
- [ ] Versão atualizada em `core/Cargo.toml` (version field)
- [ ] CHANGELOG.md atualizado
- [ ] Commits bem descritos
- [ ] Tag criada: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Tag pushed: `git push origin vX.Y.Z`
- [ ] Workflow ActionS completou (verificar em Actions tab)
- [ ] Release criada no GitHub com descrição legível aos usuários
- [ ] Binários verificados (testar download localmente)
- [ ] Anunciar no social media/forum

## Versionamento Semântico

Seguir [Semantic Versioning](https://semver.org):

- `v1.0.0` → Aumentar MAJOR para breaking changes
- `v1.1.0` → Aumentar MINOR para novas features (backward-compatible)
- `v1.0.1` → Aumentar PATCH para bug fixes
- `v1.0.0-beta.1` → Beta release (usuários testam antes)
- `v1.0.0-rc.1` → Release candidate

Exemplo progression:
```
v0.1.0 → v0.2.0 (novas features)
v0.2.0 → v0.2.1 (bug fix)
v0.2.1 → v1.0.0 (versão estável)
v1.0.0 → v1.1.0 (mais features)
```

---

**Próximas etapas:**
1. Substituir `seu-usuario` por seu GitHub username
2. Fazer push initial com primeira tag
3. Monitorar primeiro build no Actions
4. Testar download e auto-update
5. Publicar announcement para usuários
