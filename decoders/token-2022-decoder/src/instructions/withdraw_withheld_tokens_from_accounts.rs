use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1a")]
pub struct WithdrawWithheldTokensFromAccounts {
    pub transfer_fee_discriminator: u8,
    pub num_token_accounts: u8,
}

pub struct WithdrawWithheldTokensFromAccountsInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub fee_receiver: solana_sdk::pubkey::Pubkey,
    pub withdraw_withheld_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawWithheldTokensFromAccounts {
    type ArrangedAccounts = WithdrawWithheldTokensFromAccountsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, fee_receiver, withdraw_withheld_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(WithdrawWithheldTokensFromAccountsInstructionAccounts {
            mint: mint.pubkey,
            fee_receiver: fee_receiver.pubkey,
            withdraw_withheld_authority: withdraw_withheld_authority.pubkey,
        })
    }
}
