use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08")]
pub struct WithdrawSrm {
    pub amount: u64,
}

pub struct WithdrawSrmInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_owner_account: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub srm_token: solana_sdk::pubkey::Pubkey,
    pub dest_srm_token: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawSrm {
    type ArrangedAccounts = WithdrawSrmInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let amm = accounts.get(1)?;
        let amm_owner_account = accounts.get(2)?;
        let amm_authority = accounts.get(3)?;
        let srm_token = accounts.get(4)?;
        let dest_srm_token = accounts.get(5)?;

        Some(WithdrawSrmInstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_owner_account: amm_owner_account.pubkey,
            amm_authority: amm_authority.pubkey,
            srm_token: srm_token.pubkey,
            dest_srm_token: dest_srm_token.pubkey,
        })
    }
}
