import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
    definedTypeNode,
    enumEmptyVariantTypeNode,
    enumStructVariantTypeNode,
    enumTypeNode,
    pascalCase,
    programNode,
    publicKeyTypeNode,
    rootNode,
    snakeCase,
    structFieldTypeNode,
    structTypeNode,
} from '@codama/nodes';
import { visit } from '@codama/visitors-core';

import { renderVisitor } from '../dist/index.mjs';

test('renders one well-formed Token-2022 Extension implementation', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-token-2022-extension-'));

    try {
        const nullableVariants = [
            ['transferFeeConfig', ['transferFeeConfigAuthority', 'withdrawWithheldAuthority']],
            ['mintCloseAuthority', ['closeAuthority']],
            ['confidentialTransferMint', ['authority', 'auditorElgamalPubkey']],
            ['interestBearingConfig', ['rateAuthority']],
            ['permanentDelegate', ['delegate']],
            ['transferHook', ['authority', 'programId']],
            ['confidentialTransferFee', ['authority']],
            ['metadataPointer', ['authority', 'metadataAddress']],
            ['tokenMetadata', ['updateAuthority']],
            ['groupPointer', ['authority', 'groupAddress']],
            ['tokenGroup', ['updateAuthority']],
            ['groupMemberPointer', ['authority', 'memberAddress']],
            ['scaledUiAmountConfig', ['authority']],
            ['pausableConfig', ['authority']],
            ['permissionedBurn', ['authority']],
        ];
        const extension = {
            ...definedTypeNode({
                name: 'extension',
                type: enumTypeNode([
                    enumEmptyVariantTypeNode('uninitialized'),
                    ...nullableVariants.map(([variant, fields]) =>
                        enumStructVariantTypeNode(
                            variant,
                            structTypeNode(
                                fields.map(name => structFieldTypeNode({ name, type: publicKeyTypeNode() })),
                            ),
                        ),
                    ),
                ]),
            }),
            name: 'Extension',
        };
        const program = {
            ...programNode({
                name: 'token2022',
                publicKey: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb',
                definedTypes: [extension],
            }),
            name: 'token-2022',
        };

        visit(
            rootNode(program),
            renderVisitor(outputDirectory, {
                standalone: true,
                withGraphql: false,
                withPostgres: false,
            }),
        );

        const source = readFileSync(join(outputDirectory, 'src/types/extension.rs'), 'utf8');
        const implIndex = source.indexOf('impl Extension {');

        assert.notEqual(implIndex, -1);
        assert.doesNotMatch(source.slice(0, implIndex), /pub fn from_mint_and_type/);
        assert.equal(source.match(/impl Extension \{/g)?.length, 1);
        assert.equal(source.match(/pub fn from_mint_and_type/g)?.length, 1);
        assert.equal(source.match(/pub fn from_account_and_type/g)?.length, 1);
        assert.doesNotMatch(source, /\.get\(\)\.unwrap_or_default\(\)/);
        for (const [variant, fields] of nullableVariants) {
            const variantStart = source.indexOf(`${pascalCase(variant)} {`);
            const variantEnd = source.indexOf('},', variantStart);
            assert.notEqual(variantStart, -1);
            assert.notEqual(variantEnd, -1);

            const renderedVariant = source.slice(variantStart, variantEnd);
            for (const field of fields) {
                assert.match(renderedVariant, new RegExp(`${snakeCase(field)}: Option<Pubkey>,`));
            }
        }
        assert.match(source, /ExtensionType::PermissionedBurn =>/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});
