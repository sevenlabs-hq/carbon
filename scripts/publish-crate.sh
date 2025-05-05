#!/usr/bin/env bash

set -ex

workspace_crates=(
    carbon-jito-protos
    
    carbon-macros
    carbon-proc-macros
    carbon-core
    carbon-test-utils

    carbon-helius-atlas-ws-datasource
    carbon-rpc-block-crawler-datasource
    carbon-rpc-block-subscribe-datasource
    carbon-rpc-program-subscribe-datasource
    carbon-rpc-transaction-crawler-datasource
    carbon-jito-shredstream-grpc-datasource
    carbon-yellowstone-grpc-datasource

    carbon-log-metrics
    carbon-prometheus-metrics

    carbon-associated-token-account-decoder
    carbon-boop-decoder
    carbon-drift-v2-decoder
    carbon-fluxbeam-decoder
    carbon-jupiter-dca-decoder
    carbon-jupiter-limit-order-2-decoder
    carbon-jupiter-limit-order-decoder
    carbon-jupiter-perpetuals-decoder
    carbon-jupiter-swap-decoder
    carbon-kamino-farms-decoder
    carbon-kamino-lending-decoder
    carbon-kamino-vault-decoder
    carbon-lifinity-amm-v2-decoder
    carbon-marginfi-v2-decoder
    carbon-marinade-finance-decoder
    carbon-memo-program-decoder
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
    carbon-pumpfun-decoder
    carbon-pump-swap-decoder
    carbon-raydium-amm-v4-decoder
    carbon-raydium-clmm-decoder
    carbon-raydium-cpmm-decoder
    carbon-raydium-launchpad-decoder
    carbon-raydium-liquidity-locking-decoder
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
   echo "--- $crate"
   cargo package -p $crate
   cargo publish -p $crate
done