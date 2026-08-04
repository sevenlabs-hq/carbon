import { definedTypeNode, isNode, RootNode } from '@codama/nodes';

import { getDiscriminatorBytes } from './utils';

export type RenderEvent = {
    name: string;
    discriminator: number[];
};

export type NormalizedCodamaEvents = {
    root: RootNode;
    events: RenderEvent[];
    cpiDiscriminator: number[];
    eventDataOffset: number;
};

export type NormalizeCodamaEventsOptions = {
    eventCpiDiscriminator?: number[];
};

export const LEGACY_ANCHOR_EVENT_CPI_DISCRIMINATOR = [228, 69, 165, 46, 81, 203, 154, 29];

export function normalizeCodamaEvents(
    root: RootNode,
    options: NormalizeCodamaEventsOptions = {},
): NormalizedCodamaEvents | null {
    // Codama 1.3 preserves eventNode values from JSON even though its public
    // ProgramNode type does not expose them yet. Keep the existing dependency
    // versions and narrow the structural extension locally.
    const nativeEvents = (root.program as typeof root.program & { events?: CodamaEventNode[] }).events ?? [];
    if (nativeEvents.length === 0) {
        return null;
    }

    const shapes: EventShape[] = [];
    for (const event of nativeEvents) {
        const shape = classicEventShape(event) ?? hiddenPrefixEventShape(event);
        if (shape === null) {
            warn(`event "${event.name}" does not describe a supported event-CPI payload; skipping event decoding`);
            return null;
        }
        shapes.push(shape);
    }

    const explicitCpiDiscriminators = shapes.flatMap(shape =>
        shape.cpiDiscriminator === null ? [] : [shape.cpiDiscriminator],
    );
    for (let index = 1; index < explicitCpiDiscriminators.length; index += 1) {
        if (!equalBytes(explicitCpiDiscriminators[0], explicitCpiDiscriminators[index])) {
            warn('events declare conflicting event-CPI discriminators; skipping event decoding');
            return null;
        }
    }
    // The hidden-prefix shape carries no event-CPI envelope. An emit wrapper
    // instruction's dispatch discriminator can be a strict prefix of the bytes
    // the program writes on the wire (payment-channels dispatches on byte 228
    // but emits the full 8-byte Anchor tag), so instruction discriminators
    // must never define framing.
    const cpiDiscriminator =
        explicitCpiDiscriminators[0] ?? options.eventCpiDiscriminator ?? LEGACY_ANCHOR_EVENT_CPI_DISCRIMINATOR;

    const eventNames = new Set<string>();
    for (const event of nativeEvents) {
        if (eventNames.has(event.name)) {
            warn(`duplicate event name "${event.name}"; skipping event decoding`);
            return null;
        }
        eventNames.add(event.name);
    }

    const definedTypesByName = new Map(root.program.definedTypes.map(type => [type.name as string, type]));
    const eventTypes: ReturnType<typeof definedTypeNode>[] = [];
    for (const event of nativeEvents) {
        const payload = unwrapHiddenPrefix(event.data);
        const existing = definedTypesByName.get(event.name);
        if (existing !== undefined) {
            if (!sameShape(existing.type, payload)) {
                warn(
                    `event "${event.name}" collides with a defined type of a different shape; skipping event decoding`,
                );
                return null;
            }
            continue;
        }
        eventTypes.push(
            definedTypeNode({
                name: event.name,
                docs: event.docs,
                type: payload,
            }),
        );
    }
    const events = nativeEvents.map((event, index) => ({
        name: event.name,
        discriminator: shapes[index].discriminator,
    }));

    return {
        root: {
            ...root,
            program: {
                ...root.program,
                definedTypes: [...root.program.definedTypes, ...eventTypes],
                events: [],
            },
        } as RootNode,
        events,
        cpiDiscriminator,
        eventDataOffset: cpiDiscriminator.length,
    };
}

type EventShape = {
    cpiDiscriminator: number[] | null;
    discriminator: number[];
};

type CodamaEventNode = {
    name: RootNode['program']['name'];
    docs?: Parameters<typeof definedTypeNode>[0]['docs'];
    data: Parameters<typeof definedTypeNode>[0]['type'];
    discriminators?: Array<{
        kind: string;
        offset: number;
        constant: Parameters<typeof getDiscriminatorBytes>[0];
    }>;
};

