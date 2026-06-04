#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'USAGE'
Usage: scripts/bump-crate-version.sh <version>

Bumps Carbon Rust crate versions for a release.

Updates:
  - [workspace.package].version in Cargo.toml
  - Carbon path dependency versions in [workspace.dependencies]
  - package.version in all workspace member Cargo.toml files

Examples:
  scripts/bump-crate-version.sh 1.0.0
  scripts/bump-crate-version.sh 1.1.0
USAGE
}

if [[ $# -ne 1 ]]; then
    usage
    exit 1
fi

version="$1"
if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-+][0-9A-Za-z.-]+)?$ ]]; then
    echo "error: expected a semver-like version, got '$version'" >&2
    exit 1
fi
export CARBON_RELEASE_VERSION="$version"

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

metadata="$(cargo metadata --no-deps --format-version 1)"

member_manifests=()
while IFS= read -r manifest; do
    member_manifests+=("$manifest")
done < <(
    printf '%s' "$metadata" | jq -r --arg excluded "$repo_root/misc/jito-protos/Cargo.toml" '
        .packages[]
        | select(.source == null)
        | select(.manifest_path != $excluded)
        | .manifest_path
    '
)

if [[ ${#member_manifests[@]} -eq 0 ]]; then
    echo "error: no workspace member manifests found" >&2
    exit 1
fi

perl -0pi -e '
    my $version = $ENV{"CARBON_RELEASE_VERSION"};
    s/(\[workspace\.package\][^\[]*?version\s*=\s*")[^"]+(")/$1$version$2/s;
    s/((?!carbon-jito-protos)carbon-[A-Za-z0-9_-]+\s*=\s*\{[^}]*path\s*=\s*"[^"]+"[^}]*version\s*=\s*")[^"]+(")/$1$version$2/g;
' Cargo.toml

for manifest in "${member_manifests[@]}"; do
    perl -0pi -e '
        my $version = $ENV{"CARBON_RELEASE_VERSION"};
        s/(\[package\][^\[]*?version\s*=\s*")[^"]+(")/$1$version$2/s;
    ' "$manifest"
done

cargo metadata --no-deps --format-version 1 >/dev/null

echo "Updated Carbon Rust workspace crate versions to $version"
