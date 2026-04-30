#![allow(dead_code)]

use {
    carbon_helius_gpa_v2_datasource::{HeliusGpaV2Config, HeliusGpaV2Datasource},
    carbon_rpc_gpa_datasource::GpaDatasource,
    solana_pubkey::Pubkey,
    std::env,
};

pub fn rpc(program: Pubkey) -> GpaDatasource {
    GpaDatasource::new(env::var("RPC_URL").expect("RPC_URL must be set"), program)
}

pub fn helius_gpa_v2(program: Pubkey) -> HeliusGpaV2Datasource {
    let config = HeliusGpaV2Config::new(None, None, Some(1000));

    HeliusGpaV2Datasource::new_with_config(
        env::var("HELIUS_RPC_URL").expect("HELIUS_RPC_URL must be set"),
        program,
        config,
    )
}
