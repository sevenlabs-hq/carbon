use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x10")]
pub struct InitializeAccount2 {
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct InitializeAccount2InstructionAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeAccount2 {
    type ArrangedAccounts = InitializeAccount2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeAccount2InstructionAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            rent: rent.pubkey,
        })
    }
}
