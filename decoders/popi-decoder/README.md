# Carbon Popi Decoder

Decoder for [popi.wtf](https://popi.wtf), a two-chain memecoin launchpad. Generated from the Anchor IDL published on mainnet 2026-07-27 for program [`BNtfydNwzthyGfH1LMxt5AzQkJo7iyfGFbjjNKuHFH6M`](https://solscan.io/account/BNtfydNwzthyGfH1LMxt5AzQkJo7iyfGFbjjNKuHFH6M).

## Usage

```rust
use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
use carbon_popi_decoder::{PopiDecoder, PopiInstruction};

let decoder = PopiDecoder;
if let Some(PopiInstruction::BuySlot { data, accounts, .. }) = decoder.decode_instruction(&ix) {
    println!("buy_slot {} lamports by {}", data.sol_amount, accounts.user);
}
```
