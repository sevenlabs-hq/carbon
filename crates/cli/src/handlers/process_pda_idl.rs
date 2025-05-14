use {
    crate::{commands::Url, handlers},
    anyhow::{Context, Result},
    borsh::BorshDeserialize,
    flate2::read::ZlibDecoder,
    solana_client::rpc_client::RpcClient,
    solana_commitment_config::CommitmentConfig,
    solana_pubkey::Pubkey,
    std::{fs, io::prelude::*, path::Path, str::FromStr},
};

pub fn process_pda_idl(
    program_address: String,
    url: &Url,
    output: String,
    as_crate: bool,
) -> Result<()> {
    let rpc_url = match url {
        Url::Mainnet => "https://api.mainnet-beta.solana.com",
        Url::Devnet => "https://api.devnet.solana.com",
        Url::CustomRpc(custom_url) => custom_url,
    };

    let program_address_pubkey =
        Pubkey::from_str(&program_address).context("Couldn't parse program address from string")?;

    println!(
        "Fetching IDL for program: {} from {}",
        program_address, rpc_url
    );

    let idl = fetch_idl(program_address_pubkey, rpc_url.to_string())
        .map(|idl| serde_json::to_string_pretty(&idl))
        .context("Couldn't fetch Program Idl")??;

    let idl_path = format!("./{}_idl.json", program_address);

    fs::write(&idl_path, idl)?;

    handlers::parse(idl_path.clone(), output, as_crate).context("Couldn't parse IDL")?;

    // Clean up: Delete the IDL file after parsing
    if Path::new(&idl_path).exists() {
        fs::remove_file(&idl_path).context("Failed to delete temporary IDL file")?;
    }

    Ok(())
}

fn fetch_idl(program_address: Pubkey, rpc_url: String) -> Result<serde_json::Value> {
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let mut account = client.get_account(&program_address)?;
    if account.executable {
        let program_signer = Pubkey::find_program_address(&[], &program_address).0;
        let idl_address = Pubkey::create_with_seed(&program_signer, "anchor:idl", &program_address)
            .context("Seed is always valid")?;
        account = client.get_account(&idl_address)?;
    }

    // Cut off account discriminator.
    let mut d: &[u8] = &account.data[8..];
    let idl_account: IdlAccount = BorshDeserialize::deserialize(&mut d)?;

    let compressed_len = idl_account.data_len as usize;
    let compressed_bytes = &account.data[44..44 + compressed_len];
    let mut z = ZlibDecoder::new(compressed_bytes);
    let mut s = Vec::new();
    z.read_to_end(&mut s)?;
    serde_json::from_slice(&s[..]).map_err(Into::into)
}

#[derive(Debug, BorshDeserialize)]
pub struct IdlAccount {
    // Address that can modify the IDL.
    pub authority: Pubkey,
    // Length of compressed idl bytes.
    pub data_len: u32,
}
