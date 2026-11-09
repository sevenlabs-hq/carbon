import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
    fieldDiscriminatorNode,
    instructionArgumentNode,
    instructionNode,
    numberTypeNode,
    numberValueNode,
    programNode,
    rootNode,
} from '@codama/nodes';

const packageDirectory = dirname(dirname(fileURLToPath(import.meta.url)));
const IDL_ADDRESS = 'VAU1T1111111111111111111111111111111111111';
const OTHER_ADDRESS = 'Stake11111111111111111111111111111111111111';
const ignored = /--program-id \S+ is ignored/;

function cli(...args) {
    return spawnSync(process.execPath, [join(packageDirectory, 'dist/cli.js'), ...args], {
        cwd: packageDirectory,
        encoding: 'utf8',
    });
}

function codamaIdl(directory, publicKey) {
    const path = join(directory, `codama-${publicKey || 'no-address'}.json`);
    const program = programNode({
        name: 'vault',
        publicKey,
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
    writeFileSync(path, JSON.stringify(rootNode(program)));
    return path;
}

function generatedProgramId(outDir) {
    const lib = readFileSync(join(outDir, 'src/lib.rs'), 'utf8');
    return lib.match(/from_str_const\("([^"]*)"\)/)[1];
}

test('a Codama IDL with program.publicKey generates without --program-id', () => {
    const root = mkdtempSync(join(tmpdir(), 'carbon-cli-program-id-'));

    try {
        const outDir = join(root, 'out');
        const result = cli('parse', '-i', codamaIdl(root, IDL_ADDRESS), '-o', outDir, '-s', 'codama');
        assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
        assert.equal(generatedProgramId(outDir), IDL_ADDRESS);
        assert.doesNotMatch(result.stdout, ignored);
    } finally {
        rmSync(root, { recursive: true, force: true });
    }
});

test('--program-id fills in a Codama IDL without an address', () => {
    const root = mkdtempSync(join(tmpdir(), 'carbon-cli-program-id-'));

    try {
        const idl = codamaIdl(root, '');
        const missing = cli('parse', '-i', idl, '-o', join(root, 'missing'), '-s', 'codama');
        assert.equal(missing.status, 2, `${missing.stdout}\n${missing.stderr}`);
        assert.match(missing.stdout + missing.stderr, /Program ID is required/);

        const outDir = join(root, 'out');
        const result = cli('parse', '-i', idl, '-o', outDir, '-s', 'codama', '--program-id', OTHER_ADDRESS);
        assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
        assert.equal(generatedProgramId(outDir), OTHER_ADDRESS);
    } finally {
        rmSync(root, { recursive: true, force: true });
    }
});

test("a different --program-id keeps the IDL's address and warns, for Codama and Anchor", () => {
    const root = mkdtempSync(join(tmpdir(), 'carbon-cli-program-id-'));

    try {
        const codamaOut = join(root, 'codama');
        const codama = cli(
            'parse',
            '-i',
            codamaIdl(root, IDL_ADDRESS),
            '-o',
            codamaOut,
            '-s',
            'codama',
            '--program-id',
            OTHER_ADDRESS,
        );
        assert.equal(codama.status, 0, `${codama.stdout}\n${codama.stderr}`);
        assert.equal(generatedProgramId(codamaOut), IDL_ADDRESS);
        assert.match(codama.stdout, ignored);

        // The Anchor fixture's address is 11111111111111111111111111111111.
        const anchorOut = join(root, 'anchor');
        const anchor = cli(
            'parse',
            '-i',
            join(packageDirectory, 'tests/fixtures/minimal-anchor.json'),
            '-o',
            anchorOut,
            '-s',
            'anchor',
            '--program-id',
            OTHER_ADDRESS,
        );
        assert.equal(anchor.status, 0, `${anchor.stdout}\n${anchor.stderr}`);
        assert.equal(generatedProgramId(anchorOut), '11111111111111111111111111111111');
        assert.match(anchor.stdout, ignored);
    } finally {
        rmSync(root, { recursive: true, force: true });
    }
});

test("a legacy Anchor IDL's top-level address reaches PROGRAM_ID", () => {
    // Codama's converter reads `metadata.address` from IDLs older than spec 0.1.0.
    const root = mkdtempSync(join(tmpdir(), 'carbon-cli-program-id-'));

    try {
        const idl = join(root, 'legacy.json');
        writeFileSync(
            idl,
            JSON.stringify({
                version: '0.1.0',
                name: 'legacy',
                instructions: [{ name: 'ping', accounts: [], args: [] }],
                address: IDL_ADDRESS,
            }),
        );
        const outDir = join(root, 'out');
        const result = cli('parse', '-i', idl, '-o', outDir, '-s', 'anchor');
        assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
        assert.equal(generatedProgramId(outDir), IDL_ADDRESS);
    } finally {
        rmSync(root, { recursive: true, force: true });
    }
});
