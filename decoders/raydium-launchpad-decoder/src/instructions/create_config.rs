use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc9cff3724b6f2fbd")]
pub struct CreateConfig {
    pub curve_type: u8,
    pub index: u16,
    pub migrate_fee: u64,
    pub trade_fee_rate: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub quote_token_mint: solana_pubkey::Pubkey,
    pub protocol_fee_owner: solana_pubkey::Pubkey,
    pub migrate_fee_owner: solana_pubkey::Pubkey,
    pub migrate_to_amm_wallet: solana_pubkey::Pubkey,
    pub migrate_to_cpswap_wallet: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateConfig {
    type ArrangedAccounts = CreateConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, global_config, quote_token_mint, protocol_fee_owner, migrate_fee_owner, migrate_to_amm_wallet, migrate_to_cpswap_wallet, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateConfigInstructionAccounts {
            owner: owner.pubkey,
            global_config: global_config.pubkey,
            quote_token_mint: quote_token_mint.pubkey,
            protocol_fee_owner: protocol_fee_owner.pubkey,
            migrate_fee_owner: migrate_fee_owner.pubkey,
            migrate_to_amm_wallet: migrate_to_amm_wallet.pubkey,
            migrate_to_cpswap_wallet: migrate_to_cpswap_wallet.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
