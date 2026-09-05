import { kebabCase } from '@codama/nodes';
import { CARBON_MSRV, VERSIONS, getCrateDependencyString } from '@sevenlabs-hq/carbon-versions';
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
    if (opts.withBase58) decoderFeatures.push('base58');

    const coreFeatures: string[] = [];
    if (opts.withPostgres) coreFeatures.push('postgres');
    if (opts.withGraphql) coreFeatures.push('graphql');
    const carbonCoreDep = getCrateDependencyString('carbon-core', VERSIONS['carbon-core'], coreFeatures);
    const tokioDep = getCrateDependencyString('tokio', VERSIONS['tokio']);
    const dotenvDep = getCrateDependencyString('dotenv', VERSIONS['dotenv']);
    const envLoggerDep = getCrateDependencyString('env_logger', VERSIONS['env_logger']);
    const logDep = getCrateDependencyString('log', VERSIONS.log);

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
        ? [getCrateDependencyString('yellowstone-grpc-proto', VERSIONS['yellowstone-grpc-proto'])]
        : [];

    const rustlsDep = isGrpcDataSource ? getCrateDependencyString('rustls', VERSIONS.rustls) : null;

    const crawlerDeps =
        opts.dataSource === 'rpc-transaction-crawler'
            ? getCrateDependencyString('solana-commitment-config', VERSIONS['solana-commitment-config'])
            : null;

    const rpcClientDep =
        opts.dataSource === 'rpc-block-subscribe' || opts.dataSource === 'rpc-program-subscribe'
            ? getCrateDependencyString('solana-client', VERSIONS['solana-client'])
            : null;

    const programDep =
        opts.dataSource === 'rpc-program-subscribe'
            ? getCrateDependencyString('solana-account-decoder', VERSIONS['solana-account-decoder'])
            : null;

    const blockSubscribeDep =
        opts.dataSource === 'rpc-block-subscribe'
            ? getCrateDependencyString('solana-transaction-status', VERSIONS['solana-transaction-status'])
            : null;

    const pgDeps = opts.withPostgres
        ? [
              getCrateDependencyString('sqlx', VERSIONS.sqlx, ['postgres', 'runtime-tokio-rustls', 'macros']),
              getCrateDependencyString('sqlx_migrator', VERSIONS['sqlx_migrator']),
          ]
        : [];

    const gqlDeps = opts.withGraphql ? [getCrateDependencyString('axum', VERSIONS.axum)] : [];

    const features: string[] = ['default = []'];
    if (opts.withPostgres) {
        features.push('postgres = []');
    }
    if (opts.withGraphql) {
        features.push('graphql = []');
    }

    const dependencies: string[] = [carbonCoreDep, decoderDep, datasourceDep, metricsDep];

    if (crawlerDeps) {
        dependencies.push(crawlerDeps);
    }
    if (rpcClientDep) {
        dependencies.push(rpcClientDep);
    }

    if (programDep) {
        dependencies.push(programDep);
    }

    if (blockSubscribeDep) {
        dependencies.push(blockSubscribeDep);
    }

    dependencies.push(tokioDep, dotenvDep, envLoggerDep, logDep);

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
        `rust-version = "${CARBON_MSRV}"`,
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
