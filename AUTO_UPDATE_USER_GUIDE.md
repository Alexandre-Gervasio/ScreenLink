# Atualizações Automáticas do KVM Pro

## Como Funciona

O KVM Pro verifica automaticamente por atualizações toda vez que inicia. Se uma nova versão estiver disponível no GitHub, você será notificado com opção de atualizar imediatamente ou depois.

### Fluxo de Atualização

1. **Detecção**: Ao iniciar o servidor/cliente, o sistema verifica a versão mais recente no GitHub
2. **Aviso**: Se houver nova versão, um diálogo pergunta se você deseja atualizar
3. **Download**: A nova versão é baixada para pasta temporária
4. **Backup**: Versão atual é salva em pasta de backup (pode restaurar se necessário)
5. **Instalação**: Nova versão substitui a atual
6. **Conclusão**: Sistema retorna ao normal com versão atualizada

## Instalação Manual

Se não quer atualizar automaticamente, pode baixar manualmente:

### Linux/macOS

```bash
# Extrair nova versão sobre a instalação atual
tar -xzf kvm-pro-linux.tar.gz --strip-components=1 -C /sua/pasta/kvm-pro/
```

### Windows

```batch
REM Extrair novo ZIP sobre a instalação atual
powershell -Command "Expand-Archive -Path 'kvm-pro-windows.zip' -DestinationPath 'C:\caminho\para\kvm-pro' -Force"
```

## Desabilitar Atualizações Automáticas

Se preferir controlar manualmente quando atualizar:

### Linux/macOS

Editar `run-server.sh` e remover a seção de auto-update:

```bash
# Comentar esta seção:
# echo "Checking for updates..."
# if [ -f "$SCRIPT_DIR/auto-update.sh" ]; then
#     GITHUB_REPO="$GITHUB_REPO" bash "$SCRIPT_DIR/auto-update.sh" "$SCRIPT_DIR" || true
# fi
```

Ou criar um arquivo `.no-auto-update` na pasta de instalação:

```bash
touch ~/.kvm-pro/.no-auto-update
```

### Windows

Editar `run-server-with-update.bat` e remover a chamada ao auto-update:

```batch
REM Se exist "auto-update.bat" (
REM     call auto-update.bat "%CD%"
REM )
```

## Restaurar Versão Anterior

Se algo der errado com uma atualização:

### Linux/macOS

```bash
# Ver backups disponíveis
ls -la ~/.kvm-pro/backup-*/

# Restaurar versão anterior
cp ~/.kvm-pro/backup-v1.2.0/kvm-pro-server ~/.kvm-pro/
cp ~/.kvm-pro/backup-v1.2.0/kvm-pro-client ~/.kvm-pro/

# Reiniciar
```

### Windows

```batch
REM Ver backups disponíveis
dir C:\Users\usuario\backup-*

REM Restaurar versão anterior
xcopy C:\Users\usuario\backup-v1.2.0\kvm-pro-server.exe C:\caminho\para\kvm-pro\ /Y
xcopy C:\Users\usuario\backup-v1.2.0\kvm-pro-client.exe C:\caminho\para\kvm-pro\ /Y

REM Reiniciar
```

## Histórico de Atualizações

Todas as tentativas de atualização são registradas em:

- **Linux/macOS**: `~/.kvm-pro-update.log`
- **Windows**: `%USERPROFILE%\.kvm-pro-update.log`

Utilize para diagnosticar problemas de atualização.

## Solução de Problemas

### "Erro ao conectar no GitHub"

- Verifique conexão com internet
- Verifique se firewall permite acesso a `api.github.com`
- Tente atualizar manualmente depois

### "Atualização falhou"

- O script tenta restaurar versão anterior automaticamente
- Se restauração não funcionar, use backup manual (ver acima)
- Consulte log de atualização para detalhes

### "Versão não muda após atualizar"

- Reinicie completamente (feche todos os processos KVM)
- Verifique arquivo de versão em:
  - Linux: `./kvm-pro-server --version`
  - Windows: `kvm-pro-server.exe --version`

## Configuração Avançada

### Intervalo de Verificação

Editar `auto-update.sh` ou `auto-update.bat`:

```bash
CHECK_INTERVAL_DAYS=1  # Verificar diariamente (padrão)
CHECK_INTERVAL_DAYS=7  # Verificar semanalmente
CHECK_INTERVAL_DAYS=30 # Verificar mensalmente
```

### Repositório Customizado

```bash
export GITHUB_REPO="seu-usuario/seu-fork"
./run-server.sh
```

### Desabilitar Temporariamente

```bash
# Apenas esta sessão
GITHUB_REPO=skip ./run-server.sh

# Ou em batch:
set GITHUB_REPO=skip
run-server-with-update.bat
```

## Publicar Atualizações

Como publicador/mantenedor do projeto:

1. Commit suas mudanças: `git commit -am "Descrição das mudanças"`
2. Tag a versão: `git tag -a v1.2.1 -m "Release v1.2.1"`
3. Push para GitHub: `git push origin main && git push origin v1.2.1`
4. GitHub Actions automaticamente:
   - Compila para Linux e Windows
   - Cria pacotes (tar.gz, ZIP, AppImage)
   - Publica release com binários
5. Usuários recebem notificação na próxima inicialização

## Estatísticas

- **Tempo de verificação**: ~1-2 segundos (não bloqueia startup)
- **Tempo de download**: Depende da conexão (10-50MB)
- **Tempo de instalação**: ~5-10 segundos
- **Espaço necessário**: 2x o tamanho da aplicação (para backup)

---

**Dúvidas?** Consulte [PORTABILITY_GUIDE.md](PORTABILITY_GUIDE.md) ou abra uma issue no GitHub.
