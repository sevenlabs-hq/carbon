import chalk from 'chalk';
import ora, { Ora } from 'ora';
import gradient from 'gradient-string';

const carbonGradient = gradient(['#FF6B35', '#FFB84D', '#F7931E']);
const successGradient = gradient(['#00F260', '#0575E6']);

/**
 * Display the Carbon CLI ASCII art banner
 */
export function showBanner(): void {
    const banner = `
 ██████╗ █████╗ ██████╗ ██████╗  ██████╗ ███╗   ██╗
██╔════╝██╔══██╗██╔══██╗██╔══██╗██╔═══██╗████╗  ██║
██║     ███████║██████╔╝██████╔╝██║   ██║██╔██╗ ██║
██║     ██╔══██║██╔══██╗██╔══██╗██║   ██║██║╚██╗██║
╚██████╗██║  ██║██║  ██║██████╔╝╚██████╔╝██║ ╚████║
 ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝  ╚═════╝ ╚═╝  ╚═══╝
                                                      
Generate decoders and scaffold indexers for your Solana programs.
    `;

    console.log(carbonGradient.multiline(banner));
}

/**
 * Logger class for styled console output
 */
export class Logger {
    private spinner: Ora | null = null;

    /**
     * Success message with checkmark
     */
    success(message: string, details?: Record<string, string>): void {
        console.log(chalk.green('✓') + ' ' + chalk.bold(message));
        if (details) {
            Object.entries(details).forEach(([key, value]) => {
                console.log(chalk.gray(`  • ${key}: `) + chalk.white(value));
            });
        }
    }

    /**
     * Error message with X mark
     */
    error(message: string): void {
        console.log(chalk.red('✗') + ' ' + chalk.bold.red(message));
    }

    /**
     * Warning message
     */
    warning(message: string): void {
        console.log(chalk.yellow('⚠') + ' ' + chalk.yellow(message));
    }

    /**
     * Info message
     */
    info(message: string): void {
        console.log(chalk.blue('ℹ') + ' ' + chalk.white(message));
    }

    /**
     * Start a spinner
     */
    startSpinner(message: string): void {
        this.spinner = ora({
            text: message,
            color: 'yellow',
            spinner: 'dots',
        }).start();
    }

    /**
     * Update spinner text
     */
    updateSpinner(message: string): void {
        if (this.spinner) {
            this.spinner.text = message;
        }
    }

    /**
     * Stop spinner with success
     */
    succeedSpinner(message?: string): void {
        if (this.spinner) {
            this.spinner.succeed(message);
            this.spinner = null;
        }
    }

    /**
     * Stop spinner with failure
     */
    failSpinner(message?: string): void {
        if (this.spinner) {
            this.spinner.fail(message);
            this.spinner = null;
        }
    }

    /**
     * Stop spinner without status
     */
    stopSpinner(): void {
        if (this.spinner) {
            this.spinner.stop();
            this.spinner = null;
        }
    }

    /**
     * Print a blank line
     */
    newLine(): void {
        console.log();
    }

    /**
     * Print a section header
     */
    section(title: string): void {
        console.log();
        console.log(chalk.bold.cyan('━'.repeat(50)));
        console.log(chalk.bold.cyan(`  ${title}`));
        console.log(chalk.bold.cyan('━'.repeat(50)));
    }

    /**
     * Print decoder generation success with gradient
     */
    decoderSuccess(outputPath: string, source: string): void {
        console.log();
        console.log(successGradient('✨ Decoder Generated Successfully! ✨'));
        console.log(chalk.gray('  • Output: ') + chalk.white(outputPath));
        console.log(chalk.gray('  • Source: ') + chalk.white(source));
        console.log();
    }

    /**
     * Print scaffold success with gradient
     */
    scaffoldSuccess(location: string, decoder: string): void {
        console.log();
        console.log(successGradient('✨ Scaffold Created Successfully! ✨'));
        console.log(chalk.gray('  • Location: ') + chalk.white(location));
        console.log(chalk.gray('  • Decoder: ') + chalk.white(decoder));
        console.log();
    }
}

// Export singleton instance
export const logger = new Logger();
