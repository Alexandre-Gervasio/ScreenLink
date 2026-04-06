// Jest setup file - runs before all tests
// Add global test configuration here

// Mock timers for consistent testing
jest.useFakeTimers();

// Suppress console output in tests unless needed
const originalError = console.error;
const originalWarn = console.warn;
beforeAll(() => {
  console.error = jest.fn((...args) => {
    if (
      typeof args[0] === 'string' &&
      (args[0].includes('Not implemented') || args[0].includes('Mock'))
    ) {
      return;
    }
    originalError.call(console, ...args);
  });

  console.warn = jest.fn((...args) => {
    if (typeof args[0] === 'string' && args[0].includes('Deprecation')) {
      return;
    }
    originalWarn.call(console, ...args);
  });
});

afterAll(() => {
  console.error = originalError;
  console.warn = originalWarn;
});
