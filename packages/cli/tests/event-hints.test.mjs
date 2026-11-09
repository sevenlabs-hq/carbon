import assert from 'node:assert/strict';
import { mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, relative } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
    definedTypeNode,
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

const packageDirectory = dirname(dirname(fileURLToPath(import.meta.url)));
const programId = '11111111111111111111111111111111';
const warning = /--event-hints has no effect/;

function cli(...args) {
    return spawnSync(process.execPath, [join(packageDirectory, 'dist/cli.js'), ...args], {
        cwd: packageDirectory,
        encoding: 'utf8',
    });
}

function readTree(directory) {
    const files = {};
    const walk = current => {
        for (const name of readdirSync(current)) {
            const path = join(current, name);
            if (statSync(path).isDirectory()) walk(path);
            else files[relative(directory, path)] = readFileSync(path, 'utf8');
        }
    };
    walk(directory);
    return files;
}

test('--event-hints is not listed in help', () => {
    for (const args of [['--help'], ['parse', '--help'], ['scaffold', '--help']]) {
        const result = cli(...args);
        assert.equal(result.status, 0, result.stderr);
        assert.doesNotMatch(result.stdout, /event-hints/, `carbon-cli ${args.join(' ')}`);
    }
});

test('--event-hints is accepted with a warning and changes nothing', () => {
    const root = mkdtempSync(join(tmpdir(), 'carbon-cli-hints-'));

    try {
        // The hinted name is a defined type, so a hint that did anything would change the output.
        const idl = join(root, 'codama.json');
        const program = programNode({
            name: 'vault',
            publicKey: programId,
            definedTypes: [
                definedTypeNode({
                    name: 'buyEvent',
                    type: structTypeNode([structFieldTypeNode({ name: 'amount', type: numberTypeNode('u64') })]),
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
        writeFileSync(idl, JSON.stringify(rootNode(program)));

        const parse = (outDir, ...extra) =>
            cli(
                'parse',
                '--idl',
                idl,
                '--out-dir',
                join(root, outDir),
                '-s',
                'codama',
                '--program-id',
                programId,
                ...extra,
            );
        const plain = parse('plain');
        const hinted = parse('hinted', '--event-hints', 'BuyEvent');

        assert.equal(plain.status, 0, `${plain.stdout}\n${plain.stderr}`);
        assert.equal(hinted.status, 0, `${hinted.stdout}\n${hinted.stderr}`);
        assert.match(hinted.stdout, warning);
        assert.doesNotMatch(plain.stdout, warning);
        const generated = readTree(join(root, 'plain'));
        assert.ok(
            Object.keys(generated).some(path => path.endsWith('.rs')),
            'the decoder was generated',
        );
        assert.deepEqual(readTree(join(root, 'hinted')), generated);
    } finally {
        rmSync(root, { recursive: true, force: true });
    }
});

test('--event-hints with an Anchor IDL warns instead of failing', () => {
    const root = mkdtempSync(join(tmpdir(), 'carbon-cli-hints-'));

    try {
        const result = cli(
            'parse',
            '--idl',
            join(packageDirectory, 'tests/fixtures/minimal-anchor.json'),
            '--out-dir',
            join(root, 'out'),
            '-s',
            'anchor',
            '--event-hints',
            'BuyEvent',
        );
        assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
        assert.match(result.stdout, warning);
    } finally {
        rmSync(root, { recursive: true, force: true });
    }
});
