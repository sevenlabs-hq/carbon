import { readFileSync } from 'fs';
import { resolve } from 'path';
import { createFromJson, createFromRoot } from 'codama';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import { renderVisitor } from '@sevenlabs-hq/carbon-codama-renderer';
import { exitWithError, isBase58Like } from './utils';
import { fetchAnchorIdl } from './anchor';
import { hasLegacyEvents, transformLegacyEvents, getTransformationInfo } from './idl-transformer';

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
};

export type IdlMetadata = {
    name: string;
    version?: string;
    address?: string;
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
 * Validates options for decoder generation
 */
export function validateDecoderOptions(
    standard: IdlStandard,
    idlSource: IdlSource,
    url?: string,
    eventHints?: string,
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
        exitWithError("Event hints can only be used with Codama standard");
    }
}

/**
 * Extracts metadata from an IDL
 */
export async function getIdlMetadata(idl: string, standard?: string, url?: string): Promise<IdlMetadata> {
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
    const name = idlJson.name || idlJson.metadata?.name || 'custom-decoder';
    const kebabName = name
        .replace(/([a-z])([A-Z])/g, '$1-$2')
        .replace(/[\s_]+/g, '-')
        .toLowerCase();
    
    return {
        name: kebabName,
        version: idlJson.version || idlJson.metadata?.version,
        address: idlJson.metadata?.address || (idlSource.type === 'program' ? idl : undefined),
    };
}

/**
 * Generates a decoder from an IDL
 */
export async function generateDecoder(options: DecoderGenerationOptions): Promise<void> {
    const { idl, outputDir, standard: standardOpt, url, deleteFolderBeforeRendering = true } = options;
    
    const idlSource = parseIdlSource(idl);
    const standard = validateIdlStandard(standardOpt || 'anchor', idlSource);
    validateDecoderOptions(standard, idlSource, url, options.eventHints);

    if (idlSource.type === 'program') {
        let idlJson = await fetchAnchorIdl(idl, url!);
        
        if (hasLegacyEvents(idlJson)) {
            const info = getTransformationInfo(idlJson);
            console.log(`🔄 Detected ${info.eventCount} legacy events, transforming to modern format...`);
            console.log(`📝 Events: ${info.eventNames.join(', ')}`);
            idlJson = transformLegacyEvents(idlJson);
        }
        
        const codama = createFromRoot(rootNodeFromAnchor(idlJson));
        codama.accept(
            renderVisitor(outputDir, {
                deleteFolderBeforeRendering,
                anchorEvents: idlJson.events,
            }),
        );
        return;
    }

    // File-based IDL
    const idlPath = resolve(process.cwd(), idl);
    let idlJson = JSON.parse(readFileSync(idlPath, 'utf8'));

    if (hasLegacyEvents(idlJson)) {
        const info = getTransformationInfo(idlJson);
        idlJson = transformLegacyEvents(idlJson);
    }

    if (standard === 'anchor') {
        const codama = createFromRoot(rootNodeFromAnchor(idlJson));
        codama.accept(
            renderVisitor(outputDir, {
                deleteFolderBeforeRendering,
                anchorEvents: idlJson.events,
            }),
        );
    } else {
        const codama = createFromJson(JSON.stringify(idlJson));
        codama.accept(
            renderVisitor(outputDir, {
                deleteFolderBeforeRendering,
            }),
        );
    }
}

