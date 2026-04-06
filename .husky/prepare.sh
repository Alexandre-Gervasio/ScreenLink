#!/bin/bash

# Husky install script
# This script is automatically run after npm install

echo "🔧 Setting up Git hooks with Husky..."

# Install husky
npx husky install

# Add pre-commit hook
npx husky add .husky/pre-commit "npm run lint"
npx husky add .husky/commit-msg 'npx --no -- commitlint --edit "$1"'

echo "✅ Git hooks installed!"
echo ""
echo "Installed hooks:"
echo "  • pre-commit: Runs linting and formatting checks"
echo "  • commit-msg: Validates commit messages"
echo ""
echo "💡 To run hooks manually:"
echo "  npm run lint       # Run all linters"
echo "  npm run format     # Auto-format code"
echo ""
