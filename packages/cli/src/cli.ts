import { Command } from 'commander';
import { resolve, join } from 'path';
import { promptForParse, promptForScaffold } from './lib/prompts';
import { generateDecoder, parseIdlSource, getIdlMetadata } from './lib/decoder';
import type { PackageMetadata } from '@sevenlabs-hq/carbon-codama-renderer';
import { validateDataSource, validateMetrics } from './lib/validation';
import { parseBoolOpt, resolveRpcUrl, runCargoFmt } from './lib/utils';
import { logger, showBanner } from './lib/logger';

const program = new Command();

// eslint-disable-next-line @typescript-eslint/no-var-requires
const pkg = require('../package.json');

// Rustfmt configuration options matching carbon's rustfmt.toml
const cargoFmtOptions: Record<string, string | number | boolean> = {
    comment_width: 80,
    edition: '2021',
    group_imports: 'One',
    imports_granularity: 'One',
    wrap_comments: true,
};

program
    .name('carbon-cli')
    .description('Carbon CLI: Parse IDLs and generate decoders')
    .version(pkg.version ?? '0.0.0');

program.addHelpText(
    'after',
    `\nExamples:
  # Parse command (generate decoder only)
  $ carbon-cli parse -i packages/example/anchor-idl.json -o packages/example/generated -s anchor
  $ carbon-cli parse -i packages/example/codama.json -o packages/example/generated -s codama --event-hints "BuyEvent,CreatePoolEvent"
  $ carbon-cli parse -i <ProgramPubkey> -u devnet -o ./generated -s anchor
  
  # Scaffold with generated decoder from IDL
  $ carbon-cli scaffold -n my-project -o . --idl ./idl.json --idl-standard anchor -s rpc-block-subscribe
  
  # Scaffold with Helius Laserstream datasource
  $ carbon-cli scaffold -n my-project -o . --idl ./idl.json --idl-standard anchor -s helius-laserstream
  
  
  # Interactive mode (no options required)
  $ carbon-cli parse
  $ carbon-cli scaffold
`,
);

