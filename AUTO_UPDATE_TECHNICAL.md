# Auto-Update System - Resumo Técnico

## Visão Geral

O KVM Pro possui um sistema completo de auto-update que permite usuários receberem atualizações automaticamente quando novas versões são publicadas no GitHub, sem necessidade de ação manual.

## Arquitetura

```
┌─────────────────────────────────────────────────────┐
│                  User Start                          │
└────────────────┬────────────────────────────────────┘
                 │
                 ▼
        ┌────────────────────┐
        │  run-server.sh     │
        │  (ou .bat)         │
        └────────┬───────────┘
                 │
                 ▼
    ┌───────────────────────────┐
    │  auto-update.sh (ou .bat) │
    │  - Check interval ✓       │
    │  - GitHub API query ✓     │
    │  - Version compare ✓      │
    │  - User prompt ✓          │
    └────┬────────────┬─────────┘
         │            │
    No update       Update available
         │            │
         └────┬───────┘
              │
              ▼
    ┌────────────────────────────┐
    │  Download (tar.gz/zip)     │
    │  - 10-50MB                 │
    │  - To temp directory       │
    └────┬──────────────────────┘
         │
         ▼
    ┌────────────────────────────┐
    │  Backup current version    │
    │  - backup-v0.1.0/          │
    │  - kvm-pro-server          │
    │  - kvm-pro-client          │
    └────┬──────────────────────┘
         │
         ▼
    ┌────────────────────────────┐
    │  Extract & Install         │
    │  - Replace binaries        │
    │  - Keep config intact      │
    └────┬──────────────────────┘
         │
         ▼
    ┌────────────────────────────┐
    │  Start Server/Client       │
    │  - Now v1.0.0              │
    └────────────────────────────┘
```

## Componentes

### 1. Scripts de Launcher

**Linux:** `run-server.sh`, `run-client.sh`
```bash
# Chama auto-update.sh
# Depois executa binário
```

**Windows:** `run-server-with-update.bat`, `run-client.bat`
```batch
REM Chama auto-update.bat
REM Depois executa binário
```

### 2. Auto-Update Scripts

#### Linux: `scripts/auto-update.sh` (~220 linhas)
- ✅ Verifica intervalo de dias (configurable)
- ✅ Consulta GitHub API (`/repos/{owner}/{repo}/releases/latest`)
- ✅ Compara versões (tag vs binary --version)
- ✅ Download de pacote (tar.gz)
- ✅ Backup de versão anterior
- ✅ Extração e instalação
- ✅ Rollback em caso de erro
- ✅ Logging em `~/.kvm-pro-update.log`

#### Windows: `scripts/auto-update.bat` (~200 linhas)
- ✅ Mesma funcionalidade do Linux
- ✅ Compatível com cmd.exe
- ✅ Usa PowerShell para operações complexas
- ✅ Logging em `%USERPROFILE%\.kvm-pro-update.log`

### 3. Módulo Rust (Rust Library)

**Arquivo:** `core/src/utils/update_checker.rs` (~100 linhas)

Estrutura:
```rust
pub struct UpdateChecker {
    github_repo: String,
    check_interval_days: u64,
}

pub struct VersionInfo {
    current: String,
    latest: Option<String>,
    release_url: Option<String>,
}

impl UpdateChecker {
    pub async fn check_for_updates(&self) -> Result<Option<VersionInfo>>
    pub fn should_check(&self) -> bool
    pub fn mark_checked(&self) -> Result<()>
}
```

Uso:
```rust
let checker = UpdateChecker::new("seu-usuario/kvm-pro", 1); // 1 dia
if let Ok(info) = checker.check_for_updates().await {
    if info.latest.is_some() {
        println!("Update available!");
    }
}
```

### 4. GitHub Actions Workflows

**`.github/workflows/release.yml`**
- ✅ Trigger: push tag v*.*.*
- ✅ Build Linux estático (musl)
- ✅ Build Windows estático (MinGW)
- ✅ Build AppImage (Universal)
- ✅ Criar release no GitHub
- ✅ Upload de binários
- ✅ Tempo total: ~15 minutos

**`.github/workflows/ci.yml`**
- ✅ Trigger: push/PR em main/develop
- ✅ Testes (cargo test)
- ✅ Formatação (cargo fmt)
- ✅ Lints (cargo clippy)
- ✅ Build para múltiplas plataformas

### 5. Documentação

**`AUTO_UPDATE_USER_GUIDE.md`**
- Como funciona o auto-update
- Como desabilitar (opcional)
- Como atualizar manualmente
- Como restaurar versão anterior
- Solução de problemas
- Configurações avançadas

**`DEPLOYMENT_GUIDE.md`**
- Processo completo de release
- Passo a passo com git/GitHub
- Workflow automático vs manual
- Monitorar updates
- Publicar em outros canais (AUR, Homebrew, Scoop)
- Troubleshooting

**`CHANGELOG.md`**
- Template para documentar mudanças
- Semântica de versionamento
- Checklist para releases

## Fluxo de Atualização

### Do Ponto de Vista do Desenvolvedor

```
1. Work (commits)
   git commit -am "Feature: add X"

2. Test (CI runs automatically)
   Testes passam? ✓
   Lint OK? ✓
   Build OK? ✓

3. Release (create tag)
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0

4. Build (GitHub Actions)
   ✓ Compile Linux (musl)
   ✓ Compile Windows (MinGW)
   ✓ Create packages
   ✓ Upload to release

5. Done!
   Users automatically notified
```

