import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
    fieldDiscriminatorNode,
    instructionArgumentNode,
    instructionNode,
    numberTypeNode,
    numberValueNode,
    programNode,
    rootNode,
} from '@codama/nodes';
import { visit } from '@codama/visitors-core';

import { renderVisitor } from '../dist/index.mjs';

test('skips every discriminator byte before reading instruction fields', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-instruction-discriminators-'));

    try {
        const program = programNode({
            name: 'nestedDiscriminators',
            publicKey: '11111111111111111111111111111111',
            instructions: [
                instructionNode({
                    name: 'transferCheckedWithFee',
                    arguments: [
                        instructionArgumentNode({
                            name: 'discriminator',
                            type: numberTypeNode('u8'),
                            defaultValue: numberValueNode(26),
                            defaultValueStrategy: 'omitted',
                        }),
                        instructionArgumentNode({
                            name: 'transferFeeDiscriminator',
                            type: numberTypeNode('u8'),
                            defaultValue: numberValueNode(1),
                            defaultValueStrategy: 'omitted',
                        }),
                        instructionArgumentNode({ name: 'amount', type: numberTypeNode('u64') }),
                    ],
                    discriminators: [
                        fieldDiscriminatorNode('discriminator', 0),
                        fieldDiscriminatorNode('transferFeeDiscriminator', 1),
                    ],
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

        const source = readFileSync(join(outputDirectory, 'src/instructions/transfer_checked_with_fee.rs'), 'utf8');
        assert.match(source, /if transfer_fee_discriminator != 1 \{/);
        assert.match(source, /data_slice = &data_slice\[2\.\.\];/);
    } finally {
        rmSync(outputDirectory, { recursive: true, force: true });
    }
});