function classicEventShape(event: CodamaEventNode): EventShape | null {
    const discriminators = event.discriminators ?? [];
    const constantDiscriminators = discriminators.filter(
        discriminator => discriminator.kind === 'constantDiscriminatorNode',
    );
    if (constantDiscriminators.length !== discriminators.length) {
        return null;
    }

    const sorted = [...constantDiscriminators].sort((left, right) => left.offset - right.offset);
    if (sorted.length < 2 || sorted[0].offset !== 0) {
        return null;
    }

    const eventCpiDiscriminator = tryGetDiscriminatorBytes(sorted[0].constant);
    if (eventCpiDiscriminator === null) {
        return null;
    }
    if (eventCpiDiscriminator.length === 0) {
        return null;
    }
    const eventDiscriminator: number[] = [];
    let expectedOffset = eventCpiDiscriminator.length;
    for (const discriminator of sorted.slice(1)) {
        if (discriminator.offset !== expectedOffset) {
            return null;
        }
        const bytes = tryGetDiscriminatorBytes(discriminator.constant);
        if (bytes === null) {
            return null;
        }
        eventDiscriminator.push(...bytes);
        expectedOffset += bytes.length;
    }
    if (eventDiscriminator.length === 0) {
        return null;
    }

    return {
        cpiDiscriminator: eventCpiDiscriminator,
        discriminator: eventDiscriminator,
    };
}

// Codama also encodes an event as a single discriminator at offset zero whose
// bytes repeat as a hiddenPrefixTypeNode constant on the data type. The
// event-CPI envelope is not part of the event node in that shape; it comes
// from the eventCpiDiscriminator renderer option or the legacy Anchor
// constant.
function hiddenPrefixEventShape(event: CodamaEventNode): EventShape | null {
    const discriminators = event.discriminators ?? [];
    if (discriminators.length !== 1) {
        return null;
    }
    const [discriminator] = discriminators;
    if (discriminator.kind !== 'constantDiscriminatorNode' || discriminator.offset !== 0) {
        return null;
    }
    const bytes = tryGetDiscriminatorBytes(discriminator.constant);
    if (bytes === null || bytes.length === 0) {
        return null;
    }

    const data = event.data;
    if (!isNode(data, 'hiddenPrefixTypeNode') || data.prefix.length !== 1) {
        return null;
    }
    const prefixBytes = tryGetDiscriminatorBytes(data.prefix[0]);
    if (prefixBytes === null || !equalBytes(bytes, prefixBytes)) {
        return null;
    }
    // The event page renders the payload as a struct body; other type kinds
    // would produce invalid Rust.
    if (!isNode(data.type, 'structTypeNode')) {
        return null;
    }

    return {
        cpiDiscriminator: null,
        discriminator: bytes,
    };
}

function unwrapHiddenPrefix(data: CodamaEventNode['data']): CodamaEventNode['data'] {
    return isNode(data, 'hiddenPrefixTypeNode') ? data.type : data;
}

// Key order in IDL JSON is not significant and docs never affect the wire
// layout, so neither may influence the comparison.
function sameShape(left: unknown, right: unknown): boolean {
    if (left === right) {
        return true;
    }
    if (Array.isArray(left) || Array.isArray(right)) {
        return (
            Array.isArray(left) &&
            Array.isArray(right) &&
            left.length === right.length &&
            left.every((item, index) => sameShape(item, right[index]))
        );
    }
    if (typeof left !== 'object' || typeof right !== 'object' || left === null || right === null) {
        return false;
    }
    const leftRecord = left as Record<string, unknown>;
    const rightRecord = right as Record<string, unknown>;
    const shapeKeys = (record: Record<string, unknown>) =>
        Object.keys(record)
            .filter(key => key !== 'docs' && record[key] !== undefined)
            .sort();
    const leftKeys = shapeKeys(leftRecord);
    const rightKeys = shapeKeys(rightRecord);
    return (
        leftKeys.length === rightKeys.length &&
        leftKeys.every((key, index) => key === rightKeys[index] && sameShape(leftRecord[key], rightRecord[key]))
    );
}

function warn(message: string): void {
    console.warn(`[carbon-codama-renderer] ${message}`);
}

function equalBytes(left: number[], right: number[]): boolean {
    return left.length === right.length && left.every((byte, index) => byte === right[index]);
}

function tryGetDiscriminatorBytes(constant: Parameters<typeof getDiscriminatorBytes>[0]): number[] | null {
    try {
        return getDiscriminatorBytes(constant);
    } catch {
        return null;
    }
}
