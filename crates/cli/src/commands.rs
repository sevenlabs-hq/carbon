use {
    clap::{Parser, Subcommand, ValueEnum},
    std::{fmt, str::FromStr},
};

#[derive(Parser)]
#[command(name = "IDL Parser CLI")]
#[command(about = "Generate Program Parsers for Carbon.")]
#[command(version = "0.9.1", author = "aimbot-labs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "parse")]
    #[command(about = "Generate code for Carbon around the submitted IDL.")]
    Parse(ParseOptions),
    #[command(name = "scaffold")]
    #[command(about = "Generate skeleton of the project.")]
    Scaffold(ScaffoldOptions),
}

#[derive(Parser)]
pub struct ParseOptions {
    #[arg(short, long, required = true)]
    #[arg(help = "Path to an IDL json file or a Solana program address.")]
    pub idl: IdlSource,

    #[arg(short, long, required = true)]
    #[arg(help = "Path to the desired output directory.")]
    pub output: String,

    #[arg(short = 'c', long = "as-crate", default_value_t = false)]
    #[arg(help = "Generate a directory or a crate.")]
    pub as_crate: bool,

    #[arg(short, long = "standard", default_value = "anchor")]
    #[arg(help = "Specify the IDL standard to parse.")]
    pub standard: IdlStandard,

    #[arg(short, long)]
    #[arg(help = "Comma-separated names of defined types to parse as CPI Events.")]
    pub event_hints: Option<String>,

    #[arg(short, long, required_if_eq("idl", "ProgramAddress"))]
    #[arg(help = "Network URL to fetch the IDL from. Required if input is a program address.")]
    pub url: Option<Url>,
}

#[derive(Parser)]
pub struct ScaffoldOptions {
    #[arg(short, long, required = true)]
    #[arg(help = "Name of your project.")]
    pub name: String,

    #[arg(short, long, required = true)]
    #[arg(help = "Path to the desired output directory.")]
    pub output: String,

    #[arg(short = 'd', long, required = true)]
    #[arg(help = "Comma-separated names of decoders.")]
    pub decoders: String,

    #[arg(short = 's', long, required = true)]
    #[arg(help = "Name of data source.")]
    pub data_source: String,

    #[arg(short = 'm', long, default_value = "log")]
    #[arg(help = "Metrics to use.")]
    pub metrics: String,
}

#[derive(Clone, Debug)]
pub enum IdlSource {
    FilePath(String),
    ProgramAddress(String),
}

impl std::str::FromStr for IdlSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with(".json") {
            Ok(IdlSource::FilePath(s.to_string()))
        } else if s.len() <= 44 && s.len() >= 32 && s.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(IdlSource::ProgramAddress(s.to_string()))
        } else {
            Err("Invalid input: Must be either a valid file path (ending in .json) or a valid Solana program address.".to_string())
        }
    }
}

#[derive(Clone, Debug)]
pub enum Url {
    Mainnet,
    Devnet,
    CustomRpc(String),
}

impl FromStr for Url {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet-beta" => Ok(Url::Mainnet),
            "devnet" => Ok(Url::Devnet),
            _ if s.starts_with("http") => Ok(Url::CustomRpc(s.to_string())),
            _ => {
                Err("Invalid network: Must be 'mainnet', 'devnet', or a valid RPC URL.".to_string())
            }
        }
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Url::Mainnet => write!(f, "mainnet-beta"),
            Url::Devnet => write!(f, "devnet"),
            Url::CustomRpc(ref url) => write!(f, "{url}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IdlStandard {
    Anchor,
    Codama,
}

impl fmt::Display for IdlStandard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IdlStandard::Anchor => write!(f, "anchor"),
            IdlStandard::Codama => write!(f, "codama"),
        }
    }
}

