#!/usr/bin/env bash

set -ex

VERSION="0.9.0"

workspace_crates=(
    carbon-macros
    carbon-proc-macros
    carbon-test-utils
    carbon-core

    carbon-postgres-client
    carbon-gql-server

    carbon-helius-atlas-ws-datasource
    carbon-rpc-block-crawler-datasource
    carbon-rpc-block-subscribe-datasource
    carbon-rpc-program-subscribe-datasource
    carbon-rpc-transaction-crawler-datasource
    carbon-jito-shredstream-grpc-datasource
    carbon-yellowstone-grpc-datasource

    carbon-log-metrics
    carbon-prometheus-metrics

    carbon-address-lookup-table-decoder
    carbon-associated-token-account-decoder
    carbon-boop-decoder
    carbon-drift-v2-decoder
    carbon-fluxbeam-decoder
    carbon-gavel-decoder
    carbon-jupiter-dca-decoder
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
    carbon-meteora-dlmm-decoder
    carbon-meteora-pools-decoder
    carbon-moonshot-decoder
    carbon-mpl-core-decoder
    carbon-mpl-token-metadata-decoder
    carbon-name-service-decoder
    carbon-okx-dex-decoder
    carbon-openbook-v2-decoder
    carbon-orca-whirlpool-decoder
    carbon-phoenix-v1-decoder
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
    carbon-system-program-decoder
    carbon-token-2022-decoder
    carbon-token-program-decoder
    carbon-virtual-curve-decoder
    carbon-virtuals-decoder
    carbon-zeta-decoder
)

for crate in "${workspace_crates[@]}"; do
    echo "--- Yanking $crate version $VERSION"
    cargo yank --version $VERSION $crate
done

echo "All crates yanked for version $VERSION" 