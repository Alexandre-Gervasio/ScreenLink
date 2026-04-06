#!/bin/bash

# Start two KVM Pro instances in different terminals
# This simulates two computers discovering each other

echo "🚀 Starting KVM Pro P2P Test (2 instances)"
echo "==========================================="
echo ""
echo "Starting MACHINE 1 (port 5000)..."
gnome-terminal -- bash -c "
  cd $(dirname \"$0\")/dist
  echo 'MACHINE 1 - Press Ctrl+C to stop'
  ./kvm-pro-linux
  bash
" &
TERM1_PID=$!

sleep 2

echo "Starting MACHINE 2 (port 5001)..."
gnome-terminal -- bash -c "
  cd $(dirname \"$0\")/dist
  echo 'MACHINE 2 - Press Ctrl+C to stop'
  ./kvm-pro-linux
  bash
" &
TERM2_PID=$!

echo ""
echo "✅ Two instances started!"
echo "   MACHINE 1 PID: $TERM1_PID"
echo "   MACHINE 2 PID: $TERM2_PID"
echo ""
echo "Watch both terminals:"
echo "  - Each should show the other as a discovered peer"
echo "  - Try connecting from one to the other"
echo ""
echo "Press Ctrl+C here to stop both instances"

wait $TERM1_PID $TERM2_PID
