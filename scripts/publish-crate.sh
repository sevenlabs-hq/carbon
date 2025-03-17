#!/usr/bin/env bash

set -ex

workspace_crates=(
    carbon-marginfi-v2-decoder
    carbon-marinade-finance-decoder
    carbon-memo-program-decoder
    carbon-meteora-dlmm-decoder
    carbon-moonshot-decoder
    carbon-mpl-core-decoder
    carbon-mpl-token-metadata-decoder
    carbon-name-service-decoder
    carbon-okx-dex-decoder
    carbon-openbook-v2-decoder
    carbon-orca-whirlpool-decoder
    carbon-phoenix-v1-decoder
    carbon-pumpfun-decoder
    carbon-raydium-amm-v4-decoder
    carbon-raydium-clmm-decoder
    carbon-raydium-cpmm-decoder
    carbon-raydium-liquidity-locking-decoder
    carbon-sharky-decoder
    carbon-solayer-restaking-program-decoder
    carbon-spl-associated-token-account-decoder
    carbon-stabble-stable-swap-decoder
    carbon-stabble-weighted-swap-decoder
    carbon-stake-program-decoder
    carbon-system-program-decoder
    carbon-token-2022-decoder
    carbon-token-program-decoder
    carbon-zeta-decoder
)

for crate in "${workspace_crates[@]}"; do
   echo "--- $crate"
   cargo publish -p $crate
done