import { renderVisitor } from '@sevenlabs-hq/carbon-codama-renderer';
import { join } from 'path';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import anchorIdl from './anchor-idl.json' with { type: 'json' };
import { createFromRoot } from 'codama';

// Example: Create a simple program with an account and instruction
const codama = createFromRoot(rootNodeFromAnchor(anchorIdl as any));

// Generate Rust code
const outputPath = join(process.cwd(), '../generated');

// Render the Rust code
codama.accept(renderVisitor(outputPath, {}));

console.log(`✅ Generated Rust code in ${outputPath}`);
