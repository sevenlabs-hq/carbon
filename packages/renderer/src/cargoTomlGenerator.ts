import { kebabCase } from '@codama/nodes';
import { VERSIONS, getCrateDependencyString } from '@sevenlabs-hq/carbon-versions';

export type DecoderCargoTomlOptions = {
    packageName?: string;
    programName: string;
    withPostgres: boolean;
    withGraphQL: boolean;
    withSerde: boolean;
    standalone?: boolean;
};

export function generateDecoderCargoToml(options: DecoderCargoTomlOptions): string {
    const { packageName, programName, withPostgres, withGraphQL, withSerde, standalone = true } = options;

    const decoderPackageName = packageName
        ? `carbon-${kebabCase(packageName)}-decoder`
        : `carbon-${kebabCase(programName)}-decoder`;

    const carbonCoreDep = getCrateDependencyString('carbon-core', VERSIONS['carbon-core'], ['macros']);
    const carbonTestUtilsDep = getCrateDependencyString('carbon-test-utils', VERSIONS['carbon-test-utils']);
    const borshDep = getCrateDependencyString('borsh', VERSIONS['borsh'], ['derive']);
    const solanaPubkeyDep = getCrateDependencyString('solana-pubkey', VERSIONS['solana-pubkey']);
    const solanaAccountDep = getCrateDependencyString('solana-account', VERSIONS['solana-account']);
    const solanaInstructionDep = getCrateDependencyString('solana-instruction', VERSIONS['solana-instruction']);
    const serdeDep = getCrateDependencyString('serde', VERSIONS['serde'], undefined, true);
    const serdeJsonDep = getCrateDependencyString('serde_json', VERSIONS['serde_json']);
    const serdeBigArrayDep = getCrateDependencyString('serde-big-array', VERSIONS['serde-big-array'], undefined, true);
    const asyncTraitDep = getCrateDependencyString('async-trait', VERSIONS['async-trait'], undefined, true);
    const sqlxDep = getCrateDependencyString('sqlx', VERSIONS['sqlx'], ['postgres', 'rust_decimal'], true);
    const sqlxMigratorDep = getCrateDependencyString('sqlx_migrator', VERSIONS['sqlx_migrator'], undefined, true);
    const juniperDep = getCrateDependencyString('juniper', VERSIONS['juniper'], undefined, true);
    const base64Dep = getCrateDependencyString('base64', VERSIONS['base64'], undefined, true);

    const features: string[] = ['default = []'];

    if (withSerde || withPostgres || withGraphQL) {
        features.push('');
        features.push('serde = ["dep:serde", "dep:serde-big-array"]');
    }

    if (withPostgres) {
        features.push('');
        features.push('postgres = [');
        features.push('    "carbon-core/postgres",');
        features.push('    "dep:sqlx",');
        features.push('    "dep:async-trait",');
        features.push('    "dep:sqlx_migrator",');
        features.push('    "serde",');
        features.push(']');
    }

    if (withGraphQL) {
        features.push('');
        features.push('graphql = [');
        features.push('    "carbon-core/graphql",');
        features.push('    "dep:juniper",');
        features.push('    "dep:base64",');
        features.push('    "serde",');
        features.push(']');
    }

    const dependencies: string[] = [
        carbonCoreDep,
        borshDep,
        solanaPubkeyDep,
        solanaAccountDep,
        solanaInstructionDep,
        serdeJsonDep,
        '',
        serdeDep,
        serdeBigArrayDep,
        '',
        sqlxDep,
        asyncTraitDep,
        sqlxMigratorDep,
        '',
        juniperDep,
        base64Dep,
    ];

    const toml = [
        '[package]',
        `name = "${decoderPackageName}"`,
        'version = "0.1.0"',
        'edition = "2021"',
        '',
        ...(standalone ? ['[workspace]', ''] : []),
        '[lib]',
        'crate-type = ["rlib"]',
        '',
        '[features]',
        ...features,
        '',
        '[dependencies]',
        ...dependencies,
        '',
        '',
        '[dev-dependencies]',
        carbonTestUtilsDep,
        '',
    ].join('\n');

    return toml;
}
