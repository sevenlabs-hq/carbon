/**
 * Carbon Version Registry
 * 
 * Centralized registry for all Rust crate versions used in Carbon code generation.
 * This package version matches the Rust workspace version.
 */

export const VERSIONS = {
    "carbon-core": "0.11.0",
    "carbon-test-utils": "0.11.0",
    "carbon-log-metrics": "0.11.0",
    "carbon-prometheus-metrics": "0.11.0",
    "carbon-helius-atlas-ws-datasource": "0.11.0",
    "carbon-helius-laserstream-datasource": "0.11.0",
    "carbon-jito-shredstream-grpc-datasource": "0.11.0",
    "carbon-rpc-block-crawler-datasource": "0.11.0",
    "carbon-rpc-block-subscribe-datasource": "0.11.0",
    "carbon-rpc-program-subscribe-datasource": "0.11.0",
    "carbon-rpc-transaction-crawler-datasource": "0.11.0",
    "carbon-stream-message-datasource": "0.11.0",
    "carbon-yellowstone-grpc-datasource": "0.11.0",
    "solana-pubkey": "^3.0.0",
    "solana-client": "^3.0.3",
    "solana-instruction": "~3.0.0",
    "solana-account": "3.0.0",
    "solana-commitment-config": "~3.0.0",
    "solana-account-decoder": "^3.0.5",
    "yellowstone-grpc-client": "9.0.0",
    "yellowstone-grpc-proto": "9.0.0",
    "borsh": "1.5.1",
    "sqlx": "0.8.5",
    "sqlx_migrator": "0.17.0",
    "juniper": "0.16.1",
    "axum": "0.8.4",
    "rustls": "0.23",
    "helius": "0.3.2",
    "futures-util": "0.3.1",
} as const;

