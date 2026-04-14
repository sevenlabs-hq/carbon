import { mkdirSync, writeFileSync, existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import nunjucks from 'nunjucks';
import { exitWithError } from './utils';
import { kebabCase } from '@codama/nodes';
import * as Datasources from '../datasources';
import type { DecoderMeta, DatasourceArtifact } from '../datasources';
import { generateIndexerCargoToml } from './cargoTomlGenerator';

export type ScaffoldOptions = {
    name: string;
    outDir: string;
    decoder: string;
    dataSource: string;
    metrics: 'log' | 'prometheus';
    withPostgres: boolean;
    withGraphql: boolean;
    withSerde: boolean;
    withBase58?: boolean;
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
        lines.push('use carbon_core::instruction::{InstructionMetadata, NestedInstructions};');
        lines.push('use carbon_core::metrics::MetricsCollection;');
        lines.push('use carbon_core::processor::Processor;');
    }

    lines.push('use carbon_core::error::CarbonResult;');

    if (ctx.withPostgres) {
        if (ctx.useGenericPostgres) {
            lines.push(
                'use carbon_core::postgres::processors::{PostgresJsonAccountProcessor, PostgresJsonInstructionProcessor};',
            );
            lines.push('use carbon_core::postgres::rows::{GenericAccountsMigration, GenericInstructionMigration};');
        } else {
            lines.push(
                'use carbon_core::postgres::processors::{PostgresAccountProcessor, PostgresInstructionProcessor};',
            );
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
                lines.push(
                    `use ${crate}::accounts::postgres::{${d.name}AccountWithMetadata, ${d.name}AccountsMigration};`,
                );
                lines.push(`use ${crate}::accounts::${d.name}Account;`);
                lines.push(
                    `use ${crate}::instructions::postgres::{${d.name}InstructionWithMetadata, ${d.name}InstructionsMigration};`,
                );
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
        const usesProgramIds =
            dsModule === 'yellowstone_grpc' ||
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

function getEnvContent(artifact: DatasourceArtifact | undefined, withPostgres: boolean): string {
    const lines: string[] = [];

    if (withPostgres) {
        lines.push('DATABASE_URL=postgres://user:password@localhost/dbname');
    }

    for (const key of artifact?.env?.required ?? []) {
        lines.push(`${key}=your-${key.toLowerCase().replace(/_/g, '-')}-here`);
    }

    return lines.join('\n');
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
    if (!builder) {
        exitWithError(`No datasource builder found for '${dsModuleName}'. This is an internal error — please report it.`);
    }
    const decodersMeta = mainContext.decoders as DecoderMeta[];
    const artifact = builder(decodersMeta);
    mainContext.datasource_imports = artifact.imports.map((i: string) => `use ${i};`).join('\n');
    mainContext.datasource_init = artifact.init;

    // Generate workspace Cargo.toml
    const workspaceToml = env.render('workspace.njk', {});
    writeFileSync(join(base, 'Cargo.toml'), workspaceToml);

    // Compute dynamic imports for main.rs
    mainContext.imports = buildProjectImports(mainContext);

    // Generate indexer main.rs
    const rendered = env.render('project.njk', mainContext);
    writeFileSync(join(indexerDir, 'src', 'main.rs'), rendered);

    // Generate indexer Cargo.toml
    const indexerCargoToml = generateIndexerCargoToml(opts);
    writeFileSync(join(indexerDir, 'Cargo.toml'), indexerCargoToml);

    // Generate .gitignore at workspace root
    const gitignore = `debug/
target/

.env
.DS_Store
`;
    writeFileSync(join(base, '.gitignore'), gitignore);

    // Generate .env at workspace root
    const envContent = getEnvContent(artifact, opts.withPostgres);
    if (envContent) {
        writeFileSync(join(base, '.env'), envContent);
    }

    // Generate README.md at workspace root
    const readme = `# ${opts.name}

Generated by carbon-cli scaffold.

## Structure

This is a Cargo workspace containing:
- \`indexer/\` - The main indexer application
- \`decoder/\` - Generated decoder from IDL

## Run

\`\`\`bash
cargo run -p ${opts.name}-indexer
\`\`\`

## Features
- Data source: ${opts.dataSource}
- Metrics: ${opts.metrics}
- Postgres: ${opts.withPostgres}
- GraphQL: ${opts.withGraphql}
- Decoder: carbon-${opts.decoder}-decoder (generated)
`;

    writeFileSync(join(base, 'README.md'), readme);
}
