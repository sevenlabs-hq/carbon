use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05")]
pub struct InitializeLpPosition {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeLpPositionInstructionAccounts {
    pub plasma_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub lp_position_owner: solana_pubkey::Pubkey,
    pub lp_position: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeLpPosition {
    type ArrangedAccounts = InitializeLpPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [plasma_program, log_authority, pool, payer, lp_position_owner, lp_position, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeLpPositionInstructionAccounts {
            plasma_program: plasma_program.pubkey,
            log_authority: log_authority.pubkey,
            pool: pool.pubkey,
            payer: payer.pubkey,
            lp_position_owner: lp_position_owner.pubkey,
            lp_position: lp_position.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
