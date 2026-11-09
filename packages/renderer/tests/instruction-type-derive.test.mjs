import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
    accountNode,
    fieldDiscriminatorNode,
    instructionArgumentNode,
    instructionNode,
    numberTypeNode,
    numberValueNode,
    programNode,
    rootNode,
    structFieldTypeNode,
    structTypeNode,
} from '@codama/nodes';
import { visit } from '@codama/visitors-core';

import { renderVisitor } from '../dist/index.mjs';

const INSTRUCTION_TYPE_DERIVE = '#[cfg_attr(feature = "serde", derive(carbon_core::InstructionType))]';

test('derives InstructionType on the instruction enum only, behind the serde feature', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-instruction-type-'));

    try {
        const program = programNode({
            name: 'vault',
            publicKey: '11111111111111111111111111111111',
            accounts: [
                accountNode({
                    name: 'config',
                    data: structTypeNode([
                        structFieldTypeNode({
                            name: 'discriminator',
                            type: numberTypeNode('u8'),
                            defaultValue: numberValueNode(2),
                            defaultValueStrategy: 'omitted',
                        }),
                        structFieldTypeNode({ name: 'rate', type: numberTypeNode('f64') }),
                    ]),
                    discriminators: [fieldDiscriminatorNode('discriminator', 0)],
                }),
            ],
            instructions: [
                instructionNode({
                    name: 'setRate',
                    arguments: [
                        instructionArgumentNode({
                            name: 'discriminator',
                            type: numberTypeNode('u8'),
                            defaultValue: numberValueNode(1),
                            defaultValueStrategy: 'omitted',
                        }),
                        instructionArgumentNode({ name: 'rate', type: numberTypeNode('f64') }),
                    ],
                    discriminators: [fieldDiscriminatorNode('discriminator', 0)],
                }),
            ],
        });

        visit(
            rootNode(program),
            renderVisitor(outputDirectory, {
                standalone: true,
                withGraphql: false,
                withPostgres: false,
            }),
        );

        const instructions = readFileSync(join(outputDirectory, 'src/instructions/mod.rs'), 'utf8');
        assert.ok(
            instructions.includes(`${INSTRUCTION_TYPE_DERIVE}\npub enum VaultInstruction {`),
            'the derive should sit directly on the instruction enum',
        );

        const accounts = readFileSync(join(outputDirectory, 'src/accounts/mod.rs'), 'utf8');
        assert.match(accounts, /pub enum VaultAccount \{/);
        assert.doesNotMatch(accounts, /InstructionType/);
    } finally {
        rmSync(outputDirectory, { recursive: true, force: true });
    }
});
