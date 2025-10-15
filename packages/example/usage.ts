import { renderVisitor } from '@sevenlabs-hq/carbon-codama-renderer';
import { join } from 'path';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import anchorIdl from './idl.json' with { type: 'json' };
import { camelCase, createFromRoot } from 'codama';
import fs from 'fs';

// Example: Create a simple program with an account and instruction
const codama = createFromRoot(rootNodeFromAnchor(anchorIdl as any));

fs.writeFileSync('./codama.json', codama.getJson());

// Generate Rust code
const outputPath = join(process.cwd(), './generated');

// Render the Rust code
codama.accept(renderVisitor(outputPath, {
    anchorEvents: anchorIdl.events,
}));

console.log(`✅ Generated Rust code in ${outputPath}`);