program
    .command('parse')
    .description('Parse an IDL and generate a decoder')
    .option('-i, --idl <fileOrAddress>', 'Path to an IDL json file or a Solana program address')
    .option('-o, --out-dir <dir>', 'Output directory for generated code')
    .option(
        '-n, --name <string>',
        'Decoder name (e.g. pumpfun). Used for crate name carbon-<name>-decoder and struct <Name>Decoder. Default: from IDL',
    )
    .option('-c, --as-crate', 'Generate as a Cargo crate layout', false)
    .option('-s, --standard <anchor|codama>', 'Specify the IDL standard to parse', 'anchor')
    .option('--event-hints <csv>', 'Comma-separated names of defined types to parse as CPI Events (Codama only)')
    .option('-u, --url <rpcUrl>', 'RPC URL for fetching IDL when using a program address')
    .option('--program-id <address>', 'Program ID (used if IDL lacks address field)')
    .option('--postgres-mode <generic|typed>', 'Postgres table storage mode', 'typed')
    .option('--with-postgres <boolean>', 'Include Postgres wiring and deps (default: true)')
    .option('--with-graphql <boolean>', 'Include GraphQL wiring and deps (default: true)')
    .option('--with-serde <boolean>', 'Include serde feature for decoder (default: false)')
    .option('--with-base58 <boolean>', 'Include base58 feature for decoder (default: false)')
    .option(
        '--standalone <boolean>',
        'Generate standalone decoder with [workspace] section (default: true). Set to false for workspace dependencies',
        'true',
    )
    .option('--description <string>', 'Package description (default: derived from decoder name)')
    .option('--keywords <csv>', 'Package keywords, comma-separated (default: solana, decoder, plus name parts)')
    .option('--categories <csv>', 'Package categories, comma-separated (default: encoding)')
    .option('--package-version <string>', 'Package version in Cargo.toml (default: 0.1.0)')
    .option('--version-name <name>', 'Decoder version name (e.g., "v1", "v2")')
    .option('--no-clean', 'Do not delete output directory before rendering')
    .action(async opts => {
        showBanner();

        // Prompt for missing options
        if (!opts.idl || !opts.outDir) {
            const answers = await promptForParse(opts);
            Object.assign(opts, answers);
        }

        // Normalize boolean options
        const withPostgres = parseBoolOpt(opts.withPostgres, true);
        const withGraphql = parseBoolOpt(opts.withGraphql, true);
        const withSerdeDefault = !withPostgres && !withGraphql;
        const withSerde = parseBoolOpt(opts.withSerde, withSerdeDefault);
        const withBase58 = parseBoolOpt(opts.withBase58, false);
        const standalone = parseBoolOpt(opts.standalone, true);
        const desc = opts.description != null ? String(opts.description).trim() || undefined : undefined;
        const kws =
            opts.keywords != null
                ? String(opts.keywords)
                      .split(',')
                      .map(k => k.trim())
                      .filter(Boolean)
                : [];
        const cats =
            opts.categories != null
                ? String(opts.categories)
                      .split(',')
                      .map(c => c.trim())
                      .filter(Boolean)
                : [];
        const packageMetadata: PackageMetadata | undefined =
            desc || kws.length > 0 || cats.length > 0
                ? {
                      ...(desc && { description: desc }),
                      ...(kws.length > 0 && { keywords: kws }),
                      ...(cats.length > 0 && { categories: cats }),
                  }
                : undefined;
        const packageName =
            opts.name != null && String(opts.name).trim() !== ''
                ? String(opts.name).trim().replace(/\s+/g, '-')
                : undefined;

        const outDir = resolve(process.cwd(), opts.outDir);

        // Generate decoder with spinner
        logger.startSpinner('Generating decoder...');

        try {
            await generateDecoder({
                idl: String(opts.idl),
                outputDir: outDir,
                standard: opts.standard,
                url: opts.url,
                eventHints: opts.eventHints,
                deleteFolderBeforeRendering: Boolean(opts.clean),
                programId: opts.programId,
                packageName,
                postgresMode: opts.postgresMode,
                withPostgres,
                withGraphql,
                withSerde,
                withBase58,
                standalone,
                packageMetadata,
                version: opts.packageVersion,
                versionName: opts.versionName,
            });

            logger.succeedSpinner('Decoder generated');

            logger.startSpinner('Formatting generated code...');
            try {
                await runCargoFmt(outDir, cargoFmtOptions);
                logger.succeedSpinner('Code formatted successfully');
            } catch (error) {
                logger.failSpinner('Failed to format code');
                logger.warning(`Formatting failed: ${error instanceof Error ? error.message : String(error)}`);
            }

            const idlSource = parseIdlSource(String(opts.idl));
            const source =
                idlSource.type === 'program'
                    ? `program-address (${opts.standard}) @ ${resolveRpcUrl(opts.url)}`
                    : `file (${opts.standard})`;

            logger.decoderSuccess(outDir, source);
        } catch (error) {
            logger.failSpinner('Failed to generate decoder');
            throw error;
        }
    });

