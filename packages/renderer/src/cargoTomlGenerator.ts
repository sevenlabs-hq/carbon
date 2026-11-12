import { kebabCase } from '@codama/nodes';
import { CARBON_MSRV, VERSIONS, getCrateDependencyString } from '@sevenlabs-hq/carbon-versions';
import { isToken2022Program } from './utils/helpers';

export type PackageMetadata = {
    description?: string;
    keywords?: string[];
    categories?: string[];
};

/** Docs site for the crates in the Carbon repo; each decoder has a page at /decoders/<slug>. */
export const CARBON_DOCS_URL = 'https://carbon.sevenlabs.org';

/** The docs page of a decoder crate in the Carbon repo, e.g. carbon-raydium-amm-v4-decoder -> /decoders/raydium-amm-v4. */
export function getDecoderDocsUrl(crateName: string): string {
    const slug = crateName.replace(/^carbon-/, '').replace(/-decoder(?=-|$)/, '');
    return `${CARBON_DOCS_URL}/decoders/${slug}`;
}

/** Returns the crate name the generated Cargo.toml uses, e.g. "carbon-raydium-amm-v4-decoder". */
export function getDecoderPackageName(packageName?: string, programName?: string, versionName?: string): string {
    let name =
        packageName && packageName.trim()
            ? `carbon-${kebabCase(packageName)}-decoder`
            : programName && programName.trim()
              ? `carbon-${kebabCase(programName)}-decoder`
              : 'carbon-decoder';
    if (versionName && versionName.trim()) {
        name = `${name}-${kebabCase(versionName)}`;
    }
    return name;
}

/**
 * README for a decoder crate. Crates published from the Carbon workspace link to their docs page;
 * decoders generated elsewhere get the title only.
 */
export function generateDecoderReadme(options: {
    displayName: string;
    crateName: string;
    programId?: string;
    inCarbonWorkspace: boolean;
}): string {
    const { displayName, crateName, programId, inCarbonWorkspace } = options;
    let md = `# Carbon ${displayName} Decoder\n`;
    if (!inCarbonWorkspace) return md;
    md += `\nDecodes ${displayName} instructions and accounts on Solana into typed Rust, for use in a [Carbon](https://github.com/sevenlabs-hq/carbon) pipeline.`;
    if (programId) md += ` Program ID: \`${programId}\`.`;
    md += `\n\nDocs, with a runnable example and every instruction, account and type: ${getDecoderDocsUrl(crateName)}\n`;
    md += `\n\`\`\`sh\ncargo add ${crateName}\n\`\`\`\n`;
    return md;
}

export function hasPackageMetadata(m?: PackageMetadata): boolean {
    return !!(
        m &&
        (m.description != null ||
            (m.keywords != null && m.keywords.length > 0) ||
            (m.categories != null && m.categories.length > 0))
    );
}

export type DecoderCargoTomlOptions = {
    packageName?: string;
    programName: string;
    originalProgramName?: string; // Original program name from IDL (for token-2022 checks)
    version?: string;
    versionName?: string;
    withPostgres: boolean;
    withGraphQL: boolean;
    withSerde: boolean;
    withBase58?: boolean;
    withSerdeBigArray?: boolean;
    standalone?: boolean;
    workspaceDeps?: boolean;
    packageMetadata?: PackageMetadata;
};

function deriveDescription(packageName?: string, programName?: string): string {
    const base = (packageName && packageName.trim()) || (programName && programName.trim());
    if (!base) return 'Decoder';
    const kebab = kebabCase(base).replace(/-decoder$/, '');
    const words = kebab.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1).toLowerCase());
    return `${words.join(' ')} Decoder`;
}

function deriveKeywords(packageName?: string, programName?: string): string[] {
    const base = (packageName && packageName.trim()) || (programName && programName.trim()) || '';
    const kebab = kebabCase(base).replace(/-decoder$/, '');
    const parts = kebab.split('-').filter(w => w.length > 0);
    return ['solana', 'decoder', ...parts];
}

