use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x886c58f5e6e06552")]
pub struct InitializeCombinedSocializedLossAccount {
    pub nonce: u8,
}

pub struct InitializeCombinedSocializedLossAccountInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub socialized_loss_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub usdc_mint: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCombinedSocializedLossAccount {
    type ArrangedAccounts = InitializeCombinedSocializedLossAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            socialized_loss_account,
            token_program,
            usdc_mint,
            admin,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(InitializeCombinedSocializedLossAccountInstructionAccounts {
            state: state.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
            token_program: token_program.pubkey,
            usdc_mint: usdc_mint.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
