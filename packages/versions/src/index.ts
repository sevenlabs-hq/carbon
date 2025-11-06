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
    'carbon-core': {
        version: '0.11.0',
        defaultFeatures: false,
    },
    'carbon-test-utils': '0.11.0',
    'carbon-log-metrics': '0.11.0',
    'carbon-prometheus-metrics': '0.11.0',
    'carbon-helius-atlas-ws-datasource': '0.11.0',
    'carbon-helius-laserstream-datasource': '0.11.0',
    'carbon-jito-shredstream-grpc-datasource': '0.11.0',
    'carbon-rpc-block-crawler-datasource': '0.11.0',
    'carbon-rpc-block-subscribe-datasource': '0.11.0',
    'carbon-rpc-program-subscribe-datasource': '0.11.0',
    'carbon-rpc-transaction-crawler-datasource': '0.11.0',
    'carbon-stream-message-datasource': '0.11.0',
    'carbon-yellowstone-grpc-datasource': '0.11.0',
    'solana-pubkey': {
        version: '^3.0.0',
        features: ['borsh'],
    },
    'solana-client': '^3.0.3',
    'solana-instruction': {
        version: '~3.0.0',
        defaultFeatures: false,
    },
    'solana-account': '3.0.0',
    'solana-commitment-config': '~3.0.0',
    'solana-account-decoder': '^3.0.5',
    'yellowstone-grpc-client': {
        git: 'https://github.com/rpcpool/yellowstone-grpc',
        rev: '73c43e1112f6b3432a6b2df9bad73438f6c51034',
    },
    'yellowstone-grpc-proto': {
        git: 'https://github.com/rpcpool/yellowstone-grpc',
        rev: '73c43e1112f6b3432a6b2df9bad73438f6c51034',
        features: ['convert'],
    },
    borsh: '1.5.1',
    sqlx: '0.8.5',
    sqlx_migrator: '0.17.0',
    juniper: '0.16.1',
    axum: '0.8.4',
    rustls: '0.23',
    helius: {
        git: 'https://github.com/helius-labs/helius-rust-sdk',
        rev: 'f62d528283ca009acacebdd343a8cf2bc0fd09cd',
    },
    'futures-util': '0.3.1',
    serde: {
        version: '1.0.208',
        features: ['derive'],
    },
    serde_json: '1.0.138',
    'serde-big-array': '0.5.1',
    'async-trait': '0.1.86',
    base64: '0.22.1',
    tokio: {
        version: '1',
        features: ['rt-multi-thread', 'macros'],
    },
    dotenv: '0.15.0',
    env_logger: '0.11.5',
    log: '0.4.25',
    anyhow: '1.0.96',
    tracing: '0.1',
    'tracing-subscriber': {
        version: '0.3',
        features: ['fmt', 'env-filter'],
    },
} as const;

export { getCrateDependencyString } from './utils';
