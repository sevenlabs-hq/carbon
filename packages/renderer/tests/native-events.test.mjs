import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
    constantDiscriminatorNode,
    constantValueNodeFromBytes,
    definedTypeNode,
    fieldDiscriminatorNode,
    hiddenPrefixTypeNode,
    instructionArgumentNode,
    instructionNode,
    numberTypeNode,
    numberValueNode,
    programNode,
    publicKeyTypeNode,
    rootNode,
    structFieldTypeNode,
    structTypeNode,
    tupleTypeNode,
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
        assert.match(cargoToml, /carbon-core = \{ version = "0\.12\.0"/);
        assert.match(cargoToml, /carbon-test-utils = "0\.12\.0"/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('serializes instruction metadata pubkeys as base58 when enabled', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-base58-program-id-'));

    try {
        const event = nativeEventNode({
            name: 'paymentCreated',
            data: structTypeNode([]),
            discriminators: [
                constantDiscriminatorNode(constantValueNodeFromBytes('base16', '01020304'), 0),
                constantDiscriminatorNode(constantValueNodeFromBytes('base16', '09'), 4),
            ],
        });
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                events: [event],
                instructions: [instructionNode({ name: 'createPayment' })],
            }),
        );

        visit(
            root,
            renderVisitor(outputDirectory, {
                standalone: true,
                withBase58: true,
                withGraphql: false,
                withPostgres: false,
                withSerde: true,
            }),
        );

        const instructionsMod = readFileSync(join(outputDirectory, 'src/instructions/mod.rs'), 'utf8');
        const programIdSerializationAttributes = instructionsMod.match(
            /serde\(serialize_with = "crate::base58::serialize"\)/g,
        );
        assert.equal(programIdSerializationAttributes?.length, 2);

        const cpiEvent = readFileSync(join(outputDirectory, 'src/instructions/cpi_event.rs'), 'utf8');
        const cpiAccountSerializationAttributes = cpiEvent.match(
            /serde\(serialize_with = "crate::base58::serialize"\)/g,
        );
        assert.equal(cpiAccountSerializationAttributes?.length, 2);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('frames hidden-prefix events with the full Anchor event-CPI envelope', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-hidden-prefix-events-'));

    try {
        const anchorEventCpiTag = [228, 69, 165, 46, 81, 203, 154, 29];
        const openedDiscriminator = [166, 172, 97, 9, 77, 76, 189, 109];
        const openedWireBytes = [
            ...anchorEventCpiTag,
            ...openedDiscriminator,
            ...new Array(32).fill(7),
            ...[42, 0, 0, 0, 0, 0, 0, 0],
        ];

        const eventTag = constantValueNodeFromBytes('base16', 'a6ac61094d4cbd6d');
        const event = nativeEventNode({
            name: 'opened',
            data: hiddenPrefixTypeNode(
                structTypeNode([
                    structFieldTypeNode({
                        name: 'channel',
                        type: publicKeyTypeNode(),
                    }),
                    structFieldTypeNode({
                        name: 'openSlot',
                        type: numberTypeNode('u64'),
                    }),
                ]),
                [eventTag],
            ),
            discriminators: [constantDiscriminatorNode(eventTag, 0)],
        });
        const emitInstruction = instructionNode({
            accounts: [
                {
                    isSigner: true,
                    isWritable: false,
                    kind: 'instructionAccountNode',
                    name: 'eventAuthority',
                },
            ],
            arguments: [
                instructionArgumentNode({
                    defaultValue: numberValueNode(228),
                    defaultValueStrategy: 'omitted',
                    name: 'discriminator',
                    type: numberTypeNode('u8'),
                }),
            ],
            discriminators: [fieldDiscriminatorNode('discriminator', 0)],
            name: 'emitEvent',
        });
        const regularInstruction = instructionNode({
            arguments: [
                instructionArgumentNode({
                    defaultValue: numberValueNode(1),
                    defaultValueStrategy: 'omitted',
                    name: 'discriminator',
                    type: numberTypeNode('u8'),
                }),
            ],
            discriminators: [fieldDiscriminatorNode('discriminator', 0)],
            name: 'createPayment',
        });
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                events: [event],
                instructions: [regularInstruction, emitInstruction],
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

        const envelope = openedWireBytes.slice(0, 8);
        const eventDiscriminator = openedWireBytes.slice(8, 16);

        const cpiEvent = readFileSync(join(outputDirectory, 'src/instructions/cpi_event.rs'), 'utf8');
        assert.match(cpiEvent, new RegExp(`if data\\.len\\(\\) < ${envelope.length}`));
        assert.match(cpiEvent, new RegExp(`if discriminator != \\[${envelope.join(', ')}\\]`));
        assert.match(cpiEvent, new RegExp(`let event_data = &data\\[${envelope.length}\\.\\.\\]`));

        const generatedEvent = readFileSync(join(outputDirectory, 'src/events/opened.rs'), 'utf8');
        assert.match(generatedEvent, /pub struct OpenedEvent/);
        assert.match(generatedEvent, new RegExp(`if discriminator != \\[${eventDiscriminator.join(', ')}\\]`));
        assert.match(generatedEvent, /pub channel: Pubkey/);
        assert.match(generatedEvent, /pub open_slot: u64/);

        assert.equal(existsSync(join(outputDirectory, 'src/instructions/emit_event.rs')), true);
        assert.equal(existsSync(join(outputDirectory, 'src/instructions/create_payment.rs')), true);

        const instructionsMod = readFileSync(join(outputDirectory, 'src/instructions/mod.rs'), 'utf8');
        const cpiEventArm = instructionsMod.indexOf('PaymentsInstruction::CpiEvent => CpiEvent');
        const emitEventArm = instructionsMod.indexOf('PaymentsInstruction::EmitEvent => EmitEvent');
        const createPaymentArm = instructionsMod.indexOf('PaymentsInstruction::CreatePayment => CreatePayment');
        assert.notEqual(cpiEventArm, -1);
        assert.notEqual(emitEventArm, -1);
        assert.notEqual(createPaymentArm, -1);
        assert.ok(cpiEventArm < emitEventArm);
        assert.ok(cpiEventArm < createPaymentArm);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('ignores hidden-prefix events whose payload is not a struct', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-hidden-prefix-tuple-'));

    try {
        const eventTag = constantValueNodeFromBytes('base16', 'a6ac61094d4cbd6d');
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                events: [
                    nativeEventNode({
                        name: 'tupleEvent',
                        data: hiddenPrefixTypeNode(tupleTypeNode([numberTypeNode('u32'), numberTypeNode('u64')]), [
                            eventTag,
                        ]),
                        discriminators: [constantDiscriminatorNode(eventTag, 0)],
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

test('lets the eventCpiDiscriminator option override the hidden-prefix envelope', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-envelope-option-'));

    try {
        const eventTag = constantValueNodeFromBytes('base16', 'd116b9d754a75450');
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                events: [
                    nativeEventNode({
                        name: 'payoutRedirected',
                        data: hiddenPrefixTypeNode(structTypeNode([]), [eventTag]),
                        discriminators: [constantDiscriminatorNode(eventTag, 0)],
                    }),
                ],
                instructions: [instructionNode({ name: 'createPayment' })],
            }),
        );

        visit(
            root,
            renderVisitor(outputDirectory, {
                eventCpiDiscriminator: [13, 37],
                standalone: true,
                withGraphql: false,
                withPostgres: false,
            }),
        );

        const cpiEvent = readFileSync(join(outputDirectory, 'src/instructions/cpi_event.rs'), 'utf8');
        assert.match(cpiEvent, /if data\.len\(\) < 2/);
        assert.match(cpiEvent, /if discriminator != \[13, 37\]/);
        assert.match(cpiEvent, /let event_data = &data\[2\.\.\]/);

        const generatedEvent = readFileSync(join(outputDirectory, 'src/events/payout_redirected.rs'), 'utf8');
        assert.match(generatedEvent, /if discriminator != \[209, 22, 185, 215, 84, 167, 84, 80\]/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('skips event decoding when an event collides with a defined type of a different shape', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-defined-type-clash-'));

    try {
        const eventTag = constantValueNodeFromBytes('base16', 'a6ac61094d4cbd6d');
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                definedTypes: [
                    definedTypeNode({
                        name: 'opened',
                        type: structTypeNode([
                            structFieldTypeNode({
                                name: 'amount',
                                type: numberTypeNode('u64'),
                            }),
                        ]),
                    }),
                ],
                events: [
                    nativeEventNode({
                        name: 'opened',
                        data: hiddenPrefixTypeNode(
                            structTypeNode([
                                structFieldTypeNode({
                                    name: 'slot',
                                    type: numberTypeNode('u64'),
                                }),
                            ]),
                            [eventTag],
                        ),
                        discriminators: [constantDiscriminatorNode(eventTag, 0)],
                    }),
                ],
                instructions: [instructionNode({ name: 'createPayment' })],
            }),
        );

        const warnings = [];
        const originalWarn = console.warn;
        console.warn = message => warnings.push(String(message));
        try {
            visit(
                root,
                renderVisitor(outputDirectory, {
                    standalone: true,
                    withGraphql: false,
                    withPostgres: false,
                }),
            );
        } finally {
            console.warn = originalWarn;
        }

        assert.equal(existsSync(join(outputDirectory, 'src/instructions/cpi_event.rs')), false);
        assert.equal(existsSync(join(outputDirectory, 'src/instructions/create_payment.rs')), true);
        assert.ok(warnings.some(warning => warning.includes('"opened"')));
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('reuses an existing defined type for a hidden-prefix event', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-hidden-prefix-defined-type-'));

    try {
        const eventTag = constantValueNodeFromBytes('base16', 'a6ac61094d4cbd6d');
        const fields = [
            structFieldTypeNode({
                name: 'amount',
                type: numberTypeNode('u64'),
            }),
        ];
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                definedTypes: [
                    definedTypeNode({
                        name: 'opened',
                        type: structTypeNode(fields),
                    }),
                ],
                events: [
                    nativeEventNode({
                        name: 'opened',
                        data: hiddenPrefixTypeNode(structTypeNode(fields), [eventTag]),
                        discriminators: [constantDiscriminatorNode(eventTag, 0)],
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

        const generatedEvent = readFileSync(join(outputDirectory, 'src/events/opened.rs'), 'utf8');
        assert.match(generatedEvent, /pub struct OpenedEvent/);
        assert.match(generatedEvent, /if discriminator != \[166, 172, 97, 9, 77, 76, 189, 109\]/);
        assert.match(generatedEvent, /pub amount: u64/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('reuses a defined type regardless of JSON key order and docs', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-defined-type-reorder-'));

    try {
        const eventTag = constantValueNodeFromBytes('base16', 'a6ac61094d4cbd6d');
        const buildPayload = () =>
            structTypeNode([
                structFieldTypeNode({
                    name: 'amount',
                    type: numberTypeNode('u64'),
                }),
            ]);
        const reorderedDefinedType = {
            ...reversedKeys(definedTypeNode({ name: 'opened', type: buildPayload() })),
            docs: ['pre-existing type docs'],
        };
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                definedTypes: [reorderedDefinedType],
                events: [
                    nativeEventNode({
                        name: 'opened',
                        data: hiddenPrefixTypeNode(buildPayload(), [eventTag]),
                        discriminators: [constantDiscriminatorNode(eventTag, 0)],
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

        assert.equal(existsSync(join(outputDirectory, 'src/instructions/cpi_event.rs')), true);
        const generatedEvent = readFileSync(join(outputDirectory, 'src/events/opened.rs'), 'utf8');
        assert.match(generatedEvent, /pub amount: u64/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('mixed classic and hidden-prefix events share the classic CPI discriminator', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-mixed-shape-events-'));

    try {
        const hiddenTag = constantValueNodeFromBytes('base16', 'd116b9d754a75450');
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
                    nativeEventNode({
                        name: 'payoutRedirected',
                        data: hiddenPrefixTypeNode(structTypeNode([]), [hiddenTag]),
                        discriminators: [constantDiscriminatorNode(hiddenTag, 0)],
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

        const cpiEvent = readFileSync(join(outputDirectory, 'src/instructions/cpi_event.rs'), 'utf8');
        assert.match(cpiEvent, /if discriminator != \[1, 2, 3, 4\]/);
        assert.match(cpiEvent, /let event_data = &data\[4\.\.\]/);
        assert.match(cpiEvent, /PaymentCreated/);
        assert.match(cpiEvent, /PayoutRedirected/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('defaults hidden-prefix events to the legacy Anchor CPI discriminator', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-hidden-prefix-legacy-'));

    try {
        const eventTag = constantValueNodeFromBytes('base16', 'd116b9d754a75450');
        const root = rootNode(
            programWithEvents({
                name: 'payments',
                publicKey: '11111111111111111111111111111111',
                events: [
                    nativeEventNode({
                        name: 'payoutRedirected',
                        data: hiddenPrefixTypeNode(structTypeNode([]), [eventTag]),
                        discriminators: [constantDiscriminatorNode(eventTag, 0)],
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

        const cpiEvent = readFileSync(join(outputDirectory, 'src/instructions/cpi_event.rs'), 'utf8');
        assert.match(cpiEvent, /if discriminator != \[228, 69, 165, 46, 81, 203, 154, 29\]/);
        assert.match(cpiEvent, /let event_data = &data\[8\.\.\]/);

        const generatedEvent = readFileSync(join(outputDirectory, 'src/events/payout_redirected.rs'), 'utf8');
        assert.match(generatedEvent, /if discriminator != \[209, 22, 185, 215, 84, 167, 84, 80\]/);
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

function reversedKeys(value) {
    if (Array.isArray(value)) {
        return value.map(reversedKeys);
    }
    if (value !== null && typeof value === 'object') {
        return Object.fromEntries(
            Object.keys(value)
                .reverse()
                .map(key => [key, reversedKeys(value[key])]),
        );
    }
    return value;
}

function programWithEvents(input) {
    const { events, ...programInput } = input;
    return { ...programNode(programInput), events };
}
