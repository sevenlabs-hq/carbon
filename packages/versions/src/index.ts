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
          branch?: string;
          features?: readonly string[];
          defaultFeatures?: boolean;
      };

export const VERSIONS: Record<string, CrateDependency> = {
    /// Carbon crates
    'carbon-core': {
        version: '1.0.0',
        defaultFeatures: false,
    },
    'carbon-test-utils': '1.0.0',
    'carbon-log-metrics': '1.0.0',
    'carbon-prometheus-metrics': '1.0.0',
    'carbon-helius-atlas-ws-datasource': '1.0.0',
    'carbon-helius-laserstream-datasource': '1.0.0',
    'carbon-jito-shredstream-grpc-datasource': '0.12.0',
    'carbon-rpc-block-crawler-datasource': '1.0.0',
    'carbon-rpc-block-subscribe-datasource': '1.0.0',
    'carbon-rpc-program-subscribe-datasource': '1.0.0',
    'carbon-rpc-transaction-crawler-datasource': '1.0.0',
    'carbon-stream-message-datasource': '1.0.0',
    'carbon-yellowstone-grpc-datasource': '1.0.0',
    /// Solana crates
    'solana-account': '4.3.2',
    'solana-account-decoder': {
        version: '=4.2.2',
        features: ['agave-unstable-api'],
    },
    'solana-client': '=4.2.2',
    'solana-instruction': {
        version: '3.4.1',
        defaultFeatures: false,
    },
    'solana-pubkey': {
        version: '4.2.1',
        features: ['borsh'],
    },
    'solana-commitment-config': '3.1.1',
    /// SPL Token 2022 dependencies
    'solana-program-pack': '3.1.0',
    'spl-token-2022': '11.0.0',
    'spl-pod': {
        version: '0.7.3',
        features: ['borsh'],
    },
    'spl-token-metadata-interface': '1.0.0',
    'spl-token-group-interface': '0.7.2',
    'spl-type-length-value': '0.9.1',
    'yellowstone-grpc-client': {
        version: '=13.3.0',
    },
    'yellowstone-grpc-proto': {
        version: '=12.6.0',
    },
    /// Other crates
    borsh: '1.5.1',
    sqlx: '0.8.6',
    sqlx_migrator: '0.17.0',
    juniper: '0.16.2',
    axum: '0.8.7',
    rustls: '0.23',
    helius: '2.0.0',
    'futures-util': '0.3.31',
    serde: {
        version: '1.0.228',
        features: ['derive'],
    },
    serde_json: '1.0.145',
    'serde-big-array': '0.5.1',
    'async-trait': '0.1.89',
    tokio: {
        version: '1',
        features: ['rt-multi-thread', 'macros'],
    },
    dotenv: '0.15.0',
    env_logger: '0.11.8',
    log: '0.4.28',
    anyhow: '1.0.100',
    tracing: '0.1',
    'tracing-subscriber': {
        version: '0.3',
        features: ['fmt', 'env-filter'],
    },
} as const;

export { getCrateDependencyString } from './utils';
