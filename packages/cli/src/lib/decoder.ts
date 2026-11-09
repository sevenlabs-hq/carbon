import { readFileSync } from 'fs';
import { resolve } from 'path';
import { createFromJson, createFromRoot } from 'codama';
import { rootNodeFromAnchorWithoutDefaultVisitor, rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import { renderVisitor, extractStructArrayItems, type PackageMetadata } from '@sevenlabs-hq/carbon-codama-renderer';
import {
    deduplicateIdenticalDefinedTypesVisitor,
    setFixedAccountSizesVisitor,
    setInstructionAccountDefaultValuesVisitor,
    getCommonInstructionAccountDefaultRules,
    unwrapInstructionArgsDefinedTypesVisitor,
    transformU8ArraysToBytesVisitor,
    rootNodeVisitor,
    visit,
} from '@codama/visitors';
import { assertIsNode } from '@codama/nodes';
import { exitWithError, isBase58Like } from './utils';
import { fetchAnchorIdl } from './anchor';
import { logger } from './logger';
import {
    hasLegacyEvents,
    transformLegacyEvents,
    fixPdaSeedArgumentPaths,
    fixPdaSeedAccountReferences,
    hasNestedInstructionArguments,
} from './idl-transformer';

/**
 * Creates a Codama root node from Anchor IDL without flattening instruction arguments
 * This preserves nested structure like Drift
 */
function createFromRootWithoutFlattening(idlJson: any) {
    // First get the raw root node from Anchor IDL without default visitors
    const rawRoot = rootNodeFromAnchorWithoutDefaultVisitor(idlJson);

    // Apply custom visitor pipeline WITHOUT flattenInstructionDataArgumentsVisitor
    const customVisitor = rootNodeVisitor(currentRoot => {
        let root = currentRoot;
        const updateRoot = (visitor: any) => {
            const newRoot = visit(root, visitor) as any;
            assertIsNode(newRoot, 'rootNode');
            root = newRoot;
        };

        // Apply all the standard visitors EXCEPT flattenInstructionDataArgumentsVisitor and unwrapInstructionArgsDefinedTypesVisitor
        updateRoot(deduplicateIdenticalDefinedTypesVisitor());
        updateRoot(setFixedAccountSizesVisitor());
        updateRoot(setInstructionAccountDefaultValuesVisitor(getCommonInstructionAccountDefaultRules()));
        // SKIP: unwrapInstructionArgsDefinedTypesVisitor() - this inlines defined types, causing flattening
        // SKIP: flattenInstructionDataArgumentsVisitor() - this is what preserves nested structure
        updateRoot(transformU8ArraysToBytesVisitor());

        return root;
    });

    return createFromRoot(visit(rawRoot, customVisitor));
}

export type IdlSource = {
    type: 'file' | 'program';
    value: string;
};

export type IdlStandard = 'anchor' | 'codama';

export type DecoderGenerationOptions = {
    idl: string;
    outputDir: string;
    standard?: string;
    url?: string;
    eventCpiDiscriminator?: string;
    deleteFolderBeforeRendering?: boolean;
    programId?: string;
    packageName?: string;
    postgresMode?: 'generic' | 'typed';
    withPostgres?: boolean;
    withGraphql?: boolean;
    withSerde?: boolean;
    withBase58?: boolean;
    standalone?: boolean;
    workspaceDeps?: boolean;
    packageMetadata?: PackageMetadata;
    version?: string;
    versionName?: string;
};

export type IdlMetadata = {
    name: string;
    version?: string;
    address?: string;
    programId?: string;
};

function isValidAddress(addr: unknown): addr is string {
    return (
        typeof addr === 'string' && addr.trim() !== '' && isBase58Like(addr) && addr.length >= 32 && addr.length <= 44
    );
}

/**
 * The program address the IDL declares, if it's a valid one. Codama IDLs keep it in
 * `program.publicKey`. Codama's Anchor converter reads `address` from 0.1.0-spec IDLs and
 * `metadata.address` from older ones, so for Anchor that field is tried first.
 */
export function getValidIdlAddress(idlJson: any, standard: IdlStandard): string | undefined {
    const candidates =
        standard === 'codama'
            ? [idlJson.program?.publicKey]
            : idlJson.metadata?.spec === '0.1.0'
              ? [idlJson.address, idlJson.metadata?.address]
              : [idlJson.metadata?.address, idlJson.address];
    return candidates.find(isValidAddress);
}

/**
 * Makes sure the IDL names its program where the renderer reads it. The IDL's own address
 * wins, then the program the IDL was fetched from, then `--program-id`.
 */
function ensureIdlAddress(idlJson: any, standard: IdlStandard, programId?: string, fetchedFrom?: string): void {
    const idlAddress = getValidIdlAddress(idlJson, standard);
    const address = idlAddress || fetchedFrom || programId;
    if (!address) {
        exitWithError('Program ID is required: the IDL has no valid program address. Provide one with --program-id.');
    }
    if (programId && programId !== address) {
        const source = idlAddress ? "the IDL's program address" : 'the program the IDL was fetched from';
        logger.warning(`--program-id ${programId} is ignored: ${source} is ${address}.`);
    }

    if (standard === 'codama') {
        if (!idlJson.program) {
            exitWithError('The IDL has no Codama program node. Is it an Anchor IDL? Pass --standard anchor.');
        }
        idlJson.program.publicKey = address;
    } else {
        // Both fields, so the converter finds it whichever format the IDL is in.
        idlJson.address = address;
        idlJson.metadata = { ...idlJson.metadata, address };
    }
}

/**
 * Parses the --event-cpi-discriminator hex string into bytes
 */
export function parseEventCpiDiscriminator(hex?: string): number[] | undefined {
    if (hex === undefined) {
        return undefined;
    }
    const cleaned = hex.trim().replace(/^0x/i, '');
    if (!/^[0-9a-fA-F]+$/.test(cleaned) || cleaned.length % 2 !== 0) {
        exitWithError('Invalid --event-cpi-discriminator: expected an even-length hex string (e.g. e445a52e51cb9a1d)');
    }
    const bytes: number[] = [];
    for (let index = 0; index < cleaned.length; index += 2) {
        bytes.push(parseInt(cleaned.slice(index, index + 2), 16));
    }
    return bytes;
}

/**
 * Validates and determines the IDL source type
 */
export function parseIdlSource(idl: string): IdlSource {
    const looksLikeFile = idl.endsWith('.json');
    const looksLikeProgram = !looksLikeFile && idl.length >= 32 && idl.length <= 44 && isBase58Like(idl);

    if (!looksLikeFile && !looksLikeProgram) {
        exitWithError('Invalid IDL: must be a .json file or a Solana program address');
    }

    return {
        type: looksLikeProgram ? 'program' : 'file',
        value: idl,
    };
}

/**
 * Validates and normalizes the IDL standard
 */
export function validateIdlStandard(standard: string, idlSource: IdlSource): IdlStandard {
    const normalized = standard.toLowerCase() as IdlStandard;

    if (normalized !== 'anchor' && normalized !== 'codama') {
        exitWithError("IDL standard must be 'anchor' or 'codama'");
    }

    if (normalized === 'codama' && idlSource.type === 'program') {
        exitWithError('Codama standard is only supported with IDL files, not program addresses');
    }

    return normalized;
}

/**
 * Validates program ID format
 */
export function validateProgramId(programId: string): void {
    if (!programId || programId.trim() === '') {
        exitWithError('Program ID cannot be empty');
    }

    if (!isBase58Like(programId) || programId.length < 32 || programId.length > 44) {
        exitWithError('Invalid Program ID format (must be base58, 32-44 characters)');
    }
}

/**
 * Validates options for decoder generation
 */
export function validateDecoderOptions(
    standard: IdlStandard,
    idlSource: IdlSource,
    url?: string,
    programId?: string,
    eventCpiDiscriminator?: string,
): void {
    if (idlSource.type === 'program') {
        if (!url) {
            exitWithError('RPC URL is required when using a program address');
        }
        if (standard !== 'anchor') {
            exitWithError('Only Anchor standard is supported when using a program address');
        }
    }

    if (standard === 'anchor' && eventCpiDiscriminator) {
        exitWithError('--event-cpi-discriminator can only be used with Codama standard');
    }

    if (programId) {
        validateProgramId(programId);
    }
}

/**
 * Extracts metadata from an IDL
 */
export async function getIdlMetadata(
    idl: string,
    standard?: string,
    url?: string,
    programId?: string,
): Promise<IdlMetadata> {
    const idlSource = parseIdlSource(idl);
    const normalizedStandard = validateIdlStandard(standard || 'anchor', idlSource);

    let idlJson: any;

    if (idlSource.type === 'program') {
        if (!url) {
            exitWithError('RPC URL is required when using a program address');
        }
        idlJson = await fetchAnchorIdl(idl, url);
    } else {
        const idlPath = resolve(process.cwd(), idl);
        idlJson = JSON.parse(readFileSync(idlPath, 'utf8'));
    }

    // Extract name and convert to kebab-case
    const name = idlJson.name || idlJson.program?.name || idlJson.metadata?.name || 'custom-decoder';
    const kebabName = name
        .replace(/([a-z])([A-Z])/g, '$1-$2')
        .replace(/[\s_]+/g, '-')
        .toLowerCase();

    const idlAddress = getValidIdlAddress(idlJson, normalizedStandard);

    const finalAddress = idlAddress || (idlSource.type === 'program' ? idl : undefined) || programId;

    return {
        name: kebabName,
        version: idlJson.version || idlJson.metadata?.version,
        address: finalAddress,
        programId: programId,
    };
}

/**
 * Generates a decoder from an IDL
 */
export async function generateDecoder(options: DecoderGenerationOptions): Promise<void> {
    const {
        idl,
        outputDir,
        standard: standardOpt,
        url,
        deleteFolderBeforeRendering = true,
        programId,
        packageName,
        postgresMode,
        withPostgres,
        withGraphql,
        withSerde,
        withBase58,
        standalone,
        workspaceDeps,
        packageMetadata,
        version,
        versionName,
    } = options;

    const idlSource = parseIdlSource(idl);
    const standard = validateIdlStandard(standardOpt || 'anchor', idlSource);
    validateDecoderOptions(standard, idlSource, url, programId, options.eventCpiDiscriminator);

    // Common render options shared across all branches
    const renderOpts = {
        deleteFolderBeforeRendering,
        eventCpiDiscriminator: parseEventCpiDiscriminator(options.eventCpiDiscriminator),
        packageName,
        postgresMode,
        withPostgres,
        withGraphql,
        withSerde,
        withBase58,
        standalone,
        workspaceDeps,
        packageMetadata,
        version,
        versionName,
    };

    // Load IDL from program address or file
    let idlJson: any;
    if (idlSource.type === 'program') {
        idlJson = await fetchAnchorIdl(idl, url!);
        ensureIdlAddress(idlJson, standard, programId, idl);
    } else {
        const idlPath = resolve(process.cwd(), idl);
        idlJson = JSON.parse(readFileSync(idlPath, 'utf8'));
        ensureIdlAddress(idlJson, standard, programId);
    }

    // Codama standard: no anchor transformations needed
    if (standard === 'codama') {
        const codama = createFromJson(JSON.stringify(idlJson));
        codama.update(extractStructArrayItems());
        codama.accept(renderVisitor(outputDir, renderOpts));
        return;
    }

    // Anchor standard: apply event and PDA normalization
    if (hasLegacyEvents(idlJson)) {
        idlJson = transformLegacyEvents(idlJson);
    }
    idlJson = fixPdaSeedArgumentPaths(idlJson);
    idlJson = fixPdaSeedAccountReferences(idlJson);

    if (standard === 'anchor') {
        // Check if we need to preserve nested structure
        const needsNestedPreservation = hasNestedInstructionArguments(idlJson);

        // Use custom pipeline only if we have nested arguments to preserve
        const codama = needsNestedPreservation
            ? createFromRootWithoutFlattening(idlJson)
            : createFromRoot(rootNodeFromAnchorWithoutDefaultVisitor(idlJson));
        codama.accept(
            renderVisitor(outputDir, {
                deleteFolderBeforeRendering,
                packageName,
                anchorEvents: idlJson.events,
                postgresMode,
                withPostgres,
                withGraphql,
                withSerde,
                withBase58,
                standalone,
                workspaceDeps,
                packageMetadata,
                version: options.version,
                versionName: options.versionName,
            }),
        );
    }
}
