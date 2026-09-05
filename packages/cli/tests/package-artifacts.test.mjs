import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const packageDirectory = dirname(dirname(fileURLToPath(import.meta.url)));
const packageJson = JSON.parse(readFileSync(join(packageDirectory, 'package.json'), 'utf8'));

test('published entry points exist in the built package', () => {
    const entryPoints = [...Object.values(packageJson.bin), ...Object.values(packageJson.exports['.'])];

    for (const entryPoint of entryPoints) {
        assert.ok(existsSync(join(packageDirectory, entryPoint)), `missing package entry point: ${entryPoint}`);
    }
});
