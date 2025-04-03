use solana_pubkey::Pubkey;

pub struct FluxbeamDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X");
