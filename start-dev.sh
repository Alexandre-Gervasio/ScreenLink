#!/bin/bash

echo "🚀 Starting ScreenLink (Development Mode)"
echo "======================================"

# Backend
echo "📡 Starting Backend Server (Port 3001)..."
cd backend
npm run dev &
BACKEND_PID=$!
echo "Backend PID: $BACKEND_PID"

# Sleep a bit to let backend start
sleep 3

# Frontend
echo "🖥️  Starting Frontend (Port 5173)..."
cd ../frontend
npm run dev &
FRONTEND_PID=$!
echo "Frontend PID: $FRONTEND_PID"

echo ""
echo "✅ ScreenLink is running!"
echo "======================================"
echo "🌐 Frontend: http://localhost:5173"
echo "📡 Backend: http://localhost:3001"
echo ""
echo "Press CTRL+C to stop"
echo ""

# Wait for both processes
wait
