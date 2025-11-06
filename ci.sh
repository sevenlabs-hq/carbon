#!/bin/bash
# ci-local.sh

set -e  # Exit on error

echo "🔍 Checking Node.js version..."
node --version

echo "🔍 Checking pnpm version..."
pnpm --version

echo "📦 Installing dependencies..."
pnpm install --frozen-lockfile

echo "🔨 Building packages..."
pnpm build

echo "✨ Checking formatting..."
pnpm format:check

echo "🔍 Type checking..."
pnpm type-check

echo "✅ All checks passed!"