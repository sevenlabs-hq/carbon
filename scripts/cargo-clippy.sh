#!/usr/bin/env bash

set -o errexit

cargo clippy \
    --workspace --all-targets -- \
    --deny=warnings \
    --deny=clippy::default_trait_access \
    --deny=clippy::arithmetic_side_effects \
    --deny=clippy::manual_let_else \
    --deny=clippy::used_underscore_binding
