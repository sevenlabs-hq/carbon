import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
    accountNode,
    constantDiscriminatorNode,
    constantValueNodeFromBytes,
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

test('keys instruction and CPI event rows by absolute path, with a follow-up migration', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-absolute-path-key-'));

    try {
        const program = {
            ...programNode({
                name: 'vault',
                publicKey: '11111111111111111111111111111111',
                accounts: [
                    accountNode({
                        name: 'config',
                        data: structTypeNode([
                            structFieldTypeNode({
                                name: 'discriminator',
                                type: numberTypeNode('u8'),
                                defaultValue: numberValueNode(9),
                                defaultValueStrategy: 'omitted',
                            }),
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
            }),
            events: [
                {
                    kind: 'eventNode',
                    docs: [],
                    name: 'depositMade',
                    data: structTypeNode([structFieldTypeNode({ name: 'amount', type: numberTypeNode('u64') })]),
                    discriminators: [
                        constantDiscriminatorNode(constantValueNodeFromBytes('base16', '01020304'), 0),
                        constantDiscriminatorNode(constantValueNodeFromBytes('base16', '09'), 4),
                    ],
                },
            ],
        };

        visit(
            rootNode(program),
            renderVisitor(outputDirectory, {
                standalone: true,
                withGraphql: false,
                withPostgres: true,
                withSerde: true,
            }),
        );

        const read = path => readFileSync(join(outputDirectory, path), 'utf8');

        for (const [path, table] of [
            ['src/instructions/postgres/deposit_row.rs', 'deposit_instruction'],
            ['src/instructions/postgres/cpi_event_row.rs', 'cpi_events'],
        ]) {
            const row = read(path);
            assert.match(row, /ON CONFLICT \(\s*__signature, __absolute_path\s*\)/, `${table} upserts on the path`);
            assert.match(row, /type Key = \(String, Vec<i16>\);/, `${table} keys deletes and lookups by path`);
            assert.match(row, /__signature = \$1 AND __absolute_path = \$2/);
            assert.match(row, /\.bind\(&self\.instruction_metadata\.absolute_path\)/);
            assert.doesNotMatch(row, /__instruction_index = \$2/);
            // The original CREATE TABLE is history; the follow-up migration changes the key.
            assert.match(row, /PRIMARY KEY \(__signature, __instruction_index, __stack_height\)/);
        }

        const migrations = read('src/instructions/postgres/mod.rs');
        assert.match(
            migrations,
            /fn run_before\(&self\)[^}]*vec!\[Box::new\(VaultInstructionsAbsolutePathMigration\)\]/,
            'the existing migration pulls in the follow-up one',
        );
        assert.match(migrations, /"vault_instructions_absolute_path"/);
        assert.match(migrations, /AbsolutePathKeyMigrationOperation\("deposit_instruction"\)/);
        assert.match(migrations, /AbsolutePathKeyMigrationOperation\("cpi_events"\)/);
        assert.match(migrations, /vec!\[Box::new\(VaultInstructionsMigration\)\]/, 'the follow-up runs after it');

        assert.doesNotMatch(read('src/accounts/postgres/config_row.rs'), /__absolute_path/);
    } finally {
        rmSync(outputDirectory, { recursive: true, force: true });
    }
});