/** Returns the display name for README title: "Carbon {DisplayName} Decoder" */
export function getReadmeDisplayName(
    packageName?: string,
    programName?: string,
    packageMetadata?: PackageMetadata,
): string {
    if (packageMetadata?.description) {
        const desc = packageMetadata.description.replace(/\s+Decoder\s*$/i, '').trim();
        return desc || 'Decoder';
    }
    const full = deriveDescription(packageName, programName);
    return full.replace(/\s+Decoder\s*$/i, '').trim() || 'Decoder';
}

export function generateDecoderCargoToml(options: DecoderCargoTomlOptions): string {
    const {
        packageName,
        programName,
        originalProgramName,
        version = '0.1.0',
        versionName,
        withPostgres,
        withGraphQL,
        withSerde,
        withBase58 = false,
        withSerdeBigArray = false,
        standalone = true,
        workspaceDeps,
        packageMetadata,
    } = options;

    // Use workspace dependencies if explicitly set, otherwise default to !standalone
    // When standalone: false and part of main workspace (with workspace.dependencies), use workspace = true
    // When standalone: false but scaffold (no workspace.dependencies), use direct versions
    const useWorkspace = workspaceDeps !== undefined ? workspaceDeps : !standalone;

    // Only add description, keywords, categories, readme, license when explicitly passed via packageMetadata
    const hasMeta = hasPackageMetadata(packageMetadata);
    const description = hasMeta ? (packageMetadata!.description ?? deriveDescription(packageName, programName)) : '';
    const keywords = hasMeta ? (packageMetadata!.keywords ?? deriveKeywords(packageName, programName)) : [];
    const categories = hasMeta ? (packageMetadata!.categories ?? ['encoding']) : [];

    const decoderPackageName = getDecoderPackageName(packageName, programName, versionName);

    const carbonCoreDep = getCrateDependencyString(
        'carbon-core',
        VERSIONS['carbon-core'],
        ['macros'],
        undefined,
        useWorkspace,
    );
    const carbonTestUtilsDep = getCrateDependencyString(
        'carbon-test-utils',
        VERSIONS['carbon-test-utils'],
        undefined,
        undefined,
        useWorkspace,
    );
    const borshDep = getCrateDependencyString('borsh', VERSIONS['borsh'], ['derive'], undefined, useWorkspace);
    const solanaPubkeyDep = getCrateDependencyString(
        'solana-pubkey',
        VERSIONS['solana-pubkey'],
        undefined,
        undefined,
        useWorkspace,
    );
    const solanaAccountDep = getCrateDependencyString(
        'solana-account',
        VERSIONS['solana-account'],
        undefined,
        undefined,
        useWorkspace,
    );
    const solanaInstructionDep = getCrateDependencyString(
        'solana-instruction',
        VERSIONS['solana-instruction'],
        undefined,
        undefined,
        useWorkspace,
    );
    const serdeDep = getCrateDependencyString('serde', VERSIONS['serde'], undefined, true, useWorkspace);
    const serdeJsonDep = getCrateDependencyString(
        'serde_json',
        VERSIONS['serde_json'],
        undefined,
        undefined,
        useWorkspace,
    );
    const serdeBigArrayDep = getCrateDependencyString(
        'serde-big-array',
        VERSIONS['serde-big-array'],
        undefined,
        true,
        useWorkspace,
    );
    const asyncTraitDep = getCrateDependencyString(
        'async-trait',
        VERSIONS['async-trait'],
        undefined,
        true,
        useWorkspace,
    );
    const sqlxDep = getCrateDependencyString(
        'sqlx',
        VERSIONS['sqlx'],
        ['postgres', 'rust_decimal'],
        true,
        useWorkspace,
    );
    const sqlxMigratorDep = getCrateDependencyString(
        'sqlx_migrator',
        VERSIONS['sqlx_migrator'],
        undefined,
        true,
        useWorkspace,
    );
    const juniperDep = getCrateDependencyString('juniper', VERSIONS['juniper'], undefined, true, useWorkspace);

    const features: string[] = ['default = []'];

    if (withSerde || withPostgres || withGraphQL || withBase58) {
        features.push('');
        const serdeDeps = withSerdeBigArray ? 'serde = ["dep:serde", "dep:serde-big-array"]' : 'serde = ["dep:serde"]';
        features.push(serdeDeps);
    }

    if (withPostgres || withGraphQL) {
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
        features.push('    "serde",');
        features.push(']');
    }

    if (withBase58) {
        features.push('');
        features.push('base58 = ["serde"]');
    }

    const dependencies: string[] = [
        carbonCoreDep,
        borshDep,
        solanaPubkeyDep,
        solanaAccountDep,
        solanaInstructionDep,
        serdeJsonDep,
    ];

    if (withSerde || withPostgres || withGraphQL || withBase58) {
        dependencies.push('');
        dependencies.push(serdeDep);
        if (withSerdeBigArray) {
            dependencies.push(serdeBigArrayDep);
        }
    }

    if (withPostgres || withGraphQL) {
        dependencies.push('');
        dependencies.push(sqlxDep);
        dependencies.push(asyncTraitDep);
        dependencies.push(sqlxMigratorDep);
    }

    if (withGraphQL) {
        dependencies.push('');
        dependencies.push(juniperDep);
    }

    // Add SPL Token 2022 dependencies for token-2022 program
    // Check originalProgramName or packageName to handle PascalCase transformation
    if (isToken2022Program(undefined, originalProgramName, packageName)) {
        dependencies.push('');
        dependencies.push(
            getCrateDependencyString(
                'solana-program-pack',
                VERSIONS['solana-program-pack'],
                undefined,
                undefined,
                useWorkspace,
            ),
        );
        dependencies.push(
            getCrateDependencyString('spl-token-2022', VERSIONS['spl-token-2022'], undefined, undefined, useWorkspace),
        );
        dependencies.push(getCrateDependencyString('spl-pod', VERSIONS['spl-pod'], ['borsh'], undefined, useWorkspace));
        dependencies.push(
            getCrateDependencyString(
                'spl-token-metadata-interface',
                VERSIONS['spl-token-metadata-interface'],
                undefined,
                undefined,
                useWorkspace,
            ),
        );
        dependencies.push(
            getCrateDependencyString(
                'spl-token-group-interface',
                VERSIONS['spl-token-group-interface'],
                undefined,
                undefined,
                useWorkspace,
            ),
        );
        dependencies.push(
            getCrateDependencyString(
                'spl-type-length-value',
                VERSIONS['spl-type-length-value'],
                undefined,
                undefined,
                useWorkspace,
            ),
        );
    }

    const packageLines: string[] = [
        '[package]',
        `name = "${decoderPackageName}"`,
        `version = "${version}"`,
        useWorkspace ? 'edition = { workspace = true }' : 'edition = "2021"',
        useWorkspace ? 'rust-version = { workspace = true }' : `rust-version = "${CARBON_MSRV}"`,
    ];
    if (hasMeta) {
        packageLines.push(`description = "${description.replace(/"/g, '\\"')}"`);
        if (useWorkspace) {
            packageLines.push(
                'license = { workspace = true }',
                'readme = "README.md"',
                'repository = { workspace = true }',
                `homepage = "${getDecoderDocsUrl(decoderPackageName)}"`,
            );
        } else {
            packageLines.push('readme = "README.md"');
        }
        packageLines.push(
            `keywords = [${keywords.map(k => `"${k}"`).join(', ')}]`,
            `categories = [${categories.map(c => `"${c}"`).join(', ')}]`,
            '',
        );
    }

    const toml = [
        ...packageLines,
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
