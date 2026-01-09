#!/usr/bin/env bash

set -ex

workspace_packages=(
    packages/versions
    packages/renderer
    packages/cli
)

# Pre-publish checks
pnpm type-check
pnpm build

# Publish each package
for package_path in "${workspace_packages[@]}"; do
    cd "$package_path"
    pnpm publish --access public --no-git-checks
    cd - > /dev/null
done
