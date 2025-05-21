use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x66063d1201daebea")]
pub struct Buy {
    pub amount: u64,
    pub max_sol_cost: u64,
}

#[derive(Debug, PartialEq)]
pub struct BuyInstructionAccounts {
    pub global: solana_pubkey::Pubkey,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub associated_bonding_curve: solana_pubkey::Pubkey,
    pub associated_user: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub creator_vault: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Buy {
    type ArrangedAccounts = BuyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global, fee_recipient, mint, bonding_curve, associated_bonding_curve, associated_user, user, system_program, token_program, creator_vault, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(BuyInstructionAccounts {
            global: global.pubkey,
            fee_recipient: fee_recipient.pubkey,
            mint: mint.pubkey,
            bonding_curve: bonding_curve.pubkey,
            associated_bonding_curve: associated_bonding_curve.pubkey,
            associated_user: associated_user.pubkey,
            user: user.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            creator_vault: creator_vault.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
