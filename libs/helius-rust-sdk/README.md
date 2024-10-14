# Helius SDK
An asynchronous Helius Rust SDK for building the future of Solana

## Documentation
The latest documentation can be found [here on docs.rs](https://docs.rs/helius/latest/helius/)

## Contributions
Interested in contributing to the Helius Rust SDK? Read the following [contributions guide](https://github.com/helius-labs/helius-rust-sdk/blob/dev/CONTRIBUTIONS.md) before opening up a pull request!

## Installation
To start using the Helius Rust SDK in your project, add it as a dependency via `cargo`. Open your project's `Cargo.toml` and add the following line under `[dependencies]`:
```toml
helius = "x.y.z"
```
where `x.y.z` is your desired version. Alternatively, use `cargo add helius` to add the dependency directly via the command line. This will automatically find the latest version compatible with your project and add it to your `Cargo.toml`.

Remember to run `cargo update` regularly to fetch the latest version of the SDK.

### TLS Options
The Helius Rust SDK uses the native TLS implementation by default via:
```toml
[dependencies]
helius = "x.y.z"
```

However, the SDK also supports `rustls`. Add the following to your `Cargo.toml` to use `rustls` instead of the native TLS implementation:
```toml
[dependencies]
helius = { version = "x.y.z", default-features = false, features = ["rustls"] }
```

Using `rustls` may be preferred in environments where OpenSSL is not available or when a pure Rust TLS implementation is desired. However, it may not support all the same features as the native TLS implementation

## Usage
### `Helius`
The SDK provides a [`Helius`](https://github.com/helius-labs/helius-rust-sdk/blob/dev/src/client.rs) instance that can be configured with an API key and a given Solana cluster. Developers can generate a new API key on the [Helius Developer Dashboard](https://dev.helius.xyz/dashboard/app). This instance acts as the main entry point for interacting with the SDK by providing methods to access different Solana and RPC client functionalities. The following code is an example of how to use the SDK to fetch info on [Mad Lad #8420](https://explorer.solana.com/address/F9Lw3ki3hJ7PF9HQXsBzoY8GyE6sPoEZZdXJBsTTD2rk?network=mainnet):
```rust
use helius::error::Result;
use helius::types::{Cluster, DisplayOptions, GetAssetRequest, GetAssetResponseForAsset};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key: &str = "YOUR_API_KEY";
    let cluster: Cluster = Cluster::MainnetBeta;

    let helius: Helius = Helius::new(api_key, cluster).unwrap();

    let request: GetAssetRequest = GetAssetRequest {
        id: "F9Lw3ki3hJ7PF9HQXsBzoY8GyE6sPoEZZdXJBsTTD2rk".to_string(),
        display_options: None,
    };

    let response: Result<Option<GetAssetResponseForAsset>> = helius.rpc().get_asset(request).await;

    match response {
        Ok(Some(asset)) => {
            println!("Asset: {:?}", asset);
        },
        Ok(None) => println!("No asset found."),
        Err(e) => println!("Error retrieving asset: {:?}", e),
    }

    Ok(())
}
```
### `HeliusFactory`
The SDK also comes equipped with `HeliusFactory`, a factory for creating instances of `Helius`. This factory allows for a centralized configuration and creation of `Helius` clients so work can be done across multiple clusters at the same time. Using a factory simplifies client code and enhances maintainability by ensuring that all `Helius` clients are configured consistently. It has the following functionality:
- A [`new` method](https://github.com/helius-labs/helius-rust-sdk/blob/a79a751e1a064125010bdb359068a366d635d005/src/factory.rs#L21-L36) used to create a new `HeliusFactory` capable of producing `Helius` clients. Note this method does not create a `reqwest` client
- A [`with_client` method](https://github.com/helius-labs/helius-rust-sdk/blob/a79a751e1a064125010bdb359068a366d635d005/src/factory.rs#L38-L53) used to provide a given `HeliusFactory` created with the `new` method its own `reqwest` client
- A [`create` method](https://github.com/helius-labs/helius-rust-sdk/blob/a79a751e1a064125010bdb359068a366d635d005/src/factory.rs#L55-L94) used to create multiple `Helius` clients in a thread-safe manner

### Embedded Solana Client
The `Helius` client has an embedded [Solana client](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html) that can be accessed via `helius.connection().request_name()` where `request_name()` is a given [RPC method](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#implementations). A full list of all Solana RPC HTTP methods can be found [here](https://solana.com/docs/rpc/http).

Note that this Solana client is synchronous by default. An asynchronous client can be created using the `new_with_async_solana` method in place of the `new` method. The asynchronous client can be accessed via `helius.async_connection()?.some_async_method().await?` where `some_async_method()` is a given async RPC method.

### Enhanced WebSockets
The `Helius` client can also be created with the `new_with_ws()` method in place of the `new` method. This will create a WebSocket client, adding support for the [Geyser Enhanced WebSocket methods](https://docs.helius.dev/webhooks-and-websockets/websockets#helius-geyser-enhanced-websockets-beta) [`transactionSubscribe`](https://docs.helius.dev/webhooks-and-websockets/websockets#transaction-subscribe) and [`accountSubscribe`](https://docs.helius.dev/webhooks-and-websockets/websockets#account-subscribe)

### Examples
More examples of how to use the SDK can be found in the [`examples`](https://github.com/helius-labs/helius-rust-sdk/tree/dev/examples) directory.

## Error Handling
### Common Error Codes
You may encounter several error codes when working with the Helius Rust SDK. Below is a table detailing some of the common error codes along with additional information to aid with troubleshooting:

| Error Code | Error Message             | More Information                                                                           |
|------------|---------------------------|---------------------------------------------------------------------------------------------|
| 401        | Unauthorized              | This occurs when an invalid API key is provided or access is restricted |
| 429        | Too Many Requests         | This indicates that the user has exceeded the request limit in a given timeframe or is out of credits |
| 5XX        | Internal Server Error     | This is a generic error message for server-side issues. Please contact Helius support for assistance |

If you encounter any of these errors:
- Refer to [`errors.rs`](https://github.com/helius-labs/helius-rust-sdk/blob/dev/src/error.rs) for a list of all possible errors returned by the `Helius` client
- Refer to the [Helius documentation](https://docs.helius.dev/) for further guidance
- Reach out to the Helius support team for more detailed assistance

### Result Type Alias
The SDK also has [a handy type alias for `Result`](https://github.com/helius-labs/helius-rust-sdk/blob/c24bdf3179998895e73fe455d38bd7faa2c50df5/src/error.rs#L147-L148) where `Result<(some type), HeliusError>` and be simplified to `Result<(some type)>`

## Methods
Our SDK is designed to provide a seamless developer experience when building on Solana. We've separated the core functionality into various segments:

### DAS API
- [`get_asset`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-asset) - Gets an asset by its ID
- [`get_asset_batch`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-asset/get-asset-batch) - Gets multiple assets by their ID
- [`get_asset_proof`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-asset-proof) - Gets a merkle proof for a compressed asset by its ID
- [`get_asset_proof_batch`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-asset-proof/get-asset-proof-batch) - Gets multiple asset proofs by their IDs
- [`get_assets_by_owner`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-assets-by-owner) - Gets a list of assets owned by a given address
- [`get_assets_by_authority`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-assets-by-authority) - Gets a list of assets of a given authority
- [`get_assets_by_creator`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-assets-by-creator) - Gets a list of assets of a given creator
- [`get_assets_by_group`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-assets-by-group) - Gets a list of assets by a group key and value
- [`search_assets`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/search-assets) - Gets assets based on the custom search criteria passed in
- [`get_signatures_for_asset`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-signatures-for-asset) - Gets transaction signatures for a given asset
- [`get_token_accounts`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-token-accounts) - Gets information about all token accounts for a specific mint or owner
- [`get_nft_edition`](https://docs.helius.dev/compression-and-das-api/digital-asset-standard-das-api/get-nft-editions) - Gets all the NFT editions  associated with a specific master NFT
- [`get_rwa_asset`](https://github.com/helius-labs/helius-sdk/pull/71) - Gets a Real-World Asset (RWA) based on its mint address

### Mint API
- [`mint_compressed_nft`](https://docs.helius.dev/compression-and-das-api/mint-api/mint-compressed-nft) - The easiest way to mint a compressed NFT (cNFT)

### Enhanced Transactions API
- [`parse_transactions`](https://docs.helius.dev/solana-apis/enhanced-transactions-api/parse-transaction-s) - Parses transactions given an array of transaction IDs
- [`parsed_transaction_history`](https://docs.helius.dev/solana-apis/enhanced-transactions-api/parsed-transaction-history) - Retrieves a parsed transaction history for a specific address

### Webhooks
- [`append_addresses_to_webhook`](https://github.com/helius-labs/helius-rust-sdk/blob/2d161e1ebf6d06df686d9e248ea80de215457b40/src/webhook.rs#L50-L73) - Appends a set of addresses to a given webhook
- [`create_webhook`](https://docs.helius.dev/webhooks-and-websockets/api-reference/create-webhook) - Creates a webhook given account addresses
- [`delete_webhook`](https://docs.helius.dev/webhooks-and-websockets/api-reference/delete-webhook) - Deletes a given Helius webhook programmatically
- [`edit_webhook`](https://docs.helius.dev/webhooks-and-websockets/api-reference/edit-webhook) - Edits a Helius webhook programmatically
- [`get_all_webhooks`](https://docs.helius.dev/webhooks-and-websockets/api-reference/get-all-webhooks) - Retrieves all Helius webhooks programmatically
- [`get_webhook_by_id`](https://docs.helius.dev/webhooks-and-websockets/api-reference/get-webhook) - Gets a webhook config given a webhook ID
- [`remove_addresses_from_webhook`](https://github.com/helius-labs/helius-rust-sdk/blob/bf24259e3333ae93126bb65b342c2c63e80e07a6/src/webhook.rs#L75-L105) - Removes a list of addresses from an existing webhook by its ID

### Smart Transactions
- [`create_smart_transaction`](https://github.com/helius-labs/helius-rust-sdk/blob/705d66fb7d4004fc32c2a5f0d6ca4a1f2a7b175d/src/optimized_transaction.rs#L113-L312) - Creates an optimized transaction based on the provided configuration
- [`get_compute_units`](https://github.com/helius-labs/helius-rust-sdk/blob/a79a751e1a064125010bdb359068a366d635d005/src/optimized_transaction.rs#L29-L75) - Simulates a transaction to get the total compute units consumed
- [`poll_transaction_confirmation`](https://github.com/helius-labs/helius-rust-sdk/blob/a79a751e1a064125010bdb359068a366d635d005/src/optimized_transaction.rs#L77-L112) - Polls a transaction to check whether it has been confirmed in 5 second intervals with a 15 second timeout
- [`send_smart_transaction`](https://github.com/helius-labs/helius-rust-sdk/blob/705d66fb7d4004fc32c2a5f0d6ca4a1f2a7b175d/src/optimized_transaction.rs#L314-L374) - Builds and sends an optimized transaction, and handles its confirmation status
- [`send_smart_transaction_with_seeds`]() - Sends a smart transaction using seed bytes

### Jito Smart Transactions and Helper Methods
- [`add_tip_instruction`](https://github.com/helius-labs/helius-rust-sdk/blob/02b351a5ee3fe16a36078b40f92dc72d0ad077ed/src/jito.rs#L66-L83) - Adds a tip instruction to the instructions provided
- [`create_smart_transaction_with_tip`](https://github.com/helius-labs/helius-rust-sdk/blob/02b351a5ee3fe16a36078b40f92dc72d0ad077ed/src/jito.rs#L85-L124) - Creates a smart transaction with a Jito tip
- [`get_bundle_statuses`](https://github.com/helius-labs/helius-rust-sdk/blob/02b351a5ee3fe16a36078b40f92dc72d0ad077ed/src/jito.rs#L169-L202) - Get the status of Jito bundles
- [`send_jito_bundle`](https://github.com/helius-labs/helius-rust-sdk/blob/02b351a5ee3fe16a36078b40f92dc72d0ad077ed/src/jito.rs#L126-L167) - Sends a bundle of transactions to the Jito Block Engine
- [`send_smart_transaction_with_tip`](https://github.com/helius-labs/helius-rust-sdk/blob/02b351a5ee3fe16a36078b40f92dc72d0ad077ed/src/jito.rs#L204-L269) - Sends a smart transaction as a Jito bundle with a tip

### Helper Methods
- [`get_priority_fee_estimate`](https://docs.helius.dev/solana-rpc-nodes/alpha-priority-fee-api) - Gets an estimate of the priority fees required for a transaction to be processed more quickly
- [`deserialize_str_to_number`](https://github.com/helius-labs/helius-rust-sdk/blob/dev/src/utils/deserialize_str_to_number.rs) - Deserializes a `String` to a `Number`
- [`is_valid_solana_address`](https://github.com/helius-labs/helius-rust-sdk/blob/dev/src/utils/is_valid_solana_address.rs) - Returns whether a given string slice is a valid Solana address
- [`make_keypairs`](https://github.com/helius-labs/helius-rust-sdk/blob/dev/src/utils/make_keypairs.rs) - Generates a specified number of keypairs