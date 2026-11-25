# Carbon Limitless Launchpad Decoder

Decodes **Limitless Launchpad** instructions, accounts, and Anchor events on Solana mainnet-beta into typed Rust for use in a [Carbon](https://github.com/sevenlabs-hq/carbon) indexing pipeline.

- **Program ID:** `LimitQGBbcHeGJZd3FbQEejiCRuk1rfvwhbv8XCtAoN`
- **Token Standard:** SPL Token-2022 (`TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`)
- **Graduation AMM:** Meteora DAMM v2 (`cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG`)

## Decoded Accounts

- `Config` — global protocol config (`["config"]`)
- `LaunchedCoin` — per-mint bonding curve state, virtual reserves, and graduation pointer (`["launched_coin", mint]`)
- `MeteoraGraduation` — immutable post-graduation Meteora DAMM v2 receipt (`["meteora_graduation", mint]`)

## Decoded Instructions

- `LaunchCoin`
- `CreateAndFirstBuy`
- `CreateOrBuyPda`
- `CreateOrBuyPdaWithVanityFee`
- `Buy`
- `Sell`
- `Graduate` (legacy Raydium CPMM)
- `PrepareMeteoraGraduation`
- `GraduateMeteora`
- `ClaimMeteoraFees`

## Decoded Events (`Program data:` logs)

- `CoinLaunched`
- `Bought`
- `Sold`
- `Graduated`
- `MeteoraGraduated`

## Price Formula (from `LaunchedCoin`)

```text
virtual_sol    = sol_listed + sol_reserves
virtual_tokens = virtual_token_offset + tokens_remaining_on_curve
price_sol      = (virtual_sol / 1e9) / (virtual_tokens / 10^decimals)
```
