use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "IDL Parser CLI")]
#[command(about = "Generate Program Parsers for Carbon.")]
#[command(version = "0.1.0", author = "aimbot-labs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "parse")]
    #[command(about = "Generate code for Carbon around the submitted IDL.")]
    #[command(aliases = &["create", "generate"])]
    Parse(ParseOptions),
}

#[derive(Parser)]
pub struct ParseOptions {
    #[arg(short, long)]
    #[arg(help = "Path to the IDL json file.")]
    pub idl: Option<String>,

    #[arg(short, long)]
    #[arg(help = "Path for the generated code output.")]
    pub out: Option<String>,
}
