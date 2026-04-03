#!/bin/bash
# KVM Pro v1.0.0 - Final Deployment Script
# Prepares the project for first GitHub release

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERSION="1.0.0"
GITHUB_USERNAME="${1:-your-username}"
GITHUB_REPO="${2:-kvm-pro}"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║   KVM Pro v${VERSION} - Deployment Preparation Script     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check git is initialized
if [ ! -d "$PROJECT_ROOT/.git" ]; then
    echo -e "${YELLOW}⚠ Git repository not initialized${NC}"
    echo "Initializing git..."
    cd "$PROJECT_ROOT"
    git init
    git config user.name "Developer"
    git config user.email "dev@example.com"
fi

cd "$PROJECT_ROOT"

echo -e "${BLUE}1. Validating project structure...${NC}"
required_files=(
    "README.md"
    "LICENSE"
    "core/Cargo.toml"
    ".github/workflows/release.yml"
    ".github/workflows/ci.yml"
    "scripts/release-build.sh"
    "USER_GUIDE.md"
    "RELEASE_NOTES.md"
)

for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        echo "  ✓ $file"
    else
        echo -e "  ${YELLOW}⚠ Missing: $file${NC}"
    fi
done

echo ""
echo -e "${BLUE}2. Checking core files exist...${NC}"
if [ -d "core/src" ] && [ -f "core/src/main.rs" ]; then
    echo "  ✓ Core application structure OK"
else
    echo -e "  ${YELLOW}⚠ Core structure needs attention${NC}"
fi

echo ""
echo -e "${BLUE}3. Verifying version consistency...${NC}"
CARGO_VERSION=$(grep '^version' core/Cargo.toml | head -1 | sed 's/version = "\([^"]*\)".*/\1/')
echo "  Version in Cargo.toml: $CARGO_VERSION"
if [ "$CARGO_VERSION" = "$VERSION" ]; then
    echo "  ✓ Version matches $VERSION"
else
    echo -e "  ${YELLOW}⚠ Version mismatch${NC}"
fi

echo ""
echo -e "${BLUE}4. Checking documentation...${NC}"
docs=(
    "README.md"
    "USER_GUIDE.md"
    "RELEASE_NOTES.md"
    "AUTO_UPDATE_USER_GUIDE.md"
    "DEPLOYMENT_GUIDE.md"
    "CONTRIBUTING.md"
    "LICENSE"
)

for doc in "${docs[@]}"; do
    if [ -f "$doc" ]; then
        lines=$(wc -l < "$doc")
        echo "  ✓ $doc ($lines lines)"
    else
        echo -e "  ${YELLOW}⚠ Missing documentation: $doc${NC}"
    fi
done

echo ""
echo -e "${BLUE}5. Project Statistics${NC}"
echo "  Code files:"
find core/src -name "*.rs" -type f | wc -l | xargs echo "    Rust files:"
find scripts -name "*.sh" -type f | wc -l | xargs echo "    Scripts:"
find . -name "*.md" -type f | wc -l | xargs echo "    Documentation:"

echo ""
echo -e "${BLUE}6. Git Status${NC}"
if [ -n "$(git status --short)" ]; then
    echo "  Uncommitted changes:"
    git status --short | sed 's/^/    /'
else
    echo "  ✓ Working directory clean"
fi

echo ""
echo -e "${YELLOW}════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Ready for Release!${NC}"
echo ""
echo "To publish v${VERSION} on GitHub:"
echo ""
echo "  1. Create GitHub repository:"
echo "     gh repo create $GITHUB_REPO --public"
echo ""
echo "  2. Add and commit:"
echo "     git add ."
echo "     git commit -m \"Initial commit: KVM Pro v${VERSION}\""
echo "     git branch -M main"
echo "     git push -u origin main"
echo ""
echo "  3. Create first release:"
echo "     git tag -a v${VERSION} -m \"Release v${VERSION} - Initial public release\""
echo "     git push origin v${VERSION}"
echo ""
echo "  GitHub Actions will then:"
echo "    • Compile for Linux (musl) and Windows (MinGW)"
echo "    • Create packages (tar.gz, ZIP, AppImage)"
echo "    • Publish release with binaries"
echo "    • ~15 minutes total"
echo ""
echo -e "${YELLOW}════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Next steps:"
echo "  1. Open: https://github.com/$GITHUB_USERNAME/$GITHUB_REPO"
echo "  2. Configure: Branch protection, Actions settings"
echo "  3. Announce: Tell users about the release"
echo "  4. Monitor: Check GitHub Actions logs"
echo ""
echo "Full documentation:"
echo "  • User Guide: $(pwd)/USER_GUIDE.md"
echo "  • Deployment: $(pwd)/DEPLOYMENT_GUIDE.md"
echo "  • Release Notes: $(pwd)/RELEASE_NOTES.md"
echo ""
echo -e "${GREEN}✓ Deployment preparation complete!${NC}"
