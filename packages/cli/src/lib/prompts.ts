import { input, select, confirm } from '@inquirer/prompts';
import { existsSync, readFileSync } from 'fs';
import { resolve } from 'path';
import { isBase58Like } from './utils';

export type ParseOptions = {
    idl?: string;
    outDir?: string;
    standard?: string;
    url?: string;
    eventHints?: string;
    clean?: boolean;
    asCrate?: boolean;
    programId?: string;
};

export async function promptForParse(existingOpts: ParseOptions = {}): Promise<ParseOptions> {
    const idl = existingOpts.idl || await input({
        message: 'IDL file path or Solana program address:',
        validate: (value) => {
            if (!value.trim()) return 'IDL is required';
            return true;
        },
    });

    const outDir = existingOpts.outDir || await input({
        message: 'Output directory:',
        default: './generated',
        validate: (value) => {
            if (!value.trim()) return 'Output directory is required';
            return true;
        },
    });

    const idlArg = idl.trim();
    const looksLikeFile = idlArg.endsWith('.json');
    const looksLikeProgram = !looksLikeFile && idlArg.length >= 32 && idlArg.length <= 44;

    let standard: 'anchor' | 'codama' = 'anchor';
    let url: string | undefined = existingOpts.url;
    let eventHints: string | undefined = existingOpts.eventHints;
    let clean = existingOpts.clean !== undefined ? existingOpts.clean : true;

    if (looksLikeProgram) {
        // Program address - only supports anchor
        if (!url) {
            url = await input({
                message: 'RPC URL for fetching IDL:',
                default: 'mainnet-beta',
                validate: (value) => {
                    if (!value.trim()) return 'RPC URL is required when using a program address';
                    return true;
                },
            });
        }
        standard = 'anchor';
    } else {
        // File path
        if (!existingOpts.standard) {
            standard = (await select({
                message: 'IDL standard:',
                choices: [
                    { name: 'Anchor', value: 'anchor' },
                    { name: 'Codama', value: 'codama' },
                ],
                default: 'anchor',
            })) as 'anchor' | 'codama';
        } else {
            standard = existingOpts.standard as 'anchor' | 'codama';
        }

        if (standard === 'codama' && !eventHints) {
            eventHints = await input({
                message: 'Event hints (comma-separated names of types to parse as CPI Events):',
                default: '',
            });
        }
    }

    if (existingOpts.clean === undefined) {
        clean = await confirm({
            message: 'Delete output directory before rendering?',
            default: true,
        });
    }

    let programId: string | undefined = existingOpts.programId;
    
    if (!looksLikeProgram) {
        try {
            const idlPath = resolve(process.cwd(), idlArg);
            const idlJson = JSON.parse(readFileSync(idlPath, 'utf8'));
            const hasAddress = idlJson.address || idlJson.metadata?.address;
            
            if (!hasAddress && !programId) {
                programId = await input({
                    message: 'Program ID (IDL missing address field):',
                    validate: (value) => {
                        if (!value.trim()) return 'Program ID is required when IDL lacks address';
                        if (!isBase58Like(value) || value.length < 32 || value.length > 44) {
                            return 'Invalid Program ID format (must be base58)';
                        }
                        return true;
                    },
                });
            }
        } catch (error) {
            // If we can't read the IDL file, we'll let the decoder generation handle the error
        }
    }

    return {
        idl: idlArg,
        outDir,
        standard,
        url,
        eventHints: eventHints?.trim() || undefined,
        clean,
        asCrate: existingOpts.asCrate || false,
        programId,
    };
}

export type ScaffoldOptions = {
    name?: string;
    outDir?: string;
    decoder?: string;
    idl?: string;
    idlStandard?: string;
    idlUrl?: string;
    dataSource?: string;
    metrics?: string;
    withPostgres?: boolean;
    withGraphql?: boolean;
    force?: boolean;
    programId?: string;
};

