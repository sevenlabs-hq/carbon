use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x23087952da4efca2")]
pub struct CloseNftList {}

pub struct CloseNftListInstructionAccounts {
    pub nft_list: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseNftList {
    type ArrangedAccounts = CloseNftListInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let nft_list = accounts.get(0)?;
        let payer = accounts.get(1)?;

        Some(CloseNftListInstructionAccounts {
            nft_list: nft_list.pubkey,
            payer: payer.pubkey,
        })
    }
}
