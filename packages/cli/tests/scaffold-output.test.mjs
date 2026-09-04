import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const packageDirectory = dirname(dirname(fileURLToPath(import.meta.url)));

test('scaffolds a Carbon 2 Transaction V1-ready RPC project', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-cli-v2-scaffold-'));

    try {
        const result = spawnSync(
            process.execPath,
            [
                join(packageDirectory, 'dist/cli.js'),
                'scaffold',
                '--name',
                'release-smoke',
                '--out-dir',
                outputDirectory,
                '--decoder',
                'smoke',
                '--idl',
                join(packageDirectory, 'tests/fixtures/minimal-anchor.json'),
                '--idl-standard',
                'anchor',
                '--data-source',
                'rpc-block-subscribe',
                '--metrics',
                'log',
                '--with-postgres',
                'false',
                '--with-graphql',
                'false',
                '--with-serde',
                'true',
            ],
            { cwd: packageDirectory, encoding: 'utf8' },
        );

        assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);

        const projectDirectory = join(outputDirectory, 'release-smoke');
        const cargoToml = readFileSync(join(projectDirectory, 'indexer/Cargo.toml'), 'utf8');
        const decoderCargoToml = readFileSync(join(projectDirectory, 'decoder/Cargo.toml'), 'utf8');
        const main = readFileSync(join(projectDirectory, 'indexer/src/main.rs'), 'utf8');

        const cargoFmt = spawnSync(
            'cargo',
            ['fmt', '--all', '--check', '--manifest-path', join(projectDirectory, 'Cargo.toml')],
            { cwd: projectDirectory, encoding: 'utf8' },
        );
        assert.equal(cargoFmt.status, 0, `${cargoFmt.stdout}\n${cargoFmt.stderr}`);

        assert.match(cargoToml, /rust-version = "1\.96\.1"/);
        assert.match(decoderCargoToml, /rust-version = "1\.96\.1"/);
        assert.match(cargoToml, /carbon-core = \{ version = "2\.0\.0", default-features = false \}/);
        assert.doesNotMatch(cargoToml, /features = \["postgres", "graphql"\]/);
        assert.match(cargoToml, /carbon-rpc-block-subscribe-datasource = "2\.0\.0"/);
        assert.match(cargoToml, /solana-client = "=4\.2\.2"/);
        assert.match(
            cargoToml,
            /solana-transaction-status = \{ version = "=4\.2\.2", features = \["agave-unstable-api"\] \}/,
        );
        assert.match(cargoToml, /log = "0\.4\.28"/);

        assert.match(main, /encoding: Some\(UiTransactionEncoding::Base64\)/);
        assert.match(main, /transaction_details: Some\(TransactionDetails::Full\)/);
        assert.match(main, /max_supported_transaction_version: Some\(1\)/);
        assert.match(main, /impl Processor<InstructionProcessorInputType<'_, SmokeInstruction>>/);
        assert.match(main, /input: &InstructionProcessorInputType<'_, SmokeInstruction>/);
        assert.doesNotMatch(main, /async_trait|MetricsCollection|NestedInstructions/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});

test('rejects GraphQL scaffolding without Postgres', () => {
    const outputDirectory = mkdtempSync(join(tmpdir(), 'carbon-cli-invalid-scaffold-'));

    try {
        const result = spawnSync(
            process.execPath,
            [
                join(packageDirectory, 'dist/cli.js'),
                'scaffold',
                '--name',
                'invalid-graphql',
                '--out-dir',
                outputDirectory,
                '--decoder',
                'smoke',
                '--idl',
                join(packageDirectory, 'tests/fixtures/minimal-anchor.json'),
                '--idl-standard',
                'anchor',
                '--data-source',
                'rpc-block-subscribe',
                '--metrics',
                'log',
                '--with-postgres',
                'false',
                '--with-graphql',
                'true',
            ],
            { cwd: packageDirectory, encoding: 'utf8' },
        );

        assert.equal(result.status, 2, `${result.stdout}\n${result.stderr}`);
        assert.match(result.stderr, /GraphQL scaffolding requires Postgres/);
    } finally {
        rmSync(outputDirectory, { force: true, recursive: true });
    }
});
