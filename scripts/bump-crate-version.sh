#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'USAGE'
Usage: scripts/bump-crate-version.sh <version>

Bumps Carbon Rust crate versions for a release.

Updates:
  - [workspace.package].version in Cargo.toml
  - Listed Carbon path dependency versions in [workspace.dependencies]
  - package.version in listed Rust crate Cargo.toml files

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

workspace_dependency_crates=(
    carbon-core
    carbon-macros
    carbon-proc-macros
    carbon-test-utils
    carbon-log-metrics
    carbon-prometheus-metrics
    carbon-helius-atlas-ws-datasource
    carbon-helius-gpa-v2-datasource
    carbon-helius-gtfa-datasource
    carbon-helius-laserstream-datasource
    carbon-jetstreamer-datasource
    carbon-jito-shredstream-grpc-datasource
    carbon-rpc-block-crawler-datasource
    carbon-rpc-block-subscribe-datasource
    carbon-rpc-gpa-datasource
    carbon-rpc-program-subscribe-datasource
    carbon-rpc-transaction-crawler-datasource
    carbon-stream-message-datasource
    carbon-validator-snapshot-datasource
    carbon-yellowstone-grpc-datasource
    carbon-address-lookup-table-decoder
    carbon-associated-token-account-decoder
    carbon-bonkswap-decoder
    carbon-boop-decoder
    carbon-bubblegum-decoder
    carbon-circle-message-transmitter-v2-decoder
    carbon-circle-token-messenger-v2-decoder
    carbon-dflow-aggregator-v4-decoder
    carbon-drift-v2-decoder
    carbon-fluxbeam-decoder
    carbon-gavel-decoder
    carbon-heaven-decoder
    carbon-jupiter-dca-decoder
    carbon-jupiter-lend-decoder
    carbon-jupiter-limit-order-2-decoder
    carbon-jupiter-limit-order-decoder
    carbon-jupiter-perpetuals-decoder
    carbon-jupiter-swap-decoder
    carbon-kamino-farms-decoder
    carbon-kamino-lending-decoder
    carbon-kamino-limit-order-decoder
    carbon-kamino-vault-decoder
    carbon-lifinity-amm-v2-decoder
    carbon-marginfi-v2-decoder
    carbon-marinade-finance-decoder
    carbon-memo-program-decoder
    carbon-meteora-damm-v2-decoder
    carbon-meteora-dbc-decoder
    carbon-meteora-dlmm-decoder
    carbon-meteora-pools-decoder
    carbon-meteora-vault-decoder
    carbon-moonshot-decoder
    carbon-mpl-core-decoder
    carbon-mpl-token-metadata-decoder
    carbon-name-service-decoder
    carbon-okx-dex-decoder
    carbon-onchain-labs-dex-v2-decoder
    carbon-openbook-v2-decoder
    carbon-orca-whirlpool-decoder
    carbon-pancake-swap-decoder
    carbon-phoenix-v1-decoder
    carbon-pump-fees-decoder
    carbon-pump-swap-decoder
    carbon-pumpfun-decoder
    carbon-raydium-amm-v4-decoder
    carbon-raydium-clmm-decoder
    carbon-raydium-cpmm-decoder
    carbon-raydium-launchpad-decoder
    carbon-raydium-liquidity-locking-decoder
    carbon-raydium-stable-swap-decoder
    carbon-sharky-decoder
    carbon-solayer-restaking-program-decoder
    carbon-stabble-stable-swap-decoder
    carbon-stabble-weighted-swap-decoder
    carbon-stake-program-decoder
    carbon-swig-decoder
    carbon-system-program-decoder
    carbon-token-2022-decoder
    carbon-token-program-decoder
    carbon-vertigo-decoder
    carbon-virtuals-decoder
    carbon-wavebreak-decoder
    carbon-zeta-decoder
)

