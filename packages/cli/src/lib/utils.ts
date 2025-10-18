import chalk from 'chalk';

export function exitWithError(message: string): never {
    console.error(chalk.red('✗ Error: ') + chalk.white(message));
    process.exit(2);
}

export function isBase58Like(s: string): boolean {
    return /^[1-9A-HJ-NP-Za-km-z]+$/.test(s);
}

export function resolveRpcUrl(input: string): string {
    const lower = (input || '').toLowerCase();
    if (lower === 'mainnet-beta') return 'https://api.mainnet-beta.solana.com';
    if (lower === 'devnet') return 'https://api.devnet.solana.com';
    if (/^https?:\/\//.test(input)) return input;
    return input;
}
