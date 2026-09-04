/**
 * Carbon Version Registry
 *
 * Centralized registry for all Rust crate versions used in Carbon code generation.
 * Carbon-owned entries track the current supported Rust release line.
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

export const CARBON_VERSION = '2.0.0';
export const CARBON_MSRV = '1.96.1';

export const VERSIONS: Record<string, CrateDependency> = {
    /// Carbon crates
    'carbon-core': {
        version: CARBON_VERSION,
        defaultFeatures: false,
    },
    'carbon-test-utils': CARBON_VERSION,
    'carbon-log-metrics': CARBON_VERSION,
    'carbon-prometheus-metrics': CARBON_VERSION,
    'carbon-helius-atlas-ws-datasource': CARBON_VERSION,
    'carbon-helius-gpa-v2-datasource': CARBON_VERSION,
    'carbon-helius-gtfa-datasource': CARBON_VERSION,
    'carbon-helius-laserstream-datasource': CARBON_VERSION,
    'carbon-rpc-block-crawler-datasource': CARBON_VERSION,
    'carbon-rpc-block-subscribe-datasource': CARBON_VERSION,
    'carbon-rpc-gpa-datasource': CARBON_VERSION,
    'carbon-rpc-program-subscribe-datasource': CARBON_VERSION,
    'carbon-rpc-transaction-crawler-datasource': CARBON_VERSION,
    'carbon-stream-message-datasource': CARBON_VERSION,
    'carbon-validator-snapshot-datasource': CARBON_VERSION,
    'carbon-yellowstone-grpc-datasource': CARBON_VERSION,
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
    'solana-transaction-status': {
        version: '=4.2.2',
        features: ['agave-unstable-api'],
    },
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
        version: '=13.5.0',
    },
    'yellowstone-grpc-proto': {
        version: '=12.7.0',
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
