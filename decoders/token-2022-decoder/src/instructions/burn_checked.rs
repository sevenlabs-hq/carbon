use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0f")]
pub struct BurnChecked {
    pub amount: u64,
    pub decimals: u8,
}

pub struct BurnCheckedInstructionAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnChecked {
    type ArrangedAccounts = BurnCheckedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(BurnCheckedInstructionAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
