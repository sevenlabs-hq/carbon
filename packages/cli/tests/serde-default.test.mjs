import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const packageDirectory = dirname(dirname(fileURLToPath(import.meta.url)));
const idl = join(packageDirectory, 'tests/fixtures/minimal-anchor.json');

function cli(...args) {
    return spawnSync(process.execPath, [join(packageDirectory, 'dist/cli.js'), ...args], {
        cwd: packageDirectory,
        encoding: 'utf8',
    });
}

// The decoder's Cargo.toml after `parse` with the given flags.
function parsedCargoToml(root, name, ...flags) {
    const outDir = join(root, name);
    const result = cli('parse', '--idl', idl, '--out-dir', outDir, '-s', 'anchor', ...flags);
    assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
    return readFileSync(join(outDir, 'Cargo.toml'), 'utf8');
}

test('--help gives the --with-serde default that the CLI applies', () => {
    for (const command of ['parse', 'scaffold']) {
        const result = cli(command, '--help');
        assert.equal(result.status, 0, result.stderr);
        // Commander wraps long descriptions, so compare with whitespace collapsed.
        assert.match(
            result.stdout.replace(/\s+/g, ' '),
            /--with-serde <boolean> Include serde feature for decoder \(default: true; --with-postgres, --with-graphql and --with-base58 also turn it on\)/,
            command,
        );
    }
});

test('serde is on by default, and Postgres, GraphQL and base58 turn it on too', () => {
    const root = mkdtempSync(join(tmpdir(), 'carbon-cli-serde-'));
    const allOff = ['--with-postgres', 'false', '--with-graphql', 'false', '--with-base58', 'false'];

    try {
        // Default: with the other three off, --with-serde itself defaults to true.
        assert.match(parsedCargoToml(root, 'default', ...allOff), /^serde = \[/m);
        // The flag turns it off only when nothing else needs it.
        // (`serde_json` is always a dependency; the serde feature and dependency are `serde = …` lines.)
        assert.doesNotMatch(parsedCargoToml(root, 'off', ...allOff, '--with-serde', 'false'), /^serde = /m);
        // Postgres keeps it on whatever --with-serde says.
        const postgres = parsedCargoToml(
            root,
            'postgres',
            '--with-postgres',
            'true',
            '--with-graphql',
            'false',
            '--with-serde',
            'false',
        );
        assert.match(postgres, /^postgres = \[[^\]]*"serde"/m);
    } finally {
        rmSync(root, { recursive: true, force: true });
    }
});
