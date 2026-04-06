#!/bin/bash

echo "🖥️  ScreenLink - Development Server"
echo "==================================="
echo ""
echo "Make sure to run this from the project root!"
echo ""

# Build backend first
echo "🔨 Building backend..."
cd backend
npm run build > /dev/null 2>&1
cd ..

echo "📡 Starting Backend (Port 3001)..."
node backend/dist/server.js &
BACKEND_PID=$!

echo "Waiting for backend to start..."
sleep 2

echo "🎨 Starting Frontend (Port 5173)..."
cd frontend
npm run dev &
FRONTEND_PID=$!

echo ""
echo "✅ ScreenLink is Running!"
echo "=================================="
echo "🌐 Frontend: http://localhost:5173"
echo "📡 Backend:  http://localhost:3001"
echo ""
echo "Press CTRL+C to stop both servers"
echo ""

# Cleanup on exit
trap "kill $BACKEND_PID $FRONTEND_PID 2>/dev/null || true" EXIT

# Wait for both
wait
