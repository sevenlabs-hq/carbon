import assert from 'node:assert/strict';
import { mkdtempSync, readdirSync, readFileSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, relative } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const packageDirectory = dirname(dirname(fileURLToPath(import.meta.url)));
const idl = join(packageDirectory, 'tests/fixtures/minimal-anchor.json');
const warning = /--as-crate has no effect/;

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

test('--as-crate is not listed in help', () => {
    const result = cli('parse', '--help');
    assert.equal(result.status, 0, result.stderr);
    assert.doesNotMatch(result.stdout, /as-crate|-c,/);
});

test('-c and --as-crate are accepted with a warning and change nothing', () => {
    const root = mkdtempSync(join(tmpdir(), 'carbon-cli-crate-'));

    try {
        const parse = (outDir, ...extra) =>
            cli('parse', '--idl', idl, '--out-dir', join(root, outDir), '-s', 'anchor', ...extra);
        const plain = parse('plain');
        assert.equal(plain.status, 0, `${plain.stdout}\n${plain.stderr}`);
        assert.doesNotMatch(plain.stdout, warning);

        const generated = readTree(join(root, 'plain'));
        assert.ok('Cargo.toml' in generated, 'the decoder is generated as a crate');

        for (const flag of ['-c', '--as-crate']) {
            const flagged = parse(flag, flag);
            assert.equal(flagged.status, 0, `${flagged.stdout}\n${flagged.stderr}`);
            assert.match(flagged.stdout, warning, flag);
            assert.deepEqual(readTree(join(root, flag)), generated, flag);
        }
    } finally {
        rmSync(root, { recursive: true, force: true });
    }
});
