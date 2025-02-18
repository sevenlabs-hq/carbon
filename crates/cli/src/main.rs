use anyhow::Context;
use clap::Parser;
use commands::{Cli, Commands, IdlSource};

pub mod accounts;
pub mod commands;
pub mod events;
pub mod handlers;
pub mod idl;
pub mod instructions;
pub mod legacy_idl;
pub mod types;
pub mod util;

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse(options) => match options.idl {
            IdlSource::FilePath(path) => {
                if options.codama {
                    handlers::parse_codama(
                        path,
                        options.output,
                        options.as_crate,
                        options.event_hints,
                    )?;
                } else {
                    if options.event_hints.is_some() {
                        anyhow::bail!("The '--event-hints' option can only be used with --codama.");
                    }
                    handlers::parse(path, options.output, options.as_crate)?;
                }
            }
            IdlSource::ProgramAddress(program_address) => {
                let url = options
                    .url
                    .as_ref()
                    .context("Network URL (--url / -u) argument is required when parsing an IDL from a program address.")?;

                handlers::process_pda_idl(program_address, url, options.output, options.as_crate)?;
            }
        },
    };

    Ok(())
}
