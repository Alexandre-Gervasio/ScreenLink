#!/bin/bash
set -e

echo "🔨 Building ScreenLink..."
echo "=========================="

echo "📦 Building backend..."
cd backend
npm run build
echo "✅ Backend built"
cd ..

echo "📦 Building frontend..."
cd frontend
npm run build
echo "✅ Frontend built"
cd ..

echo ""
echo "✅ Build complete!"
echo "📁 Output:"
echo "  - Backend: ./backend/dist/"
echo "  - Frontend: ./dist/"
echo ""
echo "To run the built app:"
echo "  Backend:  node backend/dist/server.js"
echo "  Frontend: npm run preview:frontend"
