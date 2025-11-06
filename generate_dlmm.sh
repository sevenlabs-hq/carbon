#!/bin/bash

# Generate all decoders script
# This script generates multiple decoders and copies .env file to each

set -e  # Exit on any error

# Set PROTOC environment variable if not already set
if [ -z "$PROTOC" ]; then
    if command -v protoc &> /dev/null; then
        export PROTOC=$(which protoc)
        echo "🔧 Setting PROTOC to: $PROTOC"
    else
        echo "❌ Error: protoc not found. Please install with: brew install protobuf"
        exit 1
    fi
fi

echo "🚀 Starting decoder generation for dlmm..."

# Change to the carbon directory
cd /Users/naidu/Documents/sevenlabs/carbon

# build the cli
cd packages/renderer && npm run build && cd ../cli && npm run build && cd ../..

node packages/cli/dist/cli.js scaffold --name dlmm --out-dir test --idl LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo --idl-url mainnet-beta --data-source helius-laserstream --with-postgres true --with-graphql true --force

cp .env test/dlmm/