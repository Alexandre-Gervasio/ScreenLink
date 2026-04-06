# Testing Guide

This guide explains how to write and run tests in ScreenLink.

## 📋 Overview

ScreenLink uses:
- **Jest** - Test runner and assertion library
- **TypeScript** - Type-safe tests
- **ESLint** - Code quality
- **Prettier** - Code formatting
- **Husky** - Git hooks for quality checks

## 🚀 Quick Start

### Run Tests
```bash
# Run all tests with coverage
npm test

# Run tests in watch mode (rerun on file changes)
npm test:watch

# Run specific test directory
npm run test:backend

# Run frontend tests
npm run test:frontend
```

### Linting & Formatting
```bash
# Check for linting errors
npm run lint

# Fix linting issues automatically
npm run lint:fix

# Check formatting
npm run format:check

# Auto-format all code
npm run format

# Type check TypeScript
npm run type-check
```

## 📝 Writing Tests

### File Structure
```
src/
├── __tests__/
│   ├── server.test.ts      # Backend tests
│   └── components.test.tsx  # Frontend tests
├── components/
├── utils/
└── index.ts
```

### Test Example - Backend

```typescript
// backend/src/__tests__/example.test.ts
describe('WebSocket Server', () => {
  describe('Connection', () => {
    it('should connect to server', () => {
      const connected = true;
      expect(connected).toBe(true);
    });

    it('should handle messages', (done) => {
      const message = { type: 'TEST' };
      expect(message.type).toBe('TEST');
      done();
    });
  });
});
```

### Test Example - Frontend

```typescript
// frontend/src/__tests__/example.test.tsx
import React from 'react';
import { render, screen } from '@testing-library/react';

describe('ScreenView Component', () => {
  it('should render video element', () => {
    render(<video data-testid="screen-video" />);
    const video = screen.getByTestId('screen-video');
    expect(video).toBeInTheDocument();
  });
});
```

## 🎯 Best Practices

### Do's ✅
- ✅ Write tests for critical functionality
- ✅ Use descriptive test names: `it('should validate UUID format')`
- ✅ Keep tests focused and small
- ✅ Mock external dependencies
- ✅ Test edge cases and error conditions
- ✅ Aim for >80% code coverage
- ✅ Write tests before fixing bugs (TDD)

### Don'ts ❌
- ❌ Test implementation details, test behavior
- ❌ Create interdependent tests
- ❌ Use `setTimeout` without proper handling
- ❌ Leave skipped tests (`it.skip()`)
- ❌ Test external APIs directly
- ❌ Ignore test failures

## 🔍 Common Test Patterns

### Testing Async Functions
```typescript
it('should fetch data', async () => {
  const data = await fetchData();
  expect(data).toBeDefined();
});

// Or with done callback
it('should handle callback', (done) => {
  fetchData().then(() => {
    expect(true).toBe(true);
    done();
  });
});
```

### Testing Error Handling
```typescript
it('should throw error on invalid input', () => {
  expect(() => processData(null)).toThrow();
});

it('should handle rejected promise', async () => {
  await expect(failingFunction()).rejects.toThrow();
});
```

### Mocking Functions
```typescript
it('should call function with correct args', () => {
  const mockFn = jest.fn();
  mockFn('test');
  expect(mockFn).toHaveBeenCalledWith('test');
});
```

## 🚀 Pre-commit Hooks

Git hooks automatically run before commits:

### pre-commit Hook
```bash
# Runs these checks automatically:
$ npm run lint       # ESLint
$ npm run format     # Prettier
$ npm run type-check # TypeScript
```

If checks fail, the commit is blocked. Fix issues and try again:
```bash
npm run lint:fix
npm run format
# Then stage fixed files and commit again
```

### Bypassing Hooks (Not Recommended!)
```bash
git commit --no-verify  # Skip all hooks
```

## 📈 Coverage Reports

After running tests, coverage report is generated:

```bash
npm test
# Creates: coverage/lcov-report/index.html

# View coverage report in browser
open coverage/lcov-report/index.html  # macOS
xdg-open coverage/lcov-report/index.html  # Linux
start coverage/lcov-report/index.html   # Windows
```

### Coverage Thresholds
- **Lines**: 70%
- **Branches**: 70%
- **Functions**: 70%
- **Statements**: 70%

## 🔗 Related Documentation

- **[Jest Documentation](https://jestjs.io/)**
- **[ESLint Guide](https://eslint.org/docs/)**
- **[Prettier Docs](https://prettier.io/docs/)**
- **[Husky Guide](https://typicode.github.io/husky/)**

## ❓ Troubleshooting

### Tests not running
```bash
# Clear Jest cache
npm test -- --clearCache

# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

### ESLint not finding files
```bash
# Check .eslintrc.json configuration
npm run type-check  # First check types

# Run with verbose output
npm run lint -- --debug
```

### Pre-commit hooks not executing
```bash
# Reinstall husky
npm run prepare

# Make hooks executable
chmod +x .husky/*
```

---

**Happy Testing!** 🧪