**Tempo total:** ~5 minutos (trabalho) + ~15 minutos (Actions)

### Do Ponto de Vista do Usuário

```
1. Start Server
   ./run-server.sh
   
2. Auto-Check
   ✓ Version check (1-2 seg)
   
3. Found Update?
   YES → Prompt user
   NO  → Continue directly
   
4. User Choice
   [y] - Download & install (30-60 seg)
   [n] - Use current version
   
5. Result
   ✓ Updated to v1.0.0
   ✓ Ready to use
```

**Tempo total:** 1-2 segundos (sem update) ou 30-60 segundos (com update)

## Configuração

### Para o Desenvolvedor

#### 1. Preparar Repositório

```bash
# Criar workflows (já feito)
mkdir -p .github/workflows
# Copiar release.yml e ci.yml

# Primeira tag
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0
```

#### 2. Versioner

Manter `core/Cargo.toml` atualizado:
```toml
[package]
version = "0.1.0"  # Atualizar aqui
```

#### 3. Release Script
```bash
#!/bin/bash
VERSION=$(./scripts/get-version.sh --version)
git tag -a "v$VERSION" -m "Release v$VERSION"
git push origin "v$VERSION"
```

### Para o Usuário Final

Nada a fazer! Tudo funciona automaticamente.

Opcionalmente:
```bash
# Desabilitar auto-update
touch ~/.no-auto-update

# Ou editar run-server.sh para remover auto-update.sh call
```

## Segurança

### Verificações Implementadas

✅ **HTTPS**: Todas as chamadas GitHub API usam HTTPS
✅ **Validação**: Verifica se arquivo baixado tem tamanho esperado
✅ **Backup automaticamente**: Antes de instalar, faz backup
✅ **Rollback**: Se instalação falhar, restaura versão anterior
✅ **Permissões**: Binários extraídos com permissões corretas
✅ **Logging**: Cada operação é registrada em log

### O Que NÃO é Verificado

⚠️ **Assinatura**: Não verifica assinatura GPG (can be added)
⚠️ **Hashes**: Não verifica SHA256 (can be added)
⚠️ **Certificado SSL**: Confia em certificados padrão

### Como Melhorar

Para produção, adicionar:

1. **Assinatura GPG**
```bash
# Assinar release
gpg --detach-sign kvm-pro-linux.tar.gz

# Verificar no update
gpg --verify kvm-pro-linux.tar.gz.sig kvm-pro-linux.tar.gz
```

2. **SHA256 Checksum**
```bash
# Gerar hashes
sha256sum kvm-pro-*.tar.gz > CHECKSUMS.txt

# Verificar no update
sha256sum -c CHECKSUMS.txt
```

## Limites Conhecidos

### Velocidade

- **Verificação**: 1-2 segundos (requer conexão)
- **Download**: 10-50 segundos (depende conexão)
- **Instalação**: 5-10 segundos
- **Total**: ~1 minuto em conexão média

### Rede

- Requer acesso a `api.github.com` (porta 443)
- Firewall corporativo pode bloquear
- Função graceful se GitHub indisponível (skip update)

### Armazenamento

- Requer ~2x tamanho da app no disco (para backup)
- Exemplo: App 30MB → precisa 60MB livre

### Plataformas

- ✅ Linux (glibc e musl)
- ✅ Windows (7+)
- ⚠️ macOS (estrutura pronta, não testado)
- ❌ BSD (pode funcionar)

## Monitoramento

### Logs

**Verificar se auto-update rodou:**
```bash
tail ~/.kvm-pro-update.log
```

**Saída esperada:**
```
[2024-01-15 10:23:45] Auto-update started
[2024-01-15 10:23:45] Current version: v0.1.0
[2024-01-15 10:23:47] Latest version available: v1.0.0
[2024-01-15 10:23:47] Update available: v0.1.0 -> v1.0.0
```

### Métricas (GitHub)

```bash
# Ver quantos downloads cada versão teve
curl -s https://api.github.com/repos/seu-usuario/kvm-pro/releases | \
  jq -r '.[0] | "\(.tag_name): \(.assets[].download_count) downloads"'
```

### Alertas

Se update falhar, usuário verá:
```
Error: Could not download update
Check ~/.kvm-pro-update.log for details
Continuing with current version...
```

## Próximas Melhorias

- [ ] Carregar AppImage diretamente (sem extração)
- [ ] Suporte a delta updates (download só diff)
- [ ] UI native para prompt de update (Gtk/Qt)
- [ ] Assinatura GPG das releases
- [ ] Múltiplos mirrors para download
- [ ] Staged rollout (% crescente de usuários)
- [ ] Notificação de update mesmo com app fechado
- [ ] Auto-restart de apps após update

## Referências

- [GitHub API - Releases](https://docs.github.com/en/rest/releases)
- [GitHub Actions](https://docs.github.com/en/actions)
- [Semantic Versioning](https://semver.org)
- [Keep a Changelog](https://keepachangelog.com)

---

**Resumo:** Sistema completo, automático, seguro e amigável para distribuir updates. Usuários recebem novas versões sem esforço, desenvolvedores publicam com um simples git tag.
