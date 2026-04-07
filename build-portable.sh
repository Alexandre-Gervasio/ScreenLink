#!/bin/bash

echo "📦 ScreenLink Portable Build Script"
echo "===================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Detect platform
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  PLATFORM="linux"
  TARGET_EXT=".AppImage"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  PLATFORM="macos"
  TARGET_EXT=".dmg"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
  PLATFORM="windows"
  TARGET_EXT=".exe"
else
  PLATFORM="unknown"
fi

echo -e "${BLUE}Detected Platform: ${PLATFORM}${NC}"
echo ""

# Check prerequisites
echo "🔍 Checking prerequisites..."

if ! command -v node &> /dev/null; then
  echo "❌ Node.js not found. Please install Node.js 18+"
  exit 1
fi

if ! command -v npm &> /dev/null; then
  echo "❌ npm not found. Please install npm"
  exit 1
fi

if ! command -v cargo &> /dev/null; then
  echo "⚠️  Rust not found. Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
fi

echo -e "${GREEN}✓ Prerequisites OK${NC}"
echo ""

# Install dependencies
echo "📥 Installing dependencies..."
npm ci

# Build backend
echo -e "\n${BLUE}Building Backend...${NC}"
npm run build:backend
if [ $? -ne 0 ]; then
  echo "❌ Backend build failed"
  exit 1
fi
echo -e "${GREEN}✓ Backend built${NC}"

# Build frontend
echo -e "\n${BLUE}Building Frontend...${NC}"
npm run build:frontend
if [ $? -ne 0 ]; then
  echo "❌ Frontend build failed"
  exit 1
fi
echo -e "${GREEN}✓ Frontend built${NC}"

# Build Tauri app
echo -e "\n${BLUE}Building Tauri App for ${PLATFORM}...${NC}"

if [ "$PLATFORM" == "linux" ]; then
  echo "📦 Checking Linux dependencies..."
  REQUIRED_LIBS=("libgtk-3-dev" "libwebkit2gtk-4.1-dev" "libayatana-appindicator3-dev" "librsvg2-dev" "patchelf")
  for lib in "${REQUIRED_LIBS[@]}"; do
    if ! dpkg -l | grep -q "$lib"; then
      echo "⚠️  Missing: $lib"
    fi
  done
  echo -e "${GREEN}✓ Linux dependencies OK${NC}"
fi

npm run build:tauri
if [ $? -ne 0 ]; then
  echo "❌ Tauri build failed"
  exit 1
fi

echo -e "${GREEN}✓ Tauri app built${NC}"
echo ""

# Find output files
echo -e "${BLUE}📦 Build Output:${NC}"
if [ "$PLATFORM" == "linux" ]; then
  OUTPUT=$(find frontend/target/release/bundle -name "*.AppImage" -type f | head -1)
elif [ "$PLATFORM" == "macos" ]; then
  OUTPUT=$(find frontend/target/release/bundle -name "*.dmg" -type f | head -1)
else
  OUTPUT=$(find frontend/target/release/bundle -name "*.exe" -type f | head -1)
fi

if [ -n "$OUTPUT" ]; then
  SIZE=$(du -h "$OUTPUT" | cut -f1)
  echo -e "${GREEN}✓ Executable: $OUTPUT${NC}"
  echo -e "  Size: $SIZE"
  echo ""
  echo -e "${GREEN}🎉 Build completed successfully!${NC}"
  echo ""
  echo "To run the app:"
  if [ "$PLATFORM" == "windows" ]; then
    echo "  $OUTPUT"
  else
    echo "  $OUTPUT"
  fi
else
  echo -e "${YELLOW}⚠️  Could not locate output executable${NC}"
  echo "Check frontend/target/release/bundle/ for build artifacts"
fi

echo ""