crate_manifests=(
    crates/core/Cargo.toml
    crates/macros/Cargo.toml
    crates/proc-macros/Cargo.toml
    crates/test-utils/Cargo.toml
    datasources/helius-atlas-ws-datasource/Cargo.toml
    datasources/helius-gpa-v2-datasource/Cargo.toml
    datasources/helius-gtfa-datasource/Cargo.toml
    datasources/helius-laserstream-datasource/Cargo.toml
    datasources/jetstreamer-datasource/Cargo.toml
    datasources/jito-shredstream-grpc-datasource/Cargo.toml
    datasources/rpc-block-crawler-datasource/Cargo.toml
    datasources/rpc-block-subscribe-datasource/Cargo.toml
    datasources/rpc-gpa-datasource/Cargo.toml
    datasources/rpc-program-subscribe-datasource/Cargo.toml
    datasources/rpc-transaction-crawler-datasource/Cargo.toml
    datasources/stream-message-datasource/Cargo.toml
    datasources/validator-snapshot-datasource/Cargo.toml
    datasources/yellowstone-grpc-datasource/Cargo.toml
    decoders/address-lookup-table-decoder/Cargo.toml
    decoders/associated-token-account-decoder/Cargo.toml
    decoders/bonkswap-decoder/Cargo.toml
    decoders/boop-decoder/Cargo.toml
    decoders/bubblegum-decoder/Cargo.toml
    decoders/circle-message-transmitter-v2-decoder/Cargo.toml
    decoders/circle-token-messenger-v2-decoder/Cargo.toml
    decoders/dflow-aggregator-v4-decoder/Cargo.toml
    decoders/drift-v2-decoder/Cargo.toml
    decoders/fluxbeam-decoder/Cargo.toml
    decoders/gavel-decoder/Cargo.toml
    decoders/heaven-decoder/Cargo.toml
    decoders/jupiter-dca-decoder/Cargo.toml
    decoders/jupiter-lend-decoder/Cargo.toml
    decoders/jupiter-limit-order-2-decoder/Cargo.toml
    decoders/jupiter-limit-order-decoder/Cargo.toml
    decoders/jupiter-perpetuals-decoder/Cargo.toml
    decoders/jupiter-swap-decoder/Cargo.toml
    decoders/kamino-farms-decoder/Cargo.toml
    decoders/kamino-lending-decoder/Cargo.toml
    decoders/kamino-limit-order-decoder/Cargo.toml
    decoders/kamino-vault-decoder/Cargo.toml
    decoders/lifinity-amm-v2-decoder/Cargo.toml
    decoders/marginfi-v2-decoder/Cargo.toml
    decoders/marinade-finance-decoder/Cargo.toml
    decoders/memo-program-decoder/Cargo.toml
    decoders/meteora-damm-v2-decoder/Cargo.toml
    decoders/meteora-dbc-decoder/Cargo.toml
    decoders/meteora-dlmm-decoder/Cargo.toml
    decoders/meteora-pools-decoder/Cargo.toml
    decoders/meteora-vault-decoder/Cargo.toml
    decoders/moonshot-decoder/Cargo.toml
    decoders/mpl-core-decoder/Cargo.toml
    decoders/mpl-token-metadata-decoder/Cargo.toml
    decoders/name-service-decoder/Cargo.toml
    decoders/okx-dex-decoder/Cargo.toml
    decoders/onchain-labs-dex-v2-decoder/Cargo.toml
    decoders/openbook-v2-decoder/Cargo.toml
    decoders/orca-whirlpool-decoder/Cargo.toml
    decoders/pancake-swap-decoder/Cargo.toml
    decoders/phoenix-v1-decoder/Cargo.toml
    decoders/pump-fees-decoder/Cargo.toml
    decoders/pump-swap-decoder/Cargo.toml
    decoders/pumpfun-decoder/Cargo.toml
    decoders/raydium-amm-v4-decoder/Cargo.toml
    decoders/raydium-clmm-decoder/Cargo.toml
    decoders/raydium-cpmm-decoder/Cargo.toml
    decoders/raydium-launchpad-decoder/Cargo.toml
    decoders/raydium-liquidity-locking-decoder/Cargo.toml
    decoders/raydium-stable-swap-decoder/Cargo.toml
    decoders/sharky-decoder/Cargo.toml
    decoders/solayer-restaking-program-decoder/Cargo.toml
    decoders/stabble-stable-swap-decoder/Cargo.toml
    decoders/stabble-weighted-swap-decoder/Cargo.toml
    decoders/stake-program-decoder/Cargo.toml
    decoders/swig-decoder/Cargo.toml
    decoders/system-program-decoder/Cargo.toml
    decoders/token-2022-decoder/Cargo.toml
    decoders/token-program-decoder/Cargo.toml
    decoders/vertigo-decoder/Cargo.toml
    decoders/virtuals-decoder/Cargo.toml
    decoders/wavebreak-decoder/Cargo.toml
    decoders/zeta-decoder/Cargo.toml
    metrics/log-metrics/Cargo.toml
    metrics/prometheus-metrics/Cargo.toml
    examples/block-subscribe-rpc/Cargo.toml
    examples/custom-datasource/Cargo.toml
    examples/gpa-rpc/Cargo.toml
    examples/jetstreamer/Cargo.toml
    examples/postgres-graphql/Cargo.toml
    examples/snapshot-validator/Cargo.toml
    examples/transaction-crawler-rpc/Cargo.toml
    examples/versioned-decoders/Cargo.toml
    examples/versioned-decoders/decoder-v1/Cargo.toml
    examples/versioned-decoders/decoder-v2/Cargo.toml
    examples/yellowstone-grpc/Cargo.toml
)

for manifest in "${crate_manifests[@]}"; do
    if [[ ! -f "$manifest" ]]; then
        echo "error: listed crate manifest does not exist: $manifest" >&2
        exit 1
    fi
done

perl -0pi -e '
    my $version = $ENV{"CARBON_RELEASE_VERSION"};
    s/(\[workspace\.package\][^\[]*?^version\s*=\s*")[^"]+(")/$1$version$2/ms;
' Cargo.toml

for crate in "${workspace_dependency_crates[@]}"; do
    CARBON_RELEASE_CRATE="$crate" perl -0pi -e '
        my $version = $ENV{"CARBON_RELEASE_VERSION"};
        my $crate = quotemeta($ENV{"CARBON_RELEASE_CRATE"});
        s/(^$crate\s*=\s*\{[^\n}]*\bpath\s*=\s*"[^"]+"[^\n}]*\bversion\s*=\s*")[^"]+(")/$1$version$2/gm;
    ' Cargo.toml
done

for manifest in "${crate_manifests[@]}"; do
    perl -0pi -e '
        my $version = $ENV{"CARBON_RELEASE_VERSION"};
        s/(\[package\][^\[]*?^version\s*=\s*")[^"]+(")/$1$version$2/ms;
    ' "$manifest"
done

cargo metadata --no-deps --format-version 1 >/dev/null

echo "Updated Carbon Rust workspace crate versions to $version"
