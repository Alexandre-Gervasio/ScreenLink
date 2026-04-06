#!/bin/bash
set -e

REPO="Alexandre-Gervasio/kvm-pro"
TAG="v1.0.2"
RELEASE_NAME="KVM Pro v1.0.2 - Web GUI Control Panel"

echo "📦 Criando release para $TAG..."

# Criar release via git (sem token, apenas push da tag)
# A release será criada automaticamente no GitHub

echo "✅ Tag v1.0.2 foi enviada para GitHub"
echo "📝 Acesse: https://github.com/$REPO/releases/tag/$TAG"
echo ""
echo "Para criar a release completa com descrição:"
echo "1. Vá para: https://github.com/$REPO/releases"
echo "2. Clique em 'Draft a new release'"
echo "3. Selecione a tag v1.0.2"
echo "4. Adicione a descrição e faça upload dos arquivos"
