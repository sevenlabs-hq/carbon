import { kebabCase } from '@codama/nodes';
import { VERSIONS, getCrateDependencyString } from '@sevenlabs-hq/carbon-versions';
import type { ScaffoldOptions } from './scaffold';
import { exitWithError } from './utils';

function requireVersion(crateName: string, context: string): NonNullable<(typeof VERSIONS)[string]> {
    const version = VERSIONS[crateName as keyof typeof VERSIONS];
    if (!version) {
        exitWithError(`Missing version for ${crateName} in VERSIONS registry. ${context}`);
    }
    return version;
}

export function generateIndexerCargoToml(opts: ScaffoldOptions): string {
    const decoderCrateName = `carbon-${kebabCase(opts.decoder)}-decoder`;

    const decoderFeatures: string[] = [];
    if (opts.withPostgres) decoderFeatures.push('postgres');
    if (opts.withGraphql) decoderFeatures.push('graphql');
    if (opts.withSerde) decoderFeatures.push('serde');

    const carbonCoreDep = getCrateDependencyString('carbon-core', VERSIONS['carbon-core'], ['postgres', 'graphql']);
    const asyncTraitDep = getCrateDependencyString('async-trait', VERSIONS['async-trait']);
    const solanaPubkeyDep = getCrateDependencyString('solana-pubkey', VERSIONS['solana-pubkey']);
    const solanaClientDep = getCrateDependencyString('solana-client', VERSIONS['solana-client']);
    const solanaInstructionDep = getCrateDependencyString('solana-instruction', VERSIONS['solana-instruction']);
    const tokioDep = getCrateDependencyString('tokio', VERSIONS['tokio']);
    const dotenvDep = getCrateDependencyString('dotenv', VERSIONS['dotenv']);
    const envLoggerDep = getCrateDependencyString('env_logger', VERSIONS['env_logger']);
    const logDep = getCrateDependencyString('log', VERSIONS['log']);
    const anyhowDep = getCrateDependencyString('anyhow', VERSIONS['anyhow']);
    const tracingDep = getCrateDependencyString('tracing', VERSIONS['tracing']);
    const tracingSubscriberDep = getCrateDependencyString('tracing-subscriber', VERSIONS['tracing-subscriber']);

    const decoderFeaturesStr =
        decoderFeatures.length > 0 ? `, features = [${decoderFeatures.map(f => `"${f}"`).join(', ')}]` : '';
    const decoderDep = `${decoderCrateName} = { path = "../decoder"${decoderFeaturesStr} }`;

    const datasourceCrateName = `carbon-${opts.dataSource.toLowerCase()}-datasource`;
    const datasourceVersion = requireVersion(
        datasourceCrateName,
        `Datasource "${opts.dataSource}" not found in VERSIONS registry. Expected crate name: ${datasourceCrateName}`,
    );
    const datasourceDep = getCrateDependencyString(datasourceCrateName, datasourceVersion);

    const metricsCrateName = `carbon-${opts.metrics.toLowerCase()}-metrics`;
    const metricsVersion = requireVersion(
        metricsCrateName,
        `Metrics "${opts.metrics}" not found in VERSIONS registry. Expected crate name: ${metricsCrateName}`,
    );
    const metricsDep = getCrateDependencyString(metricsCrateName, metricsVersion);

    const isGrpcDataSource = opts.dataSource === 'yellowstone-grpc' || opts.dataSource === 'helius-laserstream';
    const grpcDeps = isGrpcDataSource
        ? [
              getCrateDependencyString('yellowstone-grpc-client', VERSIONS['yellowstone-grpc-client']),
              getCrateDependencyString('yellowstone-grpc-proto', VERSIONS['yellowstone-grpc-proto']),
          ]
        : [];

    const rustlsDep = isGrpcDataSource ? getCrateDependencyString('rustls', VERSIONS.rustls) : null;

    const crawlerDeps =
        opts.dataSource === 'rpc-transaction-crawler'
            ? getCrateDependencyString('solana-commitment-config', VERSIONS['solana-commitment-config'])
            : null;

    const programDeps =
        opts.dataSource === 'rpc-program-subscribe'
            ? getCrateDependencyString('solana-account-decoder', VERSIONS['solana-account-decoder'])
            : null;

    const pgDeps = opts.withPostgres
        ? [
              getCrateDependencyString('sqlx', VERSIONS.sqlx, ['postgres', 'runtime-tokio-rustls', 'macros']),
              getCrateDependencyString('sqlx_migrator', VERSIONS['sqlx_migrator']),
          ]
        : [];

    const gqlDeps = opts.withGraphql
        ? [getCrateDependencyString('juniper', VERSIONS.juniper), getCrateDependencyString('axum', VERSIONS.axum)]
        : [];

    const features: string[] = ['default = []'];
    if (opts.withPostgres) {
        features.push('postgres = []');
    }
    if (opts.withGraphql) {
        features.push('graphql = []');
    }

    const dependencies: string[] = [asyncTraitDep, carbonCoreDep, decoderDep, datasourceDep, metricsDep];

    if (crawlerDeps) {
        dependencies.push(crawlerDeps);
    }
    if (programDeps) {
        dependencies.push(programDeps);
    }

    dependencies.push(
        solanaPubkeyDep,
        solanaClientDep,
        solanaInstructionDep,
        tokioDep,
        dotenvDep,
        envLoggerDep,
        logDep,
        anyhowDep,
        tracingDep,
        tracingSubscriberDep,
    );

    if (rustlsDep) {
        dependencies.push(rustlsDep);
    }
    if (grpcDeps.length > 0) {
        dependencies.push(...grpcDeps);
    }
    if (pgDeps.length > 0) {
        dependencies.push(...pgDeps);
    }
    if (gqlDeps.length > 0) {
        dependencies.push(...gqlDeps);
    }

    const toml = [
        '[package]',
        `name = "${opts.name}-indexer"`,
        'version = "0.0.1"',
        'edition = "2021"',
        '',
        '[dependencies]',
        ...dependencies,
        '',
        '[features]',
        ...features,
        '',
    ].join('\n');

    return toml;
}
