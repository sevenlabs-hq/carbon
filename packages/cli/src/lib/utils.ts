import chalk from 'chalk';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

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

/**
 * Runs cargo fmt in the specified directory to format Rust code
 * @param directory The directory containing Rust code to format
 * @throws Error if cargo fmt fails
 */
export async function runCargoFmt(directory: string): Promise<void> {
    try {
        await execAsync('cargo fmt', {
            cwd: directory,
            env: process.env,
        });
    } catch (error: any) {
        const stderr = error.stderr || '';
        const stdout = error.stdout || '';
        const fullOutput = stderr + stdout;
        
        const errorLines = fullOutput
            .split('\n')
            .filter((line: string) => 
                line.includes('error') && 
                !line.includes('Warning: can\'t set') &&
                !line.includes('unstable features')
            );
        
        if (errorLines.length > 0) {
            const errorMessage = errorLines.join('\n') || error.message || 'Unknown error';
            throw new Error(`Failed to format code with cargo fmt: ${errorMessage}`);
        }
    }
}
