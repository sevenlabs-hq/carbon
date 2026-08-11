#!/usr/bin/env node
import { writeFileSync } from 'fs';
import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Use require from the CLI package to access its dependencies
const cliPackagePath = join(__dirname, '..', 'packages', 'cli');
const require = createRequire(join(cliPackagePath, 'package.json'));
const anchor = require('@coral-xyz/anchor');

const [programId, rpcUrl, outputPath] = process.argv.slice(2);

if (!programId || !rpcUrl || !outputPath) {
    console.error('Usage: fetch-idl.mjs <program-id> <rpc-url> <output-path>');
    process.exit(1);
}

async function fetchAndCache() {
    try {
        const connection = new anchor.web3.Connection(rpcUrl, 'confirmed');
        const programIdPubkey = new anchor.web3.PublicKey(programId);
        const idl = await anchor.Program.fetchIdl(programIdPubkey, { connection });

        if (!idl) {
            console.error('No Anchor IDL found for program address.');
            process.exit(1);
        }

        writeFileSync(outputPath, JSON.stringify(idl, null, 2));
        console.log(outputPath);
    } catch (e) {
        console.error(`Failed to fetch IDL: ${e.message}`);
        process.exit(1);
    }
}

fetchAndCache();
