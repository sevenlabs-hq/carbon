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
    const nativeEvents = root.program.events ?? [];
    if (nativeEvents.length === 0) {
        return null;
    }

    const definedTypeNames = new Set(root.program.definedTypes.map(type => type.name));
    const eventTypes = nativeEvents.map(event => {
        if (definedTypeNames.has(event.name)) {
            throw new Error(`Event ${event.name} conflicts with a defined type of the same name`);
        }
        definedTypeNames.add(event.name);
        return definedTypeNode({
            name: event.name,
            docs: event.docs,
            type: event.data,
        });
    });

    const eventDiscriminators = nativeEvents.map(event => {
        const discriminators = event.discriminators ?? [];
        const constantDiscriminators = discriminators.filter(
            discriminator => discriminator.kind === 'constantDiscriminatorNode',
        );
        if (constantDiscriminators.length !== discriminators.length) {
            throw new Error(`Event ${event.name} must use constant discriminators`);
        }

        const sorted = [...constantDiscriminators].sort((left, right) => left.offset - right.offset);
        if (sorted.length < 2 || sorted[0].offset !== 0) {
            throw new Error(
                `Event ${event.name} must declare a CPI discriminator at offset 0 and an event discriminator after it`,
            );
        }

        const eventCpiDiscriminator = getDiscriminatorBytes(sorted[0].constant);
        if (eventCpiDiscriminator.length === 0) {
            throw new Error(`Event ${event.name} has an empty CPI discriminator`);
        }
        const eventDiscriminator: number[] = [];
        let expectedOffset = eventCpiDiscriminator.length;
        for (const discriminator of sorted.slice(1)) {
            if (discriminator.offset !== expectedOffset) {
                throw new Error(
                    `Event ${event.name} has a non-contiguous discriminator at offset ${discriminator.offset}`,
                );
            }
            const bytes = getDiscriminatorBytes(discriminator.constant);
            eventDiscriminator.push(...bytes);
            expectedOffset += bytes.length;
        }
        if (eventDiscriminator.length === 0) {
            throw new Error(`Event ${event.name} has an empty event discriminator`);
        }

        return {
            cpiDiscriminator: eventCpiDiscriminator,
            discriminator: eventDiscriminator,
        };
    });

    const cpiDiscriminator = eventDiscriminators[0].cpiDiscriminator;
    for (let index = 1; index < eventDiscriminators.length; index += 1) {
        if (!equalBytes(cpiDiscriminator, eventDiscriminators[index].cpiDiscriminator)) {
            throw new Error(`Event ${nativeEvents[index].name} uses a different CPI discriminator`);
        }
    }
    const events = nativeEvents.map((event, index) => ({
        name: event.name,
        discriminator: eventDiscriminators[index].discriminator,
    }));

    return {
        root: {
            ...root,
            program: {
                ...root.program,
                definedTypes: [...root.program.definedTypes, ...eventTypes],
                events: [],
            },
        },
        events,
        cpiDiscriminator,
        eventDataOffset: cpiDiscriminator.length,
    };
}

function equalBytes(left: number[], right: number[]): boolean {
    return left.length === right.length && left.every((byte, index) => byte === right[index]);
}
