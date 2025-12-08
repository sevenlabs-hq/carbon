import { kebabCase } from '@codama/nodes';
import { VERSIONS, getCrateDependencyString } from '@sevenlabs-hq/carbon-versions';
import { isToken2022Program } from './utils/helpers';

export type DecoderCargoTomlOptions = {
    packageName?: string;
    programName: string;
    originalProgramName?: string; // Original program name from IDL (for token-2022 checks)
    withPostgres: boolean;
    withGraphQL: boolean;
    withSerde: boolean;
    standalone?: boolean;
};

export function generateDecoderCargoToml(options: DecoderCargoTomlOptions): string {
    const {
        packageName,
        programName,
        originalProgramName,
        withPostgres,
        withGraphQL,
        withSerde,
        standalone = true,
    } = options;

    const decoderPackageName =
        packageName && packageName.trim()
            ? `carbon-${kebabCase(packageName)}-decoder`
            : programName && programName.trim()
              ? `carbon-${kebabCase(programName)}-decoder`
              : 'carbon-decoder';

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
    ];

    if (withSerde || withPostgres || withGraphQL) {
        dependencies.push('');
        dependencies.push(serdeDep);
        dependencies.push(serdeBigArrayDep);
    }

    if (withPostgres) {
        dependencies.push('');
        dependencies.push(sqlxDep);
        dependencies.push(asyncTraitDep);
        dependencies.push(sqlxMigratorDep);
    }

    if (withGraphQL) {
        dependencies.push('');
        dependencies.push(juniperDep);
        dependencies.push(base64Dep);
    }

    // Add SPL Token 2022 dependencies for token-2022 program
    // Check originalProgramName or packageName to handle PascalCase transformation
    if (isToken2022Program(undefined, originalProgramName, packageName)) {
        dependencies.push('');
        dependencies.push(getCrateDependencyString('solana-program-pack', VERSIONS['solana-program-pack']));
        dependencies.push(getCrateDependencyString('spl-token-2022', VERSIONS['spl-token-2022']));
        dependencies.push(getCrateDependencyString('spl-pod', VERSIONS['spl-pod'], ['borsh']));
        dependencies.push(
            getCrateDependencyString('spl-token-metadata-interface', VERSIONS['spl-token-metadata-interface']),
        );
        dependencies.push(getCrateDependencyString('spl-token-group-interface', VERSIONS['spl-token-group-interface']));
        dependencies.push(getCrateDependencyString('spl-type-length-value', VERSIONS['spl-type-length-value']));
    }

    const macheteIgnored: string[] = [];
    if (withSerde || withPostgres || withGraphQL) {
        macheteIgnored.push('serde-big-array');
    }
    if (withGraphQL) {
        macheteIgnored.push('base64');
    }

    const toml = [
        '[package]',
        `name = "${decoderPackageName}"`,
        'version = "0.1.0"',
        'edition = "2021"',
        '',
        ...(macheteIgnored.length > 0
            ? [
                  '[package.metadata.cargo-machete]',
                  `ignored = [${macheteIgnored.map(dep => `"${dep}"`).join(', ')}]`,
                  '',
              ]
            : []),
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