// Scaffold command
program
    .command('scaffold')
    .description('Generate skeleton of the project')
    .option('-n, --name <string>', 'Name of your project')
    .option('-o, --out-dir <dir>', 'Output directory')
    .option('-d, --decoder <name>', 'Decoder name (auto-detected from IDL)')
    .option('--idl <fileOrAddress>', 'IDL file or program address')
    .option('--idl-standard <anchor|codama>', 'IDL standard')
    .option('--idl-url <rpcUrl>', 'RPC URL for fetching IDL (when using program address)')
    .option('--event-hints <csv>', 'Event hints for Codama IDL')
    .option('--program-id <address>', 'Program ID (used if IDL lacks address field)')
    .option('-s, --data-source <name>', 'Name of data source')
    .option('-m, --metrics <log|prometheus>', 'Metrics to use', 'log')
    .option('--with-postgres <boolean>', 'Include Postgres wiring and deps (default: true)')
    .option('--with-graphql <boolean>', 'Include GraphQL wiring and deps (default: true)')
    .option('--with-serde <boolean>', 'Include serde feature for decoder (default: false)')
    .option('--with-base58 <boolean>', 'Include base58 feature for decoder (default: false)')
    .option('--package-version <string>', 'Package version in Cargo.toml (default: 0.1.0)')
    .option('--version-name <name>', 'Decoder version name (e.g., "v1", "v2")')
    .option('--postgres-mode <generic|typed>', 'Postgres table storage mode', 'typed')
    .option('--force', 'Overwrite output directory if it exists', false)
    .action(async opts => {
        showBanner();

        // Prompt for missing options (interactive mode)
        // We're in interactive mode if essential options are missing
        const isInteractive = !opts.name || !opts.outDir || !opts.dataSource || !opts.idl;

        if (isInteractive) {
            const answers = await promptForScaffold(opts);
            Object.assign(opts, answers);
        }

        // Normalize and validate options
        const name = String(opts.name);
        const outDir = resolve(process.cwd(), String(opts.outDir));
        const dataSource = String(opts.dataSource);
        const metrics = String(opts.metrics || 'log').toLowerCase();
        const withPostgres = parseBoolOpt(opts.withPostgres, true);
        const withGraphql = parseBoolOpt(opts.withGraphql, true);
        // Default serde to true if both postgres and graphql are disabled (since generated code always includes serde attributes)
        // Otherwise, serde will be auto-enabled by postgres/graphql, so default to false
        const withSerdeDefault = !withPostgres && !withGraphql;
        const withSerde = parseBoolOpt(opts.withSerde, withSerdeDefault);
        const withBase58 = parseBoolOpt(opts.withBase58, false);
        const force = Boolean(opts.force);

        // Use provided decoder name or auto-detect from IDL
        let decoder: string;
        if (opts.decoder && opts.decoder !== 'auto-detect') {
            decoder = String(opts.decoder).trim().replace(/\s+/g, '-');
            logger.info(`Using provided decoder name: ${decoder}`);
        } else {
            logger.startSpinner('Detecting decoder name from IDL...');
            try {
                const metadata = await getIdlMetadata(String(opts.idl), opts.idlStandard, opts.idlUrl, opts.programId);
                decoder = metadata.name;
                logger.succeedSpinner(`Detected decoder: ${decoder}`);
            } catch (error) {
                logger.failSpinner('Failed to detect decoder name');
                throw error;
            }
        }

        // Validate
        validateDataSource(dataSource);
        validateMetrics(metrics);

        // Render scaffold structure
        logger.startSpinner('Creating project structure...');

        try {
            const { renderScaffold } = await import('./lib/scaffold');
            renderScaffold({
                name,
                outDir,
                decoder,
                dataSource,
                metrics,
                withPostgres,
                withGraphql,
                withSerde,
                withBase58,
                force,
                postgresMode: opts.postgresMode,
            });

            logger.succeedSpinner('Project structure created');
        } catch (error) {
            logger.failSpinner('Failed to create project structure');
            throw error;
        }

        // Always generate decoder from IDL
        const workspaceDir = join(outDir, name);
        const decoderPath = join(workspaceDir, 'decoder');

        logger.startSpinner('Generating decoder from IDL...');

        try {
            await generateDecoder({
                idl: String(opts.idl),
                outputDir: decoderPath,
                standard: opts.idlStandard,
                url: opts.idlUrl,
                eventHints: opts.eventHints,
                deleteFolderBeforeRendering: true,
                programId: opts.programId,
                packageName: decoder,
                postgresMode: opts.postgresMode,
                withPostgres,
                withGraphql,
                withSerde,
                withBase58,
                standalone: false,
                workspaceDeps: false,
                version: opts.packageVersion,
                versionName: opts.versionName,
            });

            logger.succeedSpinner('Decoder generated successfully');
        } catch (error) {
            logger.failSpinner('Failed to generate decoder');
            throw error;
        }

        logger.startSpinner('Formatting generated code...');
        try {
            await runCargoFmt(workspaceDir, cargoFmtOptions);
            logger.succeedSpinner('Code formatted successfully');
        } catch (error) {
            logger.failSpinner('Failed to format code');
            logger.warning(`Formatting failed: ${error instanceof Error ? error.message : String(error)}`);
        }

        const decoderInfo = `${decoder} (generated from IDL)`;

        logger.scaffoldSuccess(join(outDir, name), decoderInfo);
    });

program.parseAsync(process.argv);
