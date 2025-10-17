import { Command } from 'commander';
import { readFileSync } from 'fs';
import { resolve } from 'path';
import { createFromRoot } from 'codama';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import renderer, { renderVisitor } from '@sevenlabs-hq/carbon-codama-renderer';
import { exitWithError, isBase58Like, resolveRpcUrl } from './lib/utils';
import { fetchAnchorIdl } from './lib/anchor';

const program = new Command();

// Load version from package.json for OSS hygiene
// eslint-disable-next-line @typescript-eslint/no-var-requires
const pkg = require('../package.json');
program
    .name('carbon-cli')
    .description('Carbon CLI: Parse IDLs and generate decoders')
    .version(pkg.version ?? '0.0.0');

program.addHelpText(
    'after',
    `\nExamples:\n  $ carbon-cli parse -i packages/example/anchor-idl.json -o packages/example/generated -s anchor\n  $ carbon-cli parse -i packages/example/codama.json -o packages/example/generated -s codama --event-hints "BuyEvent,CreatePoolEvent"\n  $ carbon-cli parse -i <ProgramPubkey> -u devnet -o ./generated -s anchor\n`,
);

// parse: main entry
program
    .command('parse')
    .description('Parse an IDL and generate a decoder')
    .requiredOption('-i, --idl <fileOrAddress>', 'Path to an IDL json file or a Solana program address')
    .requiredOption('-o, --out-dir <dir>', 'Output directory for generated code')
    .option('-c, --as-crate', 'Generate as a Cargo crate layout', false)
    .option('-s, --standard <anchor|codama>', 'Specify the IDL standard to parse', 'anchor')
    .option('--event-hints <csv>', 'Comma-separated names of defined types to parse as CPI Events (Codama only)')
    .option('-u, --url <rpcUrl>', 'RPC URL for fetching IDL when using a program address')
    .option('--no-clean', 'Do not delete output directory before rendering', false)
    .action(async opts => {
        const outDir = resolve(process.cwd(), opts.outDir);
        const deleteFolderBeforeRendering = Boolean(opts.clean);

        const idlArg: string = String(opts.idl);
        const looksLikeFile = idlArg.endsWith('.json');
        const looksLikeProgram = !looksLikeFile && idlArg.length >= 32 && idlArg.length <= 44 && isBase58Like(idlArg);

        if (!looksLikeFile && !looksLikeProgram) {
            exitWithError('Invalid --idl: must be a .json file or a Solana program address');
        }

        const standard = String(opts.standard || 'anchor').toLowerCase() as 'anchor' | 'codama';
        if (standard !== 'anchor' && standard !== 'codama') {
            exitWithError("--standard must be 'anchor' or 'codama'");
        }
        if (standard === 'anchor' && typeof opts.eventHints === 'string') {
            exitWithError("The '--event-hints' option can only be used with --standard codama");
        }
        if (standard === 'codama' && looksLikeProgram) {
            exitWithError('--standard codama is only supported with --idl <codama.json>');
        }

        if (looksLikeProgram) {
            if (!opts.url) {
                exitWithError('When --idl is a program address, --url is required');
            }
            if (standard !== 'anchor') {
                exitWithError('Only --standard anchor is supported when using a program address');
            }
            const idlJson = await fetchAnchorIdl(idlArg, opts.url);
            const codama = createFromRoot(rootNodeFromAnchor(idlJson));
            codama.accept(
                renderVisitor(outDir, {
                    deleteFolderBeforeRendering,
                    anchorEvents: idlJson.events,
                }),
            );
            // eslint-disable-next-line no-console
            console.log('✅ Generated Rust decoder');
            // eslint-disable-next-line no-console
            console.log(`  • output: ${outDir}`);
            // eslint-disable-next-line no-console
            console.log(`  • source: program-address (${standard}) @ ${resolveRpcUrl(opts.url)}`);
            return;
        }

        const idlPath = resolve(process.cwd(), idlArg);
        const idlJson = JSON.parse(readFileSync(idlPath, 'utf8'));

        if (standard === 'anchor') {
            const codama = createFromRoot(rootNodeFromAnchor(idlJson));
            codama.accept(
                renderVisitor(outDir, {
                    deleteFolderBeforeRendering,
                    anchorEvents: idlJson.events,
                }),
            );
        } else {
            const codama = createFromRoot(idlJson as any);
            codama.accept(
                renderVisitor(outDir, {
                    deleteFolderBeforeRendering,
                }),
            );
        }

        // eslint-disable-next-line no-console
        console.log('✅ Generated Rust decoder');
        // eslint-disable-next-line no-console
        console.log(`  • output: ${outDir}`);
        // eslint-disable-next-line no-console
        console.log(`  • source: file (${standard})`);
    });

program
    .command('scaffold')
    .description('Generate skeleton of the project')
    .requiredOption('-n, --name <string>', 'Name of your project')
    .requiredOption('-o, --out-dir <dir>', 'Output directory')
    .requiredOption('-d, --decoders <csv>', 'Comma-separated names of decoders')
    .requiredOption('-s, --data-source <name>', 'Name of data source')
    .option('-m, --metrics <log|prometheus>', 'Metrics to use', 'log')
    .action(async opts => {
        exitWithError('scaffold not implemented yet. Flags received: ' + JSON.stringify(opts));
    });

program.parseAsync(process.argv);