impl std::str::FromStr for IdlStandard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "anchor" => Ok(IdlStandard::Anchor),
            "codama" => Ok(IdlStandard::Codama),
            _ => Err("Invalid Idl Standard: Must be 'anchor' or 'codama'.".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Datasource {
    HeliusAtlasWs,
    RpcBlockSubscribe,
    RpcProgramSubscribe,
    RpcTransactionCrawler,
    YellowstoneGrpc,
}

impl fmt::Display for Datasource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Datasource::HeliusAtlasWs => write!(f, "helius-atlas-ws"),
            Datasource::RpcBlockSubscribe => write!(f, "rpc-block-subscribe"),
            Datasource::RpcProgramSubscribe => write!(f, "rpc-program-subscribe"),
            Datasource::RpcTransactionCrawler => write!(f, "rpc-transaction-crawler"),
            Datasource::YellowstoneGrpc => write!(f, "yellowstone-grpc"),
        }
    }
}

impl std::str::FromStr for Datasource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "helius-atlas-ws" => Ok(Datasource::HeliusAtlasWs),
            "rpc-block-subscribe" => Ok(Datasource::RpcBlockSubscribe),
            "rpc-program-subscribe" => Ok(Datasource::RpcProgramSubscribe),
            "rpc-transaction-crawler" => Ok(Datasource::RpcTransactionCrawler),
            "yellowstone-grpc" => Ok(Datasource::YellowstoneGrpc),
            _ => Err("Invalid Datasource".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Metrics {
    Log,
    Prometheus,
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Metrics::Log => write!(f, "log"),
            Metrics::Prometheus => write!(f, "prometheus"),
        }
    }
}

impl std::str::FromStr for Metrics {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "log" => Ok(Metrics::Log),
            "prometheus" => Ok(Metrics::Prometheus),
            _ => Err("Invalid Metrics".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Decoder {
    Drift,
    Fluxbeam,
    JupiterDCA,
    JupiterLimitOrder,
    JupiterLimitOrder2,
    JupiterPerpetuals,
    JupiterSwap,
    KaminoLending,
    KaminoVault,
    LifinityAMM,
    MemoProgram,
    MeteoraDLMM,
    Moonshot,
    MPLCore,
    MPLTokenMetadata,
    NameService,
    OKXDEX,
    Openbook,
    OrcaWhirlpool,
    Phoenix,
    Pumpfun,
    RaydiumAMM,
    RaydiumCLMM,
    RaydiumCPMM,
    RaydiumLiquidityLocking,
    Sharky,
    SPLAssociatedTokenAccount,
    StabbleStableSwap,
    StabbleWeightedSwap,
    StakeProgram,
    SystemProgram,
    TokenProgram,
    Token2022Program,
    Zeta,
}

impl fmt::Display for Decoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Decoder::Drift => write!(f, "drift-v2"),
            Decoder::Fluxbeam => write!(f, "fluxbeam"),
            Decoder::JupiterDCA => write!(f, "jupiter-dca"),
            Decoder::JupiterLimitOrder => write!(f, "jupiter-limit-order"),
            Decoder::JupiterLimitOrder2 => write!(f, "jupiter-limit-order-2"),
            Decoder::JupiterPerpetuals => write!(f, "jupiter-perpetuals"),
            Decoder::JupiterSwap => write!(f, "jupiter-swap"),
            Decoder::KaminoLending => write!(f, "kamino-lending"),
            Decoder::KaminoVault => write!(f, "kamino-vault"),
            Decoder::LifinityAMM => write!(f, "lifinity-amm-v2"),
            Decoder::MemoProgram => write!(f, "memo-program"),
            Decoder::MeteoraDLMM => write!(f, "meteora-dlmm"),
            Decoder::Moonshot => write!(f, "moonshot"),
            Decoder::MPLCore => write!(f, "mpl-core"),
            Decoder::MPLTokenMetadata => write!(f, "mpl-token-metadata"),
            Decoder::NameService => write!(f, "name-service"),
            Decoder::OKXDEX => write!(f, "okx-dex"),
            Decoder::Openbook => write!(f, "openbook-v2"),
            Decoder::OrcaWhirlpool => write!(f, "orca-whirlpool"),
            Decoder::Phoenix => write!(f, "phoenix-v1"),
            Decoder::Pumpfun => write!(f, "pumpfun"),
            Decoder::RaydiumAMM => write!(f, "raydium-amm-v4"),
            Decoder::RaydiumCLMM => write!(f, "raydium-clmm"),
            Decoder::RaydiumCPMM => write!(f, "raydium-cpmm"),
            Decoder::RaydiumLiquidityLocking => write!(f, "raydium-liquidity-locking"),
            Decoder::Sharky => write!(f, "sharky"),
            Decoder::SPLAssociatedTokenAccount => write!(f, "spl-associated-token-account"),
            Decoder::StabbleStableSwap => write!(f, "stabble-stable-swap"),
            Decoder::StabbleWeightedSwap => write!(f, "stabble-weighted-swap"),
            Decoder::StakeProgram => write!(f, "stake-program"),
            Decoder::SystemProgram => write!(f, "system-program"),
            Decoder::TokenProgram => write!(f, "token-program"),
            Decoder::Token2022Program => write!(f, "token-2022-program"),
            Decoder::Zeta => write!(f, "zeta-program"),
        }
    }
}

