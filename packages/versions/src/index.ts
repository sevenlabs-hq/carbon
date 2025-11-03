/**
 * Carbon Version Registry
 * 
 * Centralized registry for all Rust crate versions used in Carbon code generation.
 * This package version matches the Rust workspace version.
 */

export type CrateDependency = 
    | string
    | {
        version?: string;
        git?: string;
        rev?: string;
        path?: string;
        features?: readonly string[];
        defaultFeatures?: boolean;
    };

export const VERSIONS: Record<string, CrateDependency> = {
    "carbon-core": {
        path: "../../../crates/core",
        version: "0.11.0",
        defaultFeatures: false,
    },
    "carbon-test-utils": {
        path: "../../../crates/test-utils",
        version: "0.11.0",
    },
    "carbon-log-metrics": {
        path: "../../../metrics/log-metrics",
        version: "0.11.0",
    },
    "carbon-prometheus-metrics": "0.11.0",
    "carbon-helius-atlas-ws-datasource": {
        path: "../../../datasources/helius-atlas-ws-datasource",
        version: "0.11.0",
    },
    "carbon-helius-laserstream-datasource": {
        path: "../../../datasources/helius-laserstream-datasource",
        version: "0.11.0",
    },
    "carbon-jito-shredstream-grpc-datasource": {
        path: "../../../datasources/jito-shredstream-grpc-datasource",
        version: "0.11.0",
    },
    "carbon-rpc-block-crawler-datasource": {
        path: "../../../datasources/rpc-block-crawler-datasource",
        version: "0.11.0",
    },
    "carbon-rpc-block-subscribe-datasource": {
        path: "../../../datasources/rpc-block-subscribe-datasource",
        version: "0.11.0",
    },
    "carbon-rpc-program-subscribe-datasource": {
        path: "../../../datasources/rpc-program-subscribe-datasource",
        version: "0.11.0",
    },
    "carbon-rpc-transaction-crawler-datasource": {
        path: "../../../datasources/rpc-transaction-crawler-datasource",
        version: "0.11.0",
    },
    "carbon-stream-message-datasource": {
        path: "../../../datasources/stream-message-datasource",
        version: "0.11.0",
    },
    "carbon-yellowstone-grpc-datasource": {
        path: "../../../datasources/yellowstone-grpc-datasource",
        version: "0.11.0",
    },
    "solana-pubkey": {
        version: "^3.0.0",
        features: ["borsh"],
    },
    "solana-client": "^3.0.3",
    "solana-instruction": {
        version: "~3.0.0",
        defaultFeatures: false,
    },
    "solana-account": "3.0.0",
    "solana-commitment-config": "~3.0.0",
    "solana-account-decoder": "^3.0.5",
    "yellowstone-grpc-client": {
        git: "https://github.com/rpcpool/yellowstone-grpc",
        rev: "73c43e1112f6b3432a6b2df9bad73438f6c51034",
    },
    "yellowstone-grpc-proto": {
        git: "https://github.com/rpcpool/yellowstone-grpc",
        rev: "73c43e1112f6b3432a6b2df9bad73438f6c51034",
        features: ["convert"],
    },
    "borsh": "1.5.1",
    "sqlx": "0.8.5",
    "sqlx_migrator": "0.17.0",
    "juniper": "0.16.1",
    "axum": "0.8.4",
    "rustls": "0.23",
    "helius": {
        git: "https://github.com/helius-labs/helius-rust-sdk",
        rev: "f62d528283ca009acacebdd343a8cf2bc0fd09cd",
    },
    "futures-util": "0.3.1",
} as const;

export { getCrateDependencyString } from './utils';

