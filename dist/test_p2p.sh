#!/bin/bash

# KVM Pro P2P Test - Opens 2 instances in separate terminals

DIST_DIR="$(dirname "$0")"

echo "🚀 Starting KVM Pro P2P Test..."
echo ""

# Terminal 1
echo "Opening Terminal 1..."
x-terminal-emulator -e bash -c "
  cd '$DIST_DIR'
  echo '=== INSTANCE 1 ==='
  echo ''
  ./kvm-pro-linux
  bash
" &

sleep 2

# Terminal 2
echo "Opening Terminal 2..."
x-terminal-emulator -e bash -c "
  cd '$DIST_DIR'
  echo '=== INSTANCE 2 ==='
  echo ''
  ./kvm-pro-linux
  bash
" &

echo ""
echo "✅ Opened 2 terminals with KVM Pro instances"
echo "   Watch both discover each other!"
echo ""

