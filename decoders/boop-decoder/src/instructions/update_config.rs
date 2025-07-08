use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1d9efcbf0a53db63")]
pub struct UpdateConfig {
    pub new_protocol_fee_recipient: solana_pubkey::Pubkey,
    pub new_virtual_sol_reserves: u64,
    pub new_virtual_token_reserves: u64,
    pub new_graduation_target: u64,
    pub new_graduation_fee: u64,
    pub new_damping_term: u8,
    pub new_swap_fee_basis_points: u8,
    pub new_token_for_stakers_basis_points: u16,
    pub new_token_amount_for_raydium_liquidity: u64,
    pub new_max_graduation_price_deviation_basis_points: u16,
    pub new_max_swap_amount_for_pool_price_correction_basis_points: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateConfigInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateConfig {
    type ArrangedAccounts = UpdateConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, authority, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateConfigInstructionAccounts {
            config: config.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
