#!/usr/bin/env bash

set -e

packages=(
    packages/versions
    packages/renderer
    packages/cli
)

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.13.0"
    exit 1
fi

new_version=$1

echo "Updating all packages to $new_version"

# Update each package
for package_path in "${packages[@]}"; do
    node -e "
        const fs = require('fs');
        const path = '$package_path/package.json';
        const pkg = JSON.parse(fs.readFileSync(path, 'utf8'));
        pkg.version = '$new_version';
        fs.writeFileSync(path, JSON.stringify(pkg, null, 4) + '\n');
    "
    echo "  ✓ $package_path"
done

echo ""
echo "Next: pnpm install && git add -A && git commit -m \"v$new_version\""
