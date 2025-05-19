#!/usr/bin/env bash

set -eu

# Linking the script as the pre-commit hook
SCRIPT_PATH=$(realpath "$0")
HOOK_PATH=$(git rev-parse --git-dir)/hooks/pre-commit
if [ "$(realpath "$HOOK_PATH")" != "$SCRIPT_PATH" ]; then
  read -p "Link this script as the git pre-commit hook to avoid further manual running? (y/N): " answer
  if [[ $answer =~ ^[Yy]$ ]]; then
    ln -sf "$SCRIPT_PATH" "$HOOK_PATH"
  fi
fi

set -x

# Install tools
cargo clippy --version &>/dev/null || rustup component add clippy
cargo machete --version &>/dev/null || cargo install --locked cargo-machete
cargo sort --version &>/dev/null || cargo install --locked cargo-sort

cargo fmt --version &>/dev/null || rustup component add rustfmt

# Checks
cargo machete
cargo fmt -- --check
cargo sort -c
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
