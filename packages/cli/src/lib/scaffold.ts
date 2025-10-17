import { mkdirSync, writeFileSync, existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import nunjucks from 'nunjucks';
import { exitWithError } from './utils';

export type ScaffoldOptions = {
    name: string;
    outDir: string;
    decoder: string;
    dataSource: string;
    metrics: 'log' | 'prometheus';
    withPostgres: boolean;
    withGraphql: boolean;
    force?: boolean;
};

function ensureDir(path: string) {
    if (!existsSync(path)) {
        mkdirSync(path, { recursive: true });
    }
}

function buildCargoToml(opts: ScaffoldOptions): string {
    const carbonVersion = '0.9.1';
    const solanaVersion = '=2.1.15';
    const featureParts: string[] = [];

    if (opts.withPostgres) featureParts.push('"postgres"');
    if (opts.withGraphql) featureParts.push('"graphql"');

    // Fix: Properly format the decoder dependency with features
    const decoderDep = featureParts.length
        ? `carbon-${opts.decoder.toLowerCase()}-decoder = { version = "${carbonVersion}", features = [${featureParts.join(', ')}] }`
        : `carbon-${opts.decoder.toLowerCase()}-decoder = "${carbonVersion}"`;

    const datasourceDep = `carbon-${opts.dataSource.toLowerCase()}-datasource = "${carbonVersion}"`;
    const metricsDep = `carbon-${opts.metrics.toLowerCase()}-metrics = "${carbonVersion}"`;

    const grpcDeps =
        opts.dataSource === 'yellowstone-grpc'
            ? `yellowstone-grpc-client = { version = "5.0.0" }\nyellowstone-grpc-proto = { version = "5.0.0" }`
            : '';

    const pgDeps = opts.withPostgres
        ? `sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "macros"] }\nsqlx_migrator = "0.17.0"`
        : '';

    const gqlDeps = opts.withGraphql ? `juniper = "0.15"\naxum = "0.7"` : '';

    // Add rustls workaround if using yellowstone-grpc
    const rustlsDep = opts.dataSource === 'yellowstone-grpc' ? 'rustls = "0.23"' : '';

    const features = ['default = []', opts.withPostgres ? 'postgres = []' : '', opts.withGraphql ? 'graphql = []' : '']
        .filter(Boolean)
        .join('\n');

    return `[package]
name = "${opts.name}"
version = "0.0.1"
edition = "2021"

[dependencies]
async-trait = "0.1.86"
carbon-core = "${carbonVersion}"
${decoderDep}
${datasourceDep}
${metricsDep}
solana-sdk = "${solanaVersion}"
solana-pubkey = "${solanaVersion}"
solana-client = "${solanaVersion}"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
dotenv = "0.15.0"
env_logger = "0.11.5"
log = "0.4.25"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter"] }
${rustlsDep}
${grpcDeps}
${pgDeps}
${gqlDeps}

[features]
${features}
`;
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
    ensureDir(join(base, 'src'));

    const thisDir = dirname(fileURLToPath(import.meta.url));
    const templatesDir = join(thisDir, '..', 'templates');

    if (!existsSync(join(templatesDir, 'project.njk'))) {
        exitWithError('Template file not found. Please ensure cli/templates/project.njk exists.');
    }

    const env = nunjucks.configure(templatesDir, {
        autoescape: false,
        noCache: false,
    });

    const context = {
        projectName: opts.name,
        decoders: [
            {
                name: opts.decoder
                    .replace(/-/g, '_')
                    .split('_')
                    .map((w: string) => w.charAt(0).toUpperCase() + w.slice(1).toLowerCase())
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

    // Generate main.rs
    const rendered = env.render('project.njk', context);
    writeFileSync(join(base, 'src', 'main.rs'), rendered);

    // Generate Cargo.toml
    const cargoToml = buildCargoToml(opts);
    writeFileSync(join(base, 'Cargo.toml'), cargoToml);

    // Generate .gitignore
    const gitignore = `debug/
target/

.env
.DS_Store
`;
    writeFileSync(join(base, '.gitignore'), gitignore);

    // Generate .env
    const envContent = getEnvContent(opts.dataSource, opts.withPostgres);
    if (envContent) {
        writeFileSync(join(base, '.env'), envContent);
    }

    // Generate README.md
    const readme = `# ${opts.name}

Generated by carbon-cli scaffold.

## Run

\`\`\`bash
cargo run
\`\`\`

## Features
- Data source: ${opts.dataSource}
- Metrics: ${opts.metrics}
- Postgres: ${opts.withPostgres}
- GraphQL: ${opts.withGraphql}
`;

    writeFileSync(join(base, 'README.md'), readme);
}
