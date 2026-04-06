# Contributing to ScreenLink

Thank you for your interest in contributing! This guide will help you get started.

## 🚀 Quick Start

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/YOUR_USERNAME/screenlink.git`
3. **Create a branch**: `git checkout -b feature/your-feature`
4. **Make changes** and test thoroughly
5. **Commit** with clear messages: `git commit -am 'feat: add amazing feature'`
6. **Push** to your fork: `git push origin feature/your-feature`
7. **Open a Pull Request** with description

## 📋 Development Setup

```bash
# Install dependencies
npm install

# Start development
npm run dev

# Run tests
npm test

# Build for production
npm run build
```

## 🎯 Code Standards

### Commits
Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` new feature
- `fix:` bug fix
- `docs:` documentation
- `style:` formatting
- `refactor:` code restructure
- `test:` test additions
- `chore:` maintenance

Example: `feat: add video quality selector`

### Code Style
- Use **TypeScript** for new code
- Run `npm run lint` before committing
- Format with Prettier: `npm run format`
- Keep functions small and focused
- Add JSDoc comments for public APIs

### Testing
- Write tests for new features
- Maintain >80% coverage
- Run `npm test` locally before pushing
- Tests must pass in CI

## 🐛 Reporting Issues

**Before reporting**, please:
1. Search existing issues - your issue may already be reported
2. Test with latest version
3. Gather system info: `node --version`, `npm --version`, OS

**Include in bug report**:
- Clear title and description
- Steps to reproduce
- Expected vs actual behavior
- System information
- Screenshots/logs if applicable

## 💡 Suggesting Features

- **Check discussions** for similar ideas
- Explain the use case clearly
- Keep proposals focused and achievable
- Be open to alternatives

## 📚 Project Structure

```
ScreenLink/
├── frontend/       # Tauri + React desktop app
├── backend/        # Node.js WebSocket server
├── shared/         # Shared types and constants
├── docs/           # Documentation
└── .github/        # GitHub workflows & config
```

## 🔄 PR Review Process

1. **Automated Checks**: CI/CD pipeline must pass
2. **Code Review**: Maintainers review code quality
3. **Feedback**: Address comments or ask questions
4. **Approval**: Get approval from maintainers
5. **Merge**: PR gets merged to develop branch

## 📦 Release Process

Maintainers handle releases:
1. Update version in `package.json`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag vX.Y.Z`
4. Push tag: `git push origin vX.Y.Z`
5. GitHub Actions auto-creates release

## ❓ Need Help?

- **Discussions**: [GitHub Discussions](https://github.com/Alexandre-Gervasio/screenlink/discussions)
- **Issues**: [Bug Reports](https://github.com/Alexandre-Gervasio/screenlink/issues)
- **Email**: dev@screenlink.app

## 🙏 Thank You!

Your contributions make ScreenLink better for everyone. Thanks for being part of our community!

---

**Happy coding!** 🚀
