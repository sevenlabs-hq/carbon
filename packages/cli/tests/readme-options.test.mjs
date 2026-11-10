import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const packageDirectory = dirname(dirname(fileURLToPath(import.meta.url)));
const readme = readFileSync(join(packageDirectory, 'README.md'), 'utf8');

for (const [command, heading] of [
    ['parse', '### Parse Command'],
    ['scaffold', '### Scaffold Command'],
]) {
    test(`README lists every ${command} flag from --help`, () => {
        const help = spawnSync(process.execPath, [join(packageDirectory, 'dist/cli.js'), command, '--help'], {
            encoding: 'utf8',
        });
        assert.equal(help.status, 0, help.stderr);

        const flags = help.stdout
            .split('\n')
            .filter(line => /^ {2}-/.test(line))
            .map(line => line.trim().split(/ {2,}/)[0])
            .flatMap(column => column.match(/--[\w-]+/g) ?? []);
        assert.ok(flags.length > 0);

        const start = readme.indexOf(heading);
        assert.notEqual(start, -1);
        const length = readme.slice(start + heading.length).search(/\n#{2,3} /);
        const section = readme.slice(start, length === -1 ? undefined : start + heading.length + length);
        const options = section
            .split('\n')
            .filter(line => line.startsWith('- `'))
            .join('\n');

        const missing = flags.filter(flag => !new RegExp(`${flag}(?![\\w-])`).test(options));
        assert.deepEqual(missing, []);
    });
}
