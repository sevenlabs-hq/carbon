# Carbon Jupiter Lend Decoder

Decoder for the Jupiter Lend Liquidity program on Solana.

## Program Information

- **Program ID**: `jupeiUmn818Jg1ekPURTpr4mFo29p46vygyykFJ3wZC`
- **Program Name**: Liquidity

## Features

This decoder provides support for:

- **Accounts**: Liquidity, TokenReserve, RateModel, UserBorrowPosition, UserSupplyPosition, UserClaim, AuthorizationList
- **Instructions**: All Jupiter Lend Liquidity program instructions including operate, claim, init_liquidity, update_rate_data, and more
- **Events**: LogOperate, LogClaim, LogUpdateExchangePrices, and other program events

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
carbon-jupiter-lend-decoder = { version = "0.12.0" }
```

### Basic Example

```rust
use carbon_jupiter_lend_decoder::{LiquidityDecoder, PROGRAM_ID};
use carbon_core::account::AccountDecoder;
use carbon_core::instruction::InstructionDecoder;

// Create a decoder instance
let decoder = LiquidityDecoder;

// Decode accounts
if let Some(decoded) = decoder.decode_account(&account) {
    // Handle decoded account
}

// Decode instructions
if let Some(decoded) = decoder.decode_instruction(&instruction) {
    // Handle decoded instruction
}
```

## Features Flags

- `serde` - Enable serde serialization/deserialization
- `postgres` - Enable PostgreSQL support
- `graphql` - Enable GraphQL support

## License

MIT
