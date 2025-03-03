use {
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

use commands::Url;
use inquire::{error::InquireResult, required, Confirm, CustomType, InquireError, Select, Text};

// carbon-cli parse --idl my_program.json --output ./src/decoders
fn main() -> InquireResult<()> {
    match Cli::try_parse() {
        Ok(cli) => process_cli_params(cli),
        Err(_) => process_prompts(),
    }
}

fn process_prompts() -> InquireResult<()> {
    let idl_source = Select::new("IDL source:", vec!["file", "program address"]).prompt()?;

    match idl_source {
        "file" => {
            let path = Text::new("Path to IDL file:")
                .with_validator(required!("Please type a path to IDL"))
                .prompt()?;
            let standard = Select::new(
                "Standard of program:",
                vec![IdlStandard::Anchor, IdlStandard::Codama],
            )
            .prompt()?;
            match standard {
                IdlStandard::Anchor => {
                    let output_dir = Text::new("Output directory:")
                        .with_validator(required!("Please type a path to output folder"))
                        .prompt()?;
                    let as_crate = Confirm::new("Generate as crate?").prompt()?;

                    handlers::parse(path, output_dir, as_crate)
                        .map_err(|e| InquireError::Custom(e.into()))?;
                }
                IdlStandard::Codama => {
                    let event_hints = Text::new("Event hints:")
                        .with_validator(required!("Please provide comma-separated event hints"))
                        .prompt()?;

                    let output_dir = Text::new("Output directory:")
                        .with_validator(required!("Please type a path to output folder"))
                        .prompt()?;
                    let as_crate = Confirm::new("Generate as crate?").prompt()?;
                    handlers::parse_codama(path, output_dir, as_crate, Some(event_hints))
                        .map_err(|e| InquireError::Custom(e.into()))?;
                }
            }
        }
        "program address" => {
            let program_address = Text::new("Program address:").prompt()?;
            let url = CustomType::<Url>::new("Network URL:").prompt()?;
            let output_dir = Text::new("Output directory:")
                .with_validator(required!("Please type a path to output folder"))
                .prompt()?;
            let as_crate = Confirm::new("Generate as crate?").prompt()?;

            handlers::process_pda_idl(program_address, &url, output_dir, as_crate)
                .map_err(|e| InquireError::Custom(e.into()))?;
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
