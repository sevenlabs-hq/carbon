import { getDefinedTypeHistogramVisitor, getRecordLinkablesVisitor } from '@codama/visitors';
import { LinkableDictionary, visit } from '@codama/visitors-core';
import { definedTypeLinkNode, isNode } from '@codama/nodes';
import { createFromRoot } from 'codama';
import { rootNodeFromAnchorWithoutDefaultVisitor } from '@codama/nodes-from-anchor';

export function hasLegacyEvents(idlJson: any): boolean {
    if (!idlJson.events || !Array.isArray(idlJson.events) || idlJson.events.length === 0) {
        return false;
    }

    return idlJson.events.some((event: any) => event.fields && Array.isArray(event.fields));
}

function generateDiscriminator(eventName: string, index: number): number[] {
    const cpiEventsDiscriminator = [228, 69, 165, 46, 81, 203, 154, 29];

    const nameBytes = Buffer.from(eventName, 'utf8');
    const eventSpecific: number[] = [];

    for (let i = 0; i < 8; i++) {
        if (i < nameBytes.length) {
            eventSpecific.push(nameBytes[i]);
        } else {
            eventSpecific.push((nameBytes.length + index + i) % 256);
        }
    }

    return [...cpiEventsDiscriminator, ...eventSpecific];
}

function convertEventToType(event: any): any {
    return {
        name: event.name,
        type: {
            kind: 'struct',
            fields: event.fields.map((field: any) => ({
                name: field.name,
                type: field.type,
            })),
        },
    };
}

function convertEventToModern(event: any, index: number): any {
    return {
        name: event.name,
        discriminator: generateDiscriminator(event.name, index),
    };
}

export function transformLegacyEvents(idlJson: any): any {
    if (!hasLegacyEvents(idlJson)) {
        return idlJson;
    }

    const transformedIdl = JSON.parse(JSON.stringify(idlJson));

    const convertedEventTypes = transformedIdl.events.map((event: any) => convertEventToType(event));

    if (!transformedIdl.types) {
        transformedIdl.types = [];
    }
    transformedIdl.types = [...transformedIdl.types, ...convertedEventTypes];

    const modernEvents = transformedIdl.events.map((event: any, index: number) => convertEventToModern(event, index));

    transformedIdl.events = modernEvents;

    return transformedIdl;
}

export function getTransformationInfo(idlJson: any): { hasLegacy: boolean; eventCount: number; eventNames: string[] } {
    const hasLegacy = hasLegacyEvents(idlJson);
    const eventCount = hasLegacy ? idlJson.events.length : 0;
    const eventNames = hasLegacy ? idlJson.events.map((e: any) => e.name) : [];

    return { hasLegacy, eventCount, eventNames };
}

/**
 * Fixes PDA seed argument paths for IDL v0.1 by adding "params." prefix
 * when the path doesn't already have it and the argument is nested in params
 */
export function fixPdaSeedArgumentPaths(idlJson: any): any {
    // Check if this is IDL v0.1
    if (!idlJson.metadata || idlJson.metadata.spec !== '0.1.0') {
        return idlJson;
    }

    const transformedIdl = JSON.parse(JSON.stringify(idlJson));

    // Process each instruction
    for (const instruction of transformedIdl.instructions || []) {
        // Check if instruction has accounts with PDA seeds
        for (const account of instruction.accounts || []) {
            if (account.pda && account.pda.seeds) {
                // Process each seed in the PDA
                for (const seed of account.pda.seeds) {
                    if (seed.kind === 'arg' && seed.path && !seed.path.startsWith('params.')) {
                        // Add "params." prefix to the path
                        seed.path = `params.${seed.path}`;
                    }
                }
            }
        }
    }

    return transformedIdl;
}

/**
 * Removes "kind": "account" problematic PDA references where target accounts lack address fields.
 * Fixes: "CodamaError: Program ID kind [account] is not implemented."
 */
export function fixPdaSeedAccountReferences(idlJson: any): any {
    if (!idlJson.metadata || idlJson.metadata.spec !== '0.1.0') {
        return idlJson;
    }

    const transformedIdl = JSON.parse(JSON.stringify(idlJson));

    for (const instruction of transformedIdl.instructions || []) {
        const accountNames = (instruction.accounts || []).map((acc: any) => acc.name);

        for (const account of instruction.accounts || []) {
            if (account.pda) {
                if (account.pda.seeds) {
                    account.pda.seeds = account.pda.seeds.filter((seed: any) => {
                        if (seed.kind === 'account' && accountNames.includes(seed.path)) {
                            return false;
                        }
                        return true;
                    });
                }

                if (account.pda.program && account.pda.program.kind === 'account') {
                    delete account.pda.program;
                }
            }
        }
    }

    return transformedIdl;
}

/**
 * Detects if an IDL has nested instruction arguments that need preservation
 * Uses Codama's exact histogram logic to determine when types will be inlined/flattened
 */
export function hasNestedInstructionArguments(idlJson: any): boolean {
    if (!idlJson.instructions || !Array.isArray(idlJson.instructions)) {
        return false;
    }

    // Convert IDL to Codama root node
    const rawRoot = rootNodeFromAnchorWithoutDefaultVisitor(idlJson);
    const root = createFromRoot(rawRoot);

    // Get the histogram (same as unwrapInstructionArgsDefinedTypesVisitor)
    const histogram = visit(rawRoot, getDefinedTypeHistogramVisitor());
    const linkables = new LinkableDictionary();
    visit(rawRoot, getRecordLinkablesVisitor(linkables));

    // Check if any types would be inlined (exact same logic as Codama)
    const definedTypesToInline = Object.keys(histogram)
        .filter(key => {
            const entry = histogram[key as keyof typeof histogram];
            return (entry?.total ?? 0) === 1 && (entry?.directlyAsInstructionArgs ?? 0) === 1;
        })
        .filter(key => {
            const names = key.split('.');
            const link = names.length == 2 ? definedTypeLinkNode(names[1], names[0]) : definedTypeLinkNode(key);
            const found = linkables.get([link]);
            return found && !isNode(found.type, 'enumTypeNode');
        });

    return definedTypesToInline.length > 0;
}
