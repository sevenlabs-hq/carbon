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
            kind: "struct",
            fields: event.fields.map((field: any) => ({
                name: field.name,
                type: field.type
            }))
        }
    };
}

function convertEventToModern(event: any, index: number): any {
    return {
        name: event.name,
        discriminator: generateDiscriminator(event.name, index)
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
    
    const modernEvents = transformedIdl.events.map((event: any, index: number) => 
        convertEventToModern(event, index)
    );
    
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
 * Detects if IDL has nested instruction arguments that should be preserved
 */
export function hasNestedInstructionArguments(idlJson: any): boolean {
    if (!idlJson.instructions || !Array.isArray(idlJson.instructions)) {
        return false;
    }
    
    for (const instruction of idlJson.instructions) {
        if (!instruction.args || !Array.isArray(instruction.args)) {
            continue;
        }
        
        for (const arg of instruction.args) {
            // Check for nested struct arguments (defined type references)
            if (arg.type?.defined) {
                // Check if it's a nested structure (not a primitive type)
                if (typeof arg.type.defined === 'object' && arg.type.defined.name) {
                    return true; // Found object-style defined type
                }
                if (typeof arg.type.defined === 'string') {
                    // Check if this defined type exists in types section
                    const typeExists = idlJson.types?.some((t: any) => t.name === arg.type.defined);
                    if (typeExists) {
                        return true; // Found string-style defined type that exists
                    }
                }
            }
        }
    }
    
    return false;
}

/**
 * Normalizes IDL for nested structure by fixing PDA seed argument paths
 * and converting account field names to match expected format
 */
import { getDefinedTypeHistogramVisitor, getRecordLinkablesVisitor } from '@codama/visitors';
import { LinkableDictionary, visit } from '@codama/visitors-core';
import { definedTypeLinkNode, isNode } from '@codama/nodes';
import { createFromRoot } from 'codama';
import { rootNodeFromAnchorWithoutDefaultVisitor } from '@codama/nodes-from-anchor';

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

