use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x93f17b64f484ae76")]
pub struct CreateTokenAccount {
    pub bump: u8,
}

pub struct CreateTokenAccountInstructionAccounts {
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateTokenAccount {
    type ArrangedAccounts = CreateTokenAccountInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_account = accounts.get(0)?;
        let user = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let token_program = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(CreateTokenAccountInstructionAccounts {
            token_account: token_account.pubkey,
            user: user.pubkey,
            mint: mint.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
