import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
    definedTypeNode,
    enumEmptyVariantTypeNode,
    enumTypeNode,
    numberTypeNode,
    programNode,
    rootNode,
    structFieldTypeNode,
    structTypeNode,
} from '@codama/nodes';
import { visit } from '@codama/visitors-core';

import { renderVisitor } from '../dist/index.mjs';

test('closes a code span that an IDL doc closes with two backticks', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-doc-comments-'));

    try {
        const program = programNode({
            name: 'docComments',
            publicKey: '11111111111111111111111111111111',
            definedTypes: [
                definedTypeNode({
                    name: 'platformConfig',
                    type: structTypeNode([
                        structFieldTypeNode({
                            name: 'creatorFeeRate',
                            type: numberTypeNode('u64'),
                            docs: [
                                'Permissions such as `transfer_fee_config_authority`` and `withdraw_withheld_authority`.',
                                'A ``double`` span and a ``` fence stay as they are.',
                                'A stray ` then ``code`` here stays as it is.',
                                '```',
                                'let s = `x`` ;',
                                '```',
                            ],
                        }),
                    ]),
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

        const source = readFileSync(join(outputDirectory, 'src/types/platform_config.rs'), 'utf8');
        assert.match(
            source,
            /\/\/\/ Permissions such as `transfer_fee_config_authority` and `withdraw_withheld_authority`\./,
        );
        assert.match(source, /\/\/\/ A ``double`` span and a ``` fence stay as they are\./);
        assert.match(source, /\/\/\/ A stray ` then ``code`` here stays as it is\./);
        assert.match(source, /\/\/\/ let s = `x`` ;/);
    } finally {
        rmSync(outputDirectory, { recursive: true, force: true });
    }
});

test('closes the same code spans in GraphQL schema docs', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-doc-comments-graphql-'));

    try {
        const program = programNode({
            name: 'docComments',
            publicKey: '11111111111111111111111111111111',
            definedTypes: [
                definedTypeNode({
                    name: 'authorityKind',
                    docs: ['Picks `transfer_fee_config_authority`` or another authority.'],
                    type: enumTypeNode([
                        enumEmptyVariantTypeNode('transferFee'),
                        enumEmptyVariantTypeNode('withdrawWithheld'),
                    ]),
                }),
                definedTypeNode({
                    name: 'emptyMarker',
                    docs: ['Marks `withdraw_withheld_authority`` as unset.'],
                    type: structTypeNode([]),
                }),
            ],
        });

        visit(
            rootNode(program),
            renderVisitor(outputDirectory, {
                standalone: true,
                withGraphql: true,
                withPostgres: false,
            }),
        );

        const enumSchema = readFileSync(join(outputDirectory, 'src/types/graphql/authority_kind_schema.rs'), 'utf8');
        assert.match(enumSchema, /\/\/\/ Picks `transfer_fee_config_authority` or another authority\./);
        const emptySchema = readFileSync(join(outputDirectory, 'src/types/graphql/empty_marker_schema.rs'), 'utf8');
        assert.match(emptySchema, /\/\/\/ Marks `withdraw_withheld_authority` as unset\./);
    } finally {
        rmSync(outputDirectory, { recursive: true, force: true });
    }
});
