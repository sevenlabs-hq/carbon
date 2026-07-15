import { definedTypeNode, RootNode } from '@codama/nodes';
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

export const ANCHOR_EVENT_CPI_DISCRIMINATOR = [228, 69, 165, 46, 81, 203, 154, 29];

export function normalizeCodamaEvents(root: RootNode): NormalizedCodamaEvents | null {
    // Codama 1.3 preserves eventNode values from JSON even though its public
    // ProgramNode type does not expose them yet. Keep the existing dependency
    // versions and narrow the structural extension locally.
    const nativeEvents = (root.program as typeof root.program & { events?: CodamaEventNode[] }).events ?? [];
    if (nativeEvents.length === 0) {
        return null;
    }

    const eventDiscriminators = nativeEvents.map(event => {
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
    });
    if (eventDiscriminators.some(discriminator => discriminator === null)) {
        return null;
    }
    const validEventDiscriminators = eventDiscriminators.filter(discriminator => discriminator !== null);

    const cpiDiscriminator = validEventDiscriminators[0].cpiDiscriminator;
    for (let index = 1; index < validEventDiscriminators.length; index += 1) {
        if (!equalBytes(cpiDiscriminator, validEventDiscriminators[index].cpiDiscriminator)) {
            return null;
        }
    }

    const eventNames = new Set<string>();
    for (const event of nativeEvents) {
        if (eventNames.has(event.name)) {
            return null;
        }
        eventNames.add(event.name);
    }

    const definedTypeNames = new Set(root.program.definedTypes.map(type => type.name));
    const eventTypes = nativeEvents.flatMap(event => {
        if (definedTypeNames.has(event.name)) {
            return [];
        }
        definedTypeNames.add(event.name);
        return [
            definedTypeNode({
                name: event.name,
                docs: event.docs,
                type: event.data,
            }),
        ];
    });
    const events = nativeEvents.map((event, index) => ({
        name: event.name,
        discriminator: validEventDiscriminators[index].discriminator,
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
