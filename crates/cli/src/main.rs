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
pub mod project;
pub mod types;
pub mod util;

use {
    commands::{Datasource, Decoder, Metrics, Url},
    inquire::{
        error::InquireResult, required, Confirm, CustomType, InquireError, MultiSelect, Select,
        Text,
    },
};

fn main() -> InquireResult<()> {
    match Cli::try_parse() {
        Ok(cli) => process_cli_params(cli),
        Err(_) => process_prompts(),
    }
}

fn process_prompts() -> InquireResult<()> {
    let cmd = Select::new("Chose mode:", vec!["parse", "scaffold"]).prompt()?;

    match cmd {
        "parse" => {
            let idl_source =
                Select::new("IDL source:", vec!["file", "program address"]).prompt()?;
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
                                .with_validator(required!(
                                    "Please provide comma-separated event hints"
                                ))
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
        }
        "scaffold" => {
            let name = Text::new("project name:")
                .with_validator(required!("Please type a project name"))
                .prompt()?;

            let output_dir = Text::new("Output directory:")
                .with_validator(required!("Please type a path to output folder"))
                .prompt()?;

            let available_decoders = vec![
                Decoder::Drift,
                Decoder::Fluxbeam,
                Decoder::JupiterDCA,
                Decoder::JupiterLimitOrder,
                Decoder::JupiterLimitOrder2,
                Decoder::JupiterPerpetuals,
                Decoder::JupiterSwap,
                Decoder::KaminoLending,
                Decoder::KaminoVault,
                Decoder::LifinityAMM,
                Decoder::MemoProgram,
                Decoder::MeteoraDLMM,
                Decoder::Moonshot,
                Decoder::MPLCore,
                Decoder::MPLTokenMetadata,
                Decoder::NameService,
                Decoder::OKXDEX,
                Decoder::Openbook,
                Decoder::OrcaWhirlpool,
                Decoder::Phoenix,
                Decoder::Pumpfun,
                Decoder::RaydiumAMM,
                Decoder::RaydiumCLMM,
                Decoder::RaydiumCPMM,
                Decoder::RaydiumLiquidityLocking,
                Decoder::Sharky,
                Decoder::SPLAssociatedTokenAccount,
                Decoder::StabbleStableSwap,
                Decoder::StabbleWeightedSwap,
                Decoder::StakeProgram,
                Decoder::SystemProgram,
                Decoder::TokenProgram,
                Decoder::Token2022Program,
                Decoder::Zeta,
            ];

            let datasource = Select::new(
                "select a datasource:",
                vec![
                    Datasource::HeliusAtlasWs,
                    Datasource::RpcBlockSubscribe,
                    Datasource::RpcProgramSubscribe,
                    Datasource::RpcTransactionCrawler,
                    Datasource::YellowstoneGrpc,
                ],
            )
            .prompt()?;

            let decoders =
                MultiSelect::new("Select the decoders for your app:", available_decoders)
                    .prompt()?;

            let metrics =
                Select::new("Select metrics:", vec![Metrics::Log, Metrics::Prometheus]).prompt()?;
            handlers::scaffold(
                name,
                output_dir,
                decoders
                    .into_iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                datasource.to_string(),
                metrics.to_string(),
            )
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
        Commands::Scaffold(options) => {
            handlers::scaffold(
                options.name,
                options.output,
                options.decoders,
                options.data_source,
                options.metrics,
            )
            .map_err(|e| InquireError::Custom(e.into()))?;
        }
    };

    Ok(())
}
