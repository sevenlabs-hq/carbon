import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
    constantDiscriminatorNode,
    constantValueNodeFromBytes,
    definedTypeNode,
    eventNode,
    instructionNode,
    numberTypeNode,
    programNode,
    rootNode,
    structFieldTypeNode,
    structTypeNode,
} from '@codama/nodes';
import { visit } from '@codama/visitors-core';

import { renderVisitor } from '../dist/index.mjs';

test('renders native Codama events with their IDL-defined CPI discriminator', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-codama-events-'));

    try {
        const event = eventNode({
            name: 'paymentCreated',
            data: structTypeNode([
                structFieldTypeNode({
                    name: 'amount',
                    type: numberTypeNode('u64'),
                }),
            ]),
            discriminators: [
                constantDiscriminatorNode(constantValueNodeFromBytes('base16', '01020304'), 0),
                constantDiscriminatorNode(constantValueNodeFromBytes('base16', '09'), 4),
            ],
        });
        const instruction = instructionNode({
            name: 'createPayment',
            discriminators: [constantDiscriminatorNode(constantValueNodeFromBytes('base16', 'aabbccdd'), 0)],
        });
        const root = rootNode(
            programNode({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                events: [event],
                instructions: [instruction],
            }),
        );

        visit(
            root,
            renderVisitor(outputDirectory, {
                standalone: true,
                withGraphql: false,
                withPostgres: false,
                withSerde: true,
            }),
        );

        const cpiEvent = readFileSync(join(outputDirectory, 'src/instructions/cpi_event.rs'), 'utf8');
        assert.match(cpiEvent, /if data\.len\(\) < 4/);
        assert.match(cpiEvent, /if discriminator != \[1, 2, 3, 4\]/);
        assert.match(cpiEvent, /let event_data = &data\[4\.\.\]/);

        const generatedEvent = readFileSync(join(outputDirectory, 'src/events/payment_created.rs'), 'utf8');
        assert.match(generatedEvent, /pub struct PaymentCreatedEvent/);
        assert.match(generatedEvent, /if discriminator != \[9\]/);

        const cargoToml = readFileSync(join(outputDirectory, 'Cargo.toml'), 'utf8');
        assert.match(cargoToml, /carbon-core = \{ version = "1\.0\.0"/);
        assert.match(cargoToml, /carbon-test-utils = "1\.0\.0"/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('keeps the anchorEvents renderer option backward compatible', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-anchor-events-'));

    try {
        const root = rootNode(
            programNode({
                name: 'legacyPayments',
                publicKey: '11111111111111111111111111111111',
                definedTypes: [
                    definedTypeNode({
                        name: 'paymentCreated',
                        type: structTypeNode([]),
                    }),
                ],
                instructions: [instructionNode({ name: 'createPayment' })],
            }),
        );

        visit(
            root,
            renderVisitor(outputDirectory, {
                anchorEvents: [{ name: 'paymentCreated', discriminator: [9] }],
                standalone: true,
                withGraphql: false,
                withPostgres: false,
            }),
        );

        const cpiEvent = readFileSync(join(outputDirectory, 'src/instructions/cpi_event.rs'), 'utf8');
        assert.match(cpiEvent, /if data\.len\(\) < 8/);
        assert.match(cpiEvent, /if discriminator != \[228, 69, 165, 46, 81, 203, 154, 29\]/);
        assert.match(cpiEvent, /let event_data = &data\[8\.\.\]/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});
