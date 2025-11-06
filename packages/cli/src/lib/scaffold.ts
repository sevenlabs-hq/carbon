import { mkdirSync, writeFileSync, existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import nunjucks from 'nunjucks';
import { exitWithError } from './utils';
import { kebabCase } from 'codama';
import * as Datasources from '../datasources';
import type { DecoderMeta } from '../datasources';
import { VERSIONS, getCrateDependencyString } from '@sevenlabs-hq/carbon-versions';

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
    postgresMode?: 'generic' | 'typed';
};

function ensureDir(path: string) {
    if (!existsSync(path)) {
        mkdirSync(path, { recursive: true });
    }
}

function buildProjectImports(ctx: any): string {
    const lines: string[] = [];

    // Common
    lines.push('use std::{env, sync::Arc};');

    // Feature-dependent
    if (!ctx.withPostgres) {
        lines.push('use async_trait::async_trait;');
        lines.push('use carbon_core::deserialize::ArrangeAccounts;');
        lines.push('use carbon_core::instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions};');
        lines.push('use carbon_core::metrics::MetricsCollection;');
        lines.push('use carbon_core::processor::Processor;');
    }

    lines.push('use carbon_core::error::CarbonResult;');

    if (ctx.withPostgres) {
        if (ctx.useGenericPostgres) {
            lines.push('use carbon_core::postgres::processors::{PostgresJsonAccountProcessor, PostgresJsonInstructionProcessor};');
            lines.push('use carbon_core::postgres::rows::{GenericAccountsMigration, GenericInstructionMigration};');
        } else {
            lines.push('use carbon_core::postgres::processors::{PostgresAccountProcessor, PostgresInstructionProcessor};');
        }
        lines.push('use sqlx_migrator::{Info, Migrate, Plan};');
    }

    // Metrics
    lines.push(`use carbon_${ctx.metrics.module_name}_metrics::${ctx.metrics.name}Metrics;`);

    // Decoders
    for (const d of ctx.decoders as Array<{ name: string; module_name: string }>) {
        const crate = `carbon_${d.module_name}_decoder`;
        if (ctx.withPostgres) {
            if (!ctx.useGenericPostgres) {
                lines.push(`use ${crate}::accounts::postgres::{${d.name}AccountWithMetadata, ${d.name}AccountsMigration};`);
                lines.push(`use ${crate}::accounts::${d.name}Account;`);
                lines.push(`use ${crate}::instructions::postgres::{${d.name}InstructionWithMetadata, ${d.name}InstructionsMigration};`);
                lines.push(`use ${crate}::instructions::${d.name}Instruction;`);
            } else {
                lines.push(`use ${crate}::accounts::${d.name}Account;`);
                lines.push(`use ${crate}::instructions::${d.name}Instruction;`);
            }
        } else {
            lines.push(`use ${crate}::instructions::${d.name}Instruction;`);
        }
        if (ctx.withGraphQL) {
            lines.push(`use ${crate}::graphql::{QueryRoot, context::GraphQLContext};`);
        }
        lines.push(`use ${crate}::${d.name}Decoder;`);
        
        const dsModule = ctx.data_source.module_name as string;
        const usesProgramIds = dsModule === 'yellowstone_grpc' || 
                              dsModule === 'helius_laserstream' || 
                              dsModule === 'rpc_program_subscribe' || 
                              dsModule === 'rpc_transaction_crawler';
        if (usesProgramIds) {
            lines.push(`use ${crate}::PROGRAM_ID as ${d.name.toUpperCase()}_PROGRAM_ID;`);
        }
    }

    // Datasource-specific imports are provided exclusively by the datasource builders

    if (ctx.withGraphQL) {
        lines.push('use std::net::SocketAddr;');
    }

    // Include datasource-specific imports from TS builders (authoritative)
    if (ctx.datasource_imports) {
        lines.push(ctx.datasource_imports);
    }

    return lines.join('\n');
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

    const datasourceCrateName = `carbon-${opts.dataSource.toLowerCase()}-datasource`;
    const datasourceDep = getCrateDependencyString(
        datasourceCrateName,
        VERSIONS[datasourceCrateName as keyof typeof VERSIONS] || VERSIONS["carbon-core"]
    );
    const metricsCrateName = `carbon-${opts.metrics.toLowerCase()}-metrics`;
    const metricsDep = getCrateDependencyString(
        metricsCrateName,
        VERSIONS[metricsCrateName as keyof typeof VERSIONS] || VERSIONS["carbon-core"]
    );

    const grpcDeps =
        opts.dataSource === 'yellowstone-grpc' || opts.dataSource === 'helius-laserstream'
            ? `${getCrateDependencyString("yellowstone-grpc-client", VERSIONS["yellowstone-grpc-client"])}\n${getCrateDependencyString("yellowstone-grpc-proto", VERSIONS["yellowstone-grpc-proto"])}`
            : '';

    const pgDeps = opts.withPostgres
        ? `${getCrateDependencyString("sqlx", VERSIONS.sqlx, ["postgres", "runtime-tokio-rustls", "macros"])}\n${getCrateDependencyString("sqlx_migrator", VERSIONS["sqlx_migrator"])}`
        : '';

    const gqlDeps = opts.withGraphql 
        ? `${getCrateDependencyString("juniper", VERSIONS.juniper)}\n${getCrateDependencyString("axum", VERSIONS.axum)}`
        : '';

    const rustlsDep = opts.dataSource === 'yellowstone-grpc' || opts.dataSource === 'helius-laserstream' 
        ? getCrateDependencyString("rustls", VERSIONS.rustls)
        : '';

    const features = ['default = []', opts.withPostgres ? 'postgres = []' : '', opts.withGraphql ? 'graphql = []' : '']
        .filter(Boolean)
        .join('\n');

    const crawlerDeps = opts.dataSource === 'rpc-transaction-crawler' 
        ? getCrateDependencyString("solana-commitment-config", VERSIONS["solana-commitment-config"])
        : '';
    const programDeps = opts.dataSource === 'rpc-program-subscribe' 
        ? getCrateDependencyString("solana-account-decoder", VERSIONS["solana-account-decoder"])
        : '';

    const carbonCoreDep = getCrateDependencyString("carbon-core", VERSIONS["carbon-core"], ["postgres", "graphql"]);
    const solanaPubkeyDep = getCrateDependencyString("solana-pubkey", VERSIONS["solana-pubkey"]);
    const solanaClientDep = getCrateDependencyString("solana-client", VERSIONS["solana-client"]);
    const solanaInstructionDep = getCrateDependencyString("solana-instruction", VERSIONS["solana-instruction"]);
    const asyncTraitDep = getCrateDependencyString("async-trait", VERSIONS["async-trait"]);
    const tokioDep = getCrateDependencyString("tokio", VERSIONS["tokio"]);
    const dotenvDep = getCrateDependencyString("dotenv", VERSIONS["dotenv"]);
    const envLoggerDep = getCrateDependencyString("env_logger", VERSIONS["env_logger"]);
    const logDep = getCrateDependencyString("log", VERSIONS["log"]);
    const anyhowDep = getCrateDependencyString("anyhow", VERSIONS["anyhow"]);
    const tracingDep = getCrateDependencyString("tracing", VERSIONS["tracing"]);
    const tracingSubscriberDep = getCrateDependencyString("tracing-subscriber", VERSIONS["tracing-subscriber"]);

    return {
        projectName: opts.name,
        hasLocalDecoder,
        decoderCrateName,
        decoderFeatures,
        carbonCoreDep,
        solanaPubkeyDep,
        solanaClientDep,
        solanaInstructionDep,
        asyncTraitDep,
        tokioDep,
        dotenvDep,
        envLoggerDep,
        logDep,
        anyhowDep,
        tracingDep,
        tracingSubscriberDep,
        decoderDependency,
        datasourceDep,
        metricsDep,
        grpcDeps,
        pgDeps,
        gqlDeps,
        rustlsDep,
        crawlerDeps,
        programDeps,
        features,
        versions: VERSIONS,
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

    // Context base for main.rs
    const mainContext: any = {
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
        useGenericPostgres: opts.postgresMode === 'generic',
    };

    // Build datasource artifacts from TS module
    const dsModuleName = mainContext.data_source.module_name as string;
    const builder = Datasources.getDatasourceBuilder(dsModuleName);
    if (builder) {
        const decodersMeta = mainContext.decoders as DecoderMeta[];
        const artifact = builder(decodersMeta);
        // Compose import lines
        const datasource_imports = artifact.imports
            .map((i: string) => `use ${i};`)
            .join('\n');
        mainContext.datasource_imports = datasource_imports;
        mainContext.datasource_init = artifact.init;
    } else {
        // Provide a clearer error message if no builder is found
        const available = Object.keys((Datasources as unknown as { getDatasourceBuilder: any }).getDatasourceBuilder ? {
            helius_laserstream: true,
            rpc_block_subscribe: true,
            yellowstone_grpc: true,
            rpc_transaction_crawler: true,
            rpc_program_subscribe: true,
        } : {});
        exitWithError(`No datasource builder found for '${dsModuleName}'. Available: ${available.join(', ')}`);
    }

    // Generate workspace Cargo.toml
    const workspaceContext = {
        hasLocalDecoder,
    };
    const workspaceToml = env.render('workspace.njk', workspaceContext);
    writeFileSync(join(base, 'Cargo.toml'), workspaceToml);

    // Compute dynamic imports for main.rs
    mainContext.imports = buildProjectImports(mainContext);

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
