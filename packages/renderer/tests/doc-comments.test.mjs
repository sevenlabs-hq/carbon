import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
    definedTypeNode,
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
    } finally {
        rmSync(outputDirectory, { recursive: true, force: true });
    }
});
