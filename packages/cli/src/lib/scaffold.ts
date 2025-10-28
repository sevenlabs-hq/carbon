import { mkdirSync, writeFileSync, existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import nunjucks from 'nunjucks';
import { exitWithError } from './utils';
import { kebabCase } from 'codama';

export type ScaffoldOptions = {
    name: string;
    outDir: string;
    decoder: string;
    decoderMode?: 'published' | 'generate';
    decoderPath?: string; // Path to generated decoder
    dataSource: string;
    metrics: 'log' | 'prometheus';
    withPostgres: boolean;
    withGraphql: boolean;
    withSerde: boolean;
    force?: boolean;
};

function ensureDir(path: string) {
    if (!existsSync(path)) {
        mkdirSync(path, { recursive: true });
    }
}

function buildIndexerCargoContext(opts: ScaffoldOptions) {
    const featureParts: string[] = [];

    if (opts.withPostgres) featureParts.push('"postgres"');
    if (opts.withGraphql) featureParts.push('"graphql"');
    if (opts.withSerde) featureParts.push('"serde"');

    const hasLocalDecoder = true;
    const decoderCrateName = kebabCase(opts.decoder)
    
    let decoderDependency: string = '';
    let decoderFeatures = '';
    if (featureParts.length) {
        decoderFeatures = `, features = [${featureParts.join(', ')}]`;
    }

    const datasourceDep = `carbon-${opts.dataSource.toLowerCase()}-datasource = "0.11.0"`;
    const metricsDep = `carbon-${opts.metrics.toLowerCase()}-metrics = "0.11.0"`;

    const grpcDeps =
        opts.dataSource === 'yellowstone-grpc' || opts.dataSource === 'helius-laserstream'
            ? `yellowstone-grpc-client = { version = "9.0.0" }\nyellowstone-grpc-proto = { version = "9.0.0" }`
            : '';

    const pgDeps = opts.withPostgres
        ? `sqlx = { version = "0.8.6", features = ["postgres", "runtime-tokio-rustls", "macros"] }\nsqlx_migrator = "0.17.0"`
        : '';

    const gqlDeps = opts.withGraphql ? `juniper = "0.15"\naxum = "0.8.4"` : '';

    const rustlsDep = opts.dataSource === 'yellowstone-grpc' || opts.dataSource === 'helius-laserstream' ? 'rustls = "0.23"' : '';

    const features = ['default = []', opts.withPostgres ? 'postgres = []' : '', opts.withGraphql ? 'graphql = []' : '']
        .filter(Boolean)
        .join('\n');

    return {
        projectName: opts.name,
        hasLocalDecoder,
        decoderCrateName,
        decoderFeatures,
        decoderDependency,
        datasourceDep,
        metricsDep,
        grpcDeps,
        pgDeps,
        gqlDeps,
        rustlsDep,
        features,
    };
}

function getEnvContent(dataSource: string, withPostgres: boolean): string {
    const dataSourceLower = dataSource.toLowerCase().replace(/-/g, '_');

    let envContent = '';

    // Add database URL if postgres is enabled
    if (withPostgres) {
        envContent = 'DATABASE_URL=postgres://user:password@localhost/dbname\n';
    }

    // Add datasource-specific env vars
    switch (dataSourceLower) {
        case 'helius_atlas_ws':
            envContent += 'HELIUS_API_KEY=your-atlas-ws-url-here';
            break;
        case 'helius_laserstream':
            envContent += 'GEYSER_URL=your-grpc-url-here\nX_TOKEN=your-x-token-here';
            break;
        case 'rpc_block_subscribe':
            envContent += 'RPC_WS_URL=your-rpc-ws-url-here';
            break;
        case 'rpc_transaction_crawler':
            envContent += 'RPC_URL=your-rpc-url-here';
            break;
        case 'yellowstone_grpc':
            envContent += 'GEYSER_URL=your-rpc-url-here\nX_TOKEN=your-x-token-here';
            break;
    }

    return envContent;
}

export function renderScaffold(opts: ScaffoldOptions) {
    const base = join(opts.outDir, opts.name);

    if (existsSync(base) && !opts.force) {
        exitWithError(`Output directory already exists: ${base} (use --force to overwrite)`);
    }

    ensureDir(base);
    
    // Create workspace structure
    const indexerDir = join(base, 'indexer');
    ensureDir(indexerDir);
    ensureDir(join(indexerDir, 'src'));

    const thisDir = dirname(fileURLToPath(import.meta.url));
    const templatesDir = join(thisDir, '..', 'templates');

    if (!existsSync(join(templatesDir, 'project.njk'))) {
        exitWithError('Template file not found. Please ensure cli/templates/project.njk exists.');
    }

    const env = nunjucks.configure(templatesDir, {
        autoescape: false,
        noCache: false,
    });

    const hasLocalDecoder = opts.decoderMode === 'generate';

    // Context for main.rs
    const mainContext = {
        projectName: opts.name,
        decoders: [
            {
                name: opts.decoder
                    .split('-')
                    .map((w: string) => w.charAt(0).toUpperCase() + w.slice(1))
                    .join(''),
                module_name: opts.decoder.replace(/-/g, '_'),
            },
        ],
        data_source: {
            module_name: opts.dataSource.replace(/-/g, '_'),
        },
        metrics: {
            name: opts.metrics === 'prometheus' ? 'Prometheus' : 'Log',
            module_name: opts.metrics,
        },
        withPostgres: opts.withPostgres,
        withGraphQL: opts.withGraphql,
    };

    // Generate workspace Cargo.toml
    const workspaceContext = {
        hasLocalDecoder,
    };
    const workspaceToml = env.render('workspace.njk', workspaceContext);
    writeFileSync(join(base, 'Cargo.toml'), workspaceToml);

    // Generate indexer main.rs
    const rendered = env.render('project.njk', mainContext);
    writeFileSync(join(indexerDir, 'src', 'main.rs'), rendered);

    // Generate indexer Cargo.toml
    const indexerCargoContext = buildIndexerCargoContext(opts);
    const indexerCargoToml = env.render('indexer-cargo.njk', indexerCargoContext);
    writeFileSync(join(indexerDir, 'Cargo.toml'), indexerCargoToml);

    // Generate .gitignore at workspace root
    const gitignore = `debug/
target/

.env
.DS_Store
`;
    writeFileSync(join(base, '.gitignore'), gitignore);

    // Generate .env at workspace root
    const envContent = getEnvContent(opts.dataSource, opts.withPostgres);
    if (envContent) {
        writeFileSync(join(base, '.env'), envContent);
    }

    // Generate README.md at workspace root
    const readme = `# ${opts.name}

Generated by carbon-cli scaffold.

## Structure

This is a Cargo workspace containing:
- \`indexer/\` - The main indexer application${hasLocalDecoder ? '\n- `decoder/` - Generated decoder from IDL' : ''}

## Run

\`\`\`bash
cargo run -p ${opts.name}-indexer
\`\`\`

## Features
- Data source: ${opts.dataSource}
- Metrics: ${opts.metrics}
- Postgres: ${opts.withPostgres}
- GraphQL: ${opts.withGraphql}
- Decoder: ${hasLocalDecoder ? 'Generated locally' : `Published (carbon-${opts.decoder}-decoder)`}
`;

    writeFileSync(join(base, 'README.md'), readme);
}