impl std::str::FromStr for Decoder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "drift-v2" => Ok(Decoder::Drift),
            "fluxbeam" => Ok(Decoder::Fluxbeam),
            "jupiter-dca" => Ok(Decoder::JupiterDCA),
            "jupiter-limit-order" => Ok(Decoder::JupiterLimitOrder),
            "jupiter-limit-order-2" => Ok(Decoder::JupiterLimitOrder2),
            "jupiter-swap" => Ok(Decoder::JupiterSwap),
            "kamino-lending" => Ok(Decoder::KaminoLending),
            "kamino-vault" => Ok(Decoder::KaminoVault),
            "lifinity-amm-v2" => Ok(Decoder::LifinityAMM),
            "memo-program" => Ok(Decoder::MemoProgram),
            "meteora-dlmm" => Ok(Decoder::MeteoraDLMM),
            "moonshot" => Ok(Decoder::Moonshot),
            "mpl-core" => Ok(Decoder::MPLCore),
            "mpl-token-metadata" => Ok(Decoder::MPLTokenMetadata),
            "name-service" => Ok(Decoder::NameService),
            "okx-dex" => Ok(Decoder::OKXDEX),
            "openbook-v2" => Ok(Decoder::Openbook),
            "orca-whirlpool" => Ok(Decoder::OrcaWhirlpool),
            "phoenix-v1" => Ok(Decoder::Phoenix),
            "pumpfun" => Ok(Decoder::Pumpfun),
            "raydium-amm-v4" => Ok(Decoder::RaydiumAMM),
            "raydium-clmm" => Ok(Decoder::RaydiumCLMM),
            "raydium-cpmm" => Ok(Decoder::RaydiumCPMM),
            "raydium-liquidity-locking" => Ok(Decoder::RaydiumLiquidityLocking),
            "sharky" => Ok(Decoder::Sharky),
            "spl-associated-token-account" => Ok(Decoder::SPLAssociatedTokenAccount),
            "stabble-stable-swap" => Ok(Decoder::StabbleStableSwap),
            "stabble-weighted-swap" => Ok(Decoder::StabbleWeightedSwap),
            "stake-program" => Ok(Decoder::StakeProgram),
            "system-program" => Ok(Decoder::SystemProgram),
            "token-program" => Ok(Decoder::TokenProgram),
            "token-2022-program" => Ok(Decoder::Token2022Program),
            "zeta" => Ok(Decoder::Zeta),
            _ => Err("Invalid Decoder".to_string()),
        }
    }
}
