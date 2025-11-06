import { readFileSync } from 'fs';
import { resolve } from 'path';
import { createFromJson, createFromRoot } from 'codama';
import { rootNodeFromAnchorWithoutDefaultVisitor, rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import { renderVisitor } from '@sevenlabs-hq/carbon-codama-renderer';
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
import {
    hasLegacyEvents,
    transformLegacyEvents,
    getTransformationInfo,
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
    eventHints?: string;
    deleteFolderBeforeRendering?: boolean;
    programId?: string;
    packageName?: string;
    postgresMode?: 'generic' | 'typed';
    withPostgres?: boolean;
    withGraphql?: boolean;
    withSerde?: boolean;
    standalone?: boolean;
};

export type IdlMetadata = {
    name: string;
    version?: string;
    address?: string;
    programId?: string;
};

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
    eventHints?: string,
    programId?: string,
): void {
    if (idlSource.type === 'program') {
        if (!url) {
            exitWithError('RPC URL is required when using a program address');
        }
        if (standard !== 'anchor') {
            exitWithError('Only Anchor standard is supported when using a program address');
        }
    }

    if (standard === 'anchor' && eventHints) {
        exitWithError('Event hints can only be used with Codama standard');
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

    const idlAddress = idlJson.address || idlJson.metadata?.address;
    const isValidAddress =
        idlAddress &&
        idlAddress.trim() !== '' &&
        isBase58Like(idlAddress) &&
        idlAddress.length >= 32 &&
        idlAddress.length <= 44;

    const finalAddress =
        (isValidAddress ? idlAddress : undefined) || (idlSource.type === 'program' ? idl : undefined) || programId;

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
        standalone,
    } = options;

    const idlSource = parseIdlSource(idl);
    const standard = validateIdlStandard(standardOpt || 'anchor', idlSource);
    validateDecoderOptions(standard, idlSource, url, options.eventHints, programId);

    if (idlSource.type === 'program') {
        let idlJson = await fetchAnchorIdl(idl, url!);

        const idlAddress = idlJson.address || idlJson.metadata?.address;
        const isValidAddress =
            idlAddress &&
            idlAddress.trim() !== '' &&
            isBase58Like(idlAddress) &&
            idlAddress.length >= 32 &&
            idlAddress.length <= 44;

        const programAddress = (isValidAddress ? idlAddress : undefined) || idl || programId;
        if (!programAddress) {
            exitWithError(
                'Program ID is required. IDL missing or invalid address field - provide via --program-id parameter',
            );
        }

        if (!isValidAddress) {
            idlJson.address = programAddress;
            if (!idlJson.metadata) idlJson.metadata = {};
            idlJson.metadata.address = programAddress;
        }

        if (hasLegacyEvents(idlJson)) {
            const info = getTransformationInfo(idlJson);
            idlJson = transformLegacyEvents(idlJson);
        }

        // Apply IDL normalization for nested structure
        idlJson = fixPdaSeedArgumentPaths(idlJson);
        idlJson = fixPdaSeedAccountReferences(idlJson);

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
                standalone,
            }),
        );
        return;
    }

    // File-based IDL
    const idlPath = resolve(process.cwd(), idl);
    let idlJson = JSON.parse(readFileSync(idlPath, 'utf8'));

    const idlAddress = idlJson.address || idlJson.metadata?.address;
    const isValidAddress =
        idlAddress &&
        idlAddress.trim() !== '' &&
        isBase58Like(idlAddress) &&
        idlAddress.length >= 32 &&
        idlAddress.length <= 44;

    if (!isValidAddress && !programId) {
        exitWithError(
            'Program ID is required. IDL missing or invalid address field - provide via --program-id parameter',
        );
    }

    const programAddress = (isValidAddress ? idlAddress : undefined) || programId;
    if (!programAddress) {
        exitWithError(
            'Program ID is required. IDL missing or invalid address field - provide via --program-id parameter',
        );
    }

    if (!isValidAddress) {
        idlJson.address = programAddress;
        if (!idlJson.metadata) idlJson.metadata = {};
        idlJson.metadata.address = programAddress;
    }

    if (hasLegacyEvents(idlJson)) {
        const info = getTransformationInfo(idlJson);
        idlJson = transformLegacyEvents(idlJson);
    }

    // Apply IDL normalization for nested structure (fixes PDA seed paths)
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
                standalone,
            }),
        );
    } else {
        const codama = createFromJson(JSON.stringify(idlJson));
        codama.accept(
            renderVisitor(outputDir, {
                deleteFolderBeforeRendering,
                packageName,
                postgresMode,
                withPostgres,
                withGraphql,
                withSerde,
                standalone,
            }),
        );
    }
}
