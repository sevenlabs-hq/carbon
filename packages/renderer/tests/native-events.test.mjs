import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
    constantDiscriminatorNode,
    constantValueNodeFromBytes,
    definedTypeNode,
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
        const event = nativeEventNode({
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
            programWithEvents({
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
            programWithEvents({
                name: 'legacyPayments',
                publicKey: '11111111111111111111111111111111',
                events: [
                    nativeEventNode({
                        name: 'nativePaymentCreated',
                        data: structTypeNode([]),
                        discriminators: [
                            constantDiscriminatorNode(constantValueNodeFromBytes('base16', '01020304'), 0),
                            constantDiscriminatorNode(constantValueNodeFromBytes('base16', '0a'), 4),
                        ],
                    }),
                ],
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
        assert.doesNotMatch(cpiEvent, /NativePaymentCreated/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('keeps an explicit empty anchorEvents option as an event opt-out', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-no-events-'));

    try {
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                events: [
                    nativeEventNode({
                        name: 'paymentCreated',
                        data: structTypeNode([]),
                        discriminators: [
                            constantDiscriminatorNode(constantValueNodeFromBytes('base16', '01020304'), 0),
                            constantDiscriminatorNode(constantValueNodeFromBytes('base16', '09'), 4),
                        ],
                    }),
                ],
                instructions: [instructionNode({ name: 'createPayment' })],
            }),
        );

        visit(
            root,
            renderVisitor(outputDirectory, {
                anchorEvents: [],
                standalone: true,
                withGraphql: false,
                withPostgres: false,
            }),
        );

        assert.equal(existsSync(join(outputDirectory, 'src/instructions/cpi_event.rs')), false);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('ignores native events that do not describe an event-CPI payload', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-non-cpi-events-'));

    try {
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                events: [
                    nativeEventNode({
                        name: 'paymentCreated',
                        data: structTypeNode([]),
                        discriminators: [
                            constantDiscriminatorNode(constantValueNodeFromBytes('base16', '01020304'), 0),
                        ],
                    }),
                ],
                instructions: [instructionNode({ name: 'createPayment' })],
            }),
        );

        visit(
            root,
            renderVisitor(outputDirectory, {
                standalone: true,
                withGraphql: false,
                withPostgres: false,
            }),
        );

        assert.equal(existsSync(join(outputDirectory, 'src/instructions/cpi_event.rs')), false);
        assert.equal(existsSync(join(outputDirectory, 'src/instructions/create_payment.rs')), true);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

function nativeEventNode(input) {
    return { kind: 'eventNode', docs: [], ...input };
}

function programWithEvents(input) {
    const { events, ...programInput } = input;
    return { ...programNode(programInput), events };
}
