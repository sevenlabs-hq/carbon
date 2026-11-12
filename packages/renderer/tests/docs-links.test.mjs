import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { instructionNode, programNode, rootNode } from '@codama/nodes';
import { visit } from '@codama/visitors-core';

import { renderVisitor } from '../dist/index.mjs';

const PROGRAM_ID = '675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8';

function renderDecoder(options) {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-docs-links-'));
    try {
        const root = rootNode(
            programNode({
                name: 'raydiumAmmV4',
                publicKey: PROGRAM_ID,
                instructions: [instructionNode({ name: 'ping' })],
            }),
        );
        visit(
            root,
            renderVisitor(outputDirectory, {
                packageName: 'raydium-amm-v4',
                packageMetadata: { description: 'Raydium AMM V4 Decoder' },
                withGraphql: false,
                withPostgres: false,
                ...options,
            }),
        );
        return {
            cargoToml: readFileSync(join(outputDirectory, 'Cargo.toml'), 'utf8'),
            readme: readFileSync(join(outputDirectory, 'README.md'), 'utf8'),
        };
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
}

test('decoders in the Carbon workspace link to their docs page', () => {
    const { cargoToml, readme } = renderDecoder({ standalone: false, workspaceDeps: true });
    const docsUrl = 'https://carbon.sevenlabs.org/decoders/raydium-amm-v4';

    assert.match(cargoToml, new RegExp(`^homepage = "${docsUrl}"$`, 'm'));
    assert.match(readme, /^# Carbon Raydium AMM V4 Decoder$/m);
    assert.ok(readme.includes(docsUrl));
    assert.ok(readme.includes(PROGRAM_ID));
    assert.ok(readme.includes('cargo add carbon-raydium-amm-v4-decoder'));
});

test('decoders generated outside the Carbon workspace get no docs link', () => {
    const { cargoToml, readme } = renderDecoder({ standalone: true });

    assert.doesNotMatch(cargoToml, /homepage/);
    assert.equal(readme, '# Carbon Raydium AMM V4 Decoder\n');
});
