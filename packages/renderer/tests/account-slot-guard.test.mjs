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

test('account upserts skip an update from an older slot than the stored row', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-account-slot-guard-'));

    try {
        const discriminator = value =>
            structFieldTypeNode({
                name: 'discriminator',
                type: numberTypeNode('u8'),
                defaultValue: numberValueNode(value),
                defaultValueStrategy: 'omitted',
            });
        const program = programNode({
            name: 'vault',
            publicKey: '11111111111111111111111111111111',
            accounts: [
                accountNode({
                    name: 'config',
                    data: structTypeNode([
                        discriminator(9),
                        structFieldTypeNode({ name: 'rate', type: numberTypeNode('u64') }),
                    ]),
                    discriminators: [fieldDiscriminatorNode('discriminator', 0)],
                }),
            ],
            instructions: [
                instructionNode({
                    name: 'deposit',
                    arguments: [
                        instructionArgumentNode({
                            name: 'discriminator',
                            type: numberTypeNode('u8'),
                            defaultValue: numberValueNode(1),
                            defaultValueStrategy: 'omitted',
                        }),
                        instructionArgumentNode({ name: 'amount', type: numberTypeNode('u64') }),
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
                withPostgres: true,
                withSerde: true,
            }),
        );

        const account = readFileSync(join(outputDirectory, 'src/accounts/postgres/config_row.rs'), 'utf8');
        const upsert = account.slice(account.indexOf('Upsert for ConfigRow'), account.indexOf('Delete for ConfigRow'));
        assert.match(upsert, /INSERT INTO config_account AS stored \(/);
        assert.match(
            upsert,
            /__slot = EXCLUDED\.__slot\n\s+WHERE EXCLUDED\.__slot >= stored\.__slot\n\s+OR EXCLUDED\.__slot IS NULL OR stored\.__slot IS NULL\n/,
        );
        // The plain insert has no conflict clause to guard.
        assert.doesNotMatch(account.slice(0, account.indexOf('Upsert for ConfigRow')), /AS stored/);

        const instruction = readFileSync(join(outputDirectory, 'src/instructions/postgres/deposit_row.rs'), 'utf8');
        assert.doesNotMatch(instruction, /AS stored|stored\.__slot/);
    } finally {
        rmSync(outputDirectory, { recursive: true, force: true });
    }
});
