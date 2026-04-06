# 🚀 ScreenLink - Quick Start Guide

## Prerequisites

- **Node.js**: v18 or higher
- **npm**: v8 or higher

Check your versions:
```bash
node --version
npm --version
```

---

## Installation

### 1. Clone the repository
```bash
git clone https://github.com/Alexandre-Gervasio/ScreenLink.git
cd ScreenLink
```

### 2. Install dependencies
```bash
npm install --legacy-peer-deps
```

This installs dependencies for:
- Root (shared configs)
- Backend (Express + WebSocket)
- Frontend (React + Vite)

---

## Running the Project

### Option 1: Use the convenience script (Recommended)
```bash
./start-dev.sh
```

This will start both backend and frontend in development mode.

### Option 2: Run separately in different terminals

**Terminal 1 - Backend:**
```bash
npm run dev:backend
# or
cd backend && npm run dev
```

**Terminal 2 - Frontend:**
```bash
npm run dev:frontend
# or  
cd frontend && npm run dev
```

---

## Accessing the Application

Once running, open your browser:

🌐 **Frontend**: http://localhost:5173
```
This is the React UI where you can:
- Start as Primary PC (share your screen)
- Connect as Secondary PC (extend your display)
```

📡 **Backend API**: http://localhost:3001
```
Available endpoints:
- GET  /health          - Server health check
- POST /api/links/create - Generate share link
- GET  /api/links       - List active links
```

---

## What to Try First

1. **Open Frontend**: http://localhost:5173
2. **Click "I'm the Main PC"** to generate a share link
3. **Copy the code** (e.g., "ABCD-1234")
4. **Open Frontend in another browser** (or incognito tab)
5. **Click "I'm the Extended Screen"**
6. **Paste the code** to "connect"

---

## Troubleshooting

### Port Already in Use

If port 3001 or 5173 is already in use:

**Backend on different port:**
```bash
PORT=3333 npm run dev:backend
```

**Frontend on different port:**
```bash
npm run dev:frontend -- --port 5174
```

### Dependencies Issues

Clear cache and reinstall:
```bash
rm -rf node_modules package-lock.json
npm install --legacy-peer-deps
```

### Build the Project

```bash
# Build backend
npm run build:backend

# Build frontend
npm run build:frontend

# Build both
npm run build
```

### Run Tests

```bash
npm test                 # All tests
npm run test:backend     # Backend tests
npm run test:frontend    # Frontend tests
```

### Code Quality

```bash
npm run lint             # Check code
npm run lint:fix         # Fix issues
npm run format           # Format code
npm run type-check       # TypeScript check
```

---

## Project Structure

```
ScreenLink/
├── backend/             # Express + WebSocket server
│   └── src/server.ts   # Main backend file
├── frontend/           # React + Vite app
│   └── src/App.tsx    # Main React component
├── shared/            # Shared types/constants
└── docs/              # Documentation
```

---

## Development Tools

### Available Scripts

```bash
npm run dev              # Start both (backend + frontend)
npm run dev:backend      # Backend only
npm run dev:frontend     # Frontend only
npm run build            # Production build
npm run lint             # Lint code
npm run format           # Format code
npm test                 # Run tests
npm run type-check       # Type check
```

---

## Common Issues

### "Port 3001 is already in use"
Another server is running on that port. Either:
- Stop the other server
- Use a different port: `PORT=3333 npm run dev:backend`

### "Cannot find module 'uuid'"
Dependencies not installed. Run:
```bash
npm install --legacy-peer-deps
```

### "Frontend showing blank page"
- Check browser console for errors (F12)
- Verify backend is running on http://localhost:3001
- Check that API calls are successful

### TypeScript errors when running npm run dev
This is normal during development. The app will still run. Check for errors with:
```bash
npm run lint
npm run type-check
```

---

## Next Steps

- 📖 Read [README.md](./README.md) for full documentation
- 🔍 Check [docs/API.md](./docs/API.md) for API details
- ⚡ Run [benchmarks](./benchmarks/run-benchmark.js) for performance testing
- 📚 See [examples/](./examples/) for client implementations

---

## Getting Help

- 💬 Check [docs/](./docs/) for documentation
- 🐛 Open issues on GitHub
- 💡 See [CONTRIBUTING.md](./CONTRIBUTING.md) to contribute

---

**Happy Coding!** 🎉