export async function promptForScaffold(existingOpts: ScaffoldOptions = {}): Promise<ScaffoldOptions> {
    const name = existingOpts.name || await input({
        message: 'Project name:',
        validate: (value) => {
            if (!value.trim()) return 'Project name is required';
            if (!/^[a-z0-9-]+$/.test(value)) {
                return 'Project name should contain only lowercase letters, numbers, and hyphens';
            }
            return true;
        },
    });

    const outDir = existingOpts.outDir || await input({
        message: 'Output directory:',
        default: '.',
        validate: (value) => {
            if (!value.trim()) return 'Output directory is required';
            return true;
        },
    });

    // Check if directory already exists
    const fullPath = resolve(process.cwd(), outDir, name);
    let force = existingOpts.force || false;
    if (existsSync(fullPath) && !force) {
        force = await confirm({
            message: `Directory ${fullPath} already exists. Overwrite?`,
            default: false,
        });
        if (!force) {
            throw new Error('Operation cancelled by user');
        }
    }

    let decoder: string | undefined = existingOpts.decoder;
    let idl: string | undefined = existingOpts.idl;
    let idlStandard: string | undefined = existingOpts.idlStandard;
    let idlUrl: string | undefined = existingOpts.idlUrl;

    // Always generate mode - ask for IDL details
    idl = idl || await input({
        message: 'IDL file path or Solana program address:',
        validate: (value) => {
            if (!value.trim()) return 'IDL is required';
            return true;
        },
    });

    const idlArg = idl.trim();
    const looksLikeFile = idlArg.endsWith('.json');
    const looksLikeProgram = !looksLikeFile && idlArg.length >= 32 && idlArg.length <= 44;

    if (looksLikeProgram) {
        // Program address - only supports anchor
        if (!idlUrl) {
            idlUrl = await input({
                message: 'RPC URL for fetching IDL:',
                default: 'mainnet-beta',
                validate: (value) => {
                    if (!value.trim()) return 'RPC URL is required when using a program address';
                    return true;
                },
            });
        }
        idlStandard = 'anchor';
    } else {
        // File path
        if (!idlStandard) {
            idlStandard = (await select({
                message: 'IDL standard:',
                choices: [
                    { name: 'Anchor', value: 'anchor' },
                    { name: 'Codama', value: 'codama' },
                ],
                default: 'anchor',
            })) as 'anchor' | 'codama';
        }
    }

    let programId: string | undefined = existingOpts.programId;
    
    if (!looksLikeProgram) {
        const idlPath = resolve(process.cwd(), idlArg);
        const idlJson = JSON.parse(readFileSync(idlPath, 'utf8'));
        const hasAddress = idlJson.address || idlJson.metadata?.address;
        
        if (!hasAddress && !programId) {
            programId = await input({
                message: 'Program ID (IDL missing address field):',
                validate: (value) => {
                    if (!value.trim()) return 'Program ID is required when IDL lacks address';
                    if (!isBase58Like(value) || value.length < 32 || value.length > 44) {
                        return 'Invalid Program ID format (must be base58, 32-44 chars)';
                    }
                    return true;
                },
            });
        }
    }

    // Ask for decoder name (optional - will auto-detect from IDL if not provided)
    if (!decoder) {
        const decoderInput = await input({
            message: 'Decoder name (press Enter to auto-detect from IDL):',
            default: '',
        });
        decoder = decoderInput.trim() || 'auto-detect';
    }

    const dataSource = existingOpts.dataSource || await select({
        message: 'Data source:',
        choices: [
            { name: 'Helius Atlas WebSocket', value: 'helius-atlas-ws' },
            { name: 'Helius Laserstream', value: 'helius-laserstream' },
            { name: 'RPC Block Subscribe', value: 'rpc-block-subscribe' },
            { name: 'RPC Program Subscribe', value: 'rpc-program-subscribe' },
            { name: 'RPC Transaction Crawler', value: 'rpc-transaction-crawler' },
            { name: 'Yellowstone gRPC', value: 'yellowstone-grpc' },
        ],
    });

    const metrics = existingOpts.metrics || (await select({
        message: 'Metrics type:',
        choices: [
            { name: 'Log', value: 'log' },
            { name: 'Prometheus', value: 'prometheus' },
        ],
        default: 'log',
    })) as 'log' | 'prometheus';

    const withPostgres = existingOpts.withPostgres !== undefined ? existingOpts.withPostgres : await confirm({
        message: 'Add Postgres processors for data persistence?',
        default: true,
    });

    const withGraphql = existingOpts.withGraphql !== undefined ? existingOpts.withGraphql : await confirm({
        message: 'Enable GraphQL API for querying data?',
        default: true,
    });

    return {
        name,
        outDir,
        decoder,
        idl,
        idlStandard,
        idlUrl,
        dataSource,
        metrics,
        withPostgres,
        withGraphql,
        force,
        programId,
    };
}

