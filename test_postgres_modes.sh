#!/bin/bash

# Test script for Postgres Table Mode options
# This script tests both generic and typed modes for parse and scaffold commands

set -e  # Exit on any error

echo "🚀 Testing Postgres Table Mode options with pump2 program..."

# Change to the carbon directory
cd /Users/naidu/Documents/sevenlabs/carbon

# Build the CLI
echo "🔧 Building CLI..."
cd packages/renderer && npm run build && cd ../cli && npm run build && cd ../..

# Clean up any existing test directories
echo "🧹 Cleaning up existing test directories..."
rm -rf test/pump2-*

# Test 1: Parse command with typed mode (default)
echo ""
echo "📦 Test 1: Parse command with typed mode (default)..."
node packages/cli/dist/cli.js parse --idl pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA --out-dir test/pump2-parse-typed --url mainnet-beta --postgres-mode typed

# Test 2: Parse command with generic mode
echo ""
echo "📦 Test 2: Parse command with generic mode..."
node packages/cli/dist/cli.js parse --idl pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA --out-dir test/pump2-parse-generic --url mainnet-beta --postgres-mode generic

# Test 3: Scaffold command with typed mode (default)
echo ""
echo "📦 Test 3: Scaffold command with typed mode (default)..."
node packages/cli/dist/cli.js scaffold --name pump2-typed --out-dir test --idl pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA --idl-url mainnet-beta --data-source helius-laserstream --with-postgres true --with-graphql true --postgres-mode typed --force

# Test 4: Scaffold command with generic mode
echo ""
echo "📦 Test 4: Scaffold command with generic mode..."
node packages/cli/dist/cli.js scaffold --name pump2-generic --out-dir test --idl pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA --idl-url mainnet-beta --data-source helius-laserstream --with-postgres true --with-graphql true --postgres-mode generic --force

echo ""
echo "🔍 Checking generated files..."

# Check if typed parse generated postgres files
echo "Checking typed parse postgres files..."
if [ -d "test/pump2-parse-typed/src/accounts/postgres" ] && [ -d "test/pump2-parse-typed/src/instructions/postgres" ]; then
    echo "✅ Typed parse: Postgres files generated correctly"
else
    echo "❌ Typed parse: Postgres files missing"
fi

# Check if generic parse skipped postgres files
echo "Checking generic parse postgres files..."
if [ ! -d "test/pump2-parse-generic/src/accounts/postgres" ] && [ ! -d "test/pump2-parse-generic/src/instructions/postgres" ]; then
    echo "✅ Generic parse: Postgres files correctly skipped"
else
    echo "❌ Generic parse: Postgres files should not exist"
fi

# Check scaffold generated files
echo "Checking scaffold files..."
if [ -d "test/pump2-typed" ] && [ -d "test/pump2-generic" ]; then
    echo "✅ Both scaffold projects generated"
    
    # Check if typed scaffold uses typed processors
    if grep -q "PostgresAccountProcessor" test/pump2-typed/indexer/src/main.rs; then
        echo "✅ Typed scaffold: Uses typed processors"
    else
        echo "❌ Typed scaffold: Missing typed processors"
    fi
    
    # Check if generic scaffold uses generic processors
    if grep -q "PostgresJsonAccountProcessor" test/pump2-generic/indexer/src/main.rs; then
        echo "✅ Generic scaffold: Uses generic processors"
    else
        echo "❌ Generic scaffold: Missing generic processors"
    fi
    
    # Check if generic scaffold uses generic migrations
    if grep -q "GenericAccountsMigration" test/pump2-generic/indexer/src/main.rs; then
        echo "✅ Generic scaffold: Uses generic migrations"
    else
        echo "❌ Generic scaffold: Missing generic migrations"
    fi
else
    echo "❌ Scaffold projects not generated correctly"
fi

echo ""
echo "🎉 Test completed!"
echo ""
echo "Generated test directories:"
echo "  - test/pump2-parse-typed/     (parse with typed mode)"
echo "  - test/pump2-parse-generic/   (parse with generic mode)"
echo "  - test/pump2-typed/           (scaffold with typed mode)"
echo "  - test/pump2-generic/         (scaffold with generic mode)"
echo ""
echo "You can inspect the generated files to verify the differences between modes."
