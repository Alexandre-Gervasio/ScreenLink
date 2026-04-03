# Changelog

Todas as mudanças notáveis neste projeto estão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com),
e este projeto segue [Semantic Versioning](https://semver.org).

## [Unreleased]

### Added
- Auto-update system para receber updates automaticamente do GitHub
- GitHub Actions CI/CD para compilações automáticas
- Suporte a Windows com MinGW static linking
- Linux AppImage para distribuição universal

### Changed
- Estrutura de projeto reorganizada para melhor modularidade
- Scripts de build agora geram binários estáticos (sem dependências)

### Fixed
- Problema com desconexão de clientes em conexões TCP instáveis
- Memory leak em event injection

### Removed
- Dependência em systemd (agora é truly portable)

## [0.9.0] - 2024-01-15

### Added
- Version 0.9.0 - Beta release

### Known Issues
- Windows input injection ainda não implementada
- Suporte a multi-monitor é framework-only
- Clipboard sync não implementada

## [0.1.0] - 2024-01-01

### Added
- Projeto KVM Pro iniciado
- Architecture básica com TCP/UDP
- Input capture para Linux
- Configuration system com TOML
- Plugin system framework

---

## Como Contribuir com Changelog

Sempre que fizer mudanças, atualize este arquivo:

1. **Added** - Novos features
2. **Changed** - Mudanças em funcionalidade existente
3. **Deprecated** - Funcionalidades que serão removidas em breve
4. **Removed** - Funcionalidades removidas
5. **Fixed** - Bug fixes
6. **Security** - Correções de segurança

Exemplo:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- Feature description

### Changed
- Change description

### Fixed
- Bug fix description
```

## Release Checklist

Antes de fazer release:

1. ✅ Atualizar `## [Unreleased]` para `## [X.Y.Z] - YYYY-MM-DD`
2. ✅ Mover todas as entradas para a seção versionada
3. ✅ Adicionar novo `## [Unreleased]` vazio
4. ✅ Commit: `git commit -am "docs: Changelog for vX.Y.Z"`
5. ✅ Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
6. ✅ Push: `git push origin main --tags`

---

**Link**: [Compare releases](https://github.com/seu-usuario/kvm-pro/releases)
