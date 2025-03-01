use {
    anyhow::Context,
    clap::Parser,
    commands::{Cli, Commands, IdlSource, IdlStandard},
};

pub mod accounts;
pub mod commands;
pub mod events;
pub mod handlers;
pub mod idl;
pub mod instructions;
pub mod legacy_idl;
pub mod types;
pub mod util;

use inquire::{
    error::InquireResult, required, Confirm, CustomType, InquireError, MultiSelect, Select, Text,
};

// carbon-cli parse --idl my_program.json --output ./src/decoders
fn main() -> InquireResult<()> {
    match Cli::try_parse() {
        Ok(cli) => process_cli_params(cli),
        Err(_) => process_prompts(),
    }
}

fn process_prompts() -> InquireResult<()> {
    println!("run prompts");
    println!("--idl path");
    println!("standard anchor/codama");
    println!("address + url");
    println!("as_crate");
    println!("--output path");
    let idl_source = Select::new("IDL source:", vec!["file", "program address"]).prompt()?;

    match idl_source {
        "file" => {
            let path = Text::new("Path to IDL file:").prompt()?;
            let standard = Select::new(
                "Standard of program:",
                vec![IdlStandard::Anchor, IdlStandard::Codama],
            )
            .prompt()?;
            let output = Text::new("Output directory:").prompt()?;
            let event_hints = Text::new("Event hints:").prompt()?;
            let as_crate = Confirm::new("Generate as crate?").prompt()?;
        }
        "program address" => {
            let program_address = Text::new("Program address:").prompt()?;
            let url = Text::new("Network URL:").prompt()?;
            let output = Text::new("Output directory:").prompt()?;
            let as_crate = Confirm::new("Generate as crate?").prompt()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn process_cli_params(cli: Cli) -> InquireResult<()> {
    match cli.command {
        Commands::Parse(options) => match options.idl {
            IdlSource::FilePath(path) => match options.standard {
                IdlStandard::Codama => {
                    handlers::parse_codama(
                        path,
                        options.output,
                        options.as_crate,
                        options.event_hints,
                    )
                    .map_err(|e| InquireError::Custom(e.into()))?;
                }
                IdlStandard::Anchor => {
                    if options.event_hints.is_some() {
                        return Err(InquireError::InvalidConfiguration(
                            "The '--event-hints' option can only be used with --codama."
                                .to_string(),
                        ));
                    }
                    handlers::parse(path, options.output, options.as_crate)
                        .map_err(|e| InquireError::Custom(e.into()))?;
                }
            },
            IdlSource::ProgramAddress(program_address) => {
                let url = options
                    .url
                    .as_ref()
                    .ok_or(InquireError::InvalidConfiguration(
                        "Network URL (--url / -u) argument is required when parsing an IDL from a program address."
                            .to_string(),
                    ))?;

                handlers::process_pda_idl(program_address, url, options.output, options.as_crate)
                    .map_err(|e| InquireError::Custom(e.into()))?;
            }
        },
    };

    Ok(())
}
