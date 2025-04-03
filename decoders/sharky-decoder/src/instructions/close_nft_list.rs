use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x23087952da4efca2")]
pub struct CloseNftList {}

pub struct CloseNftListInstructionAccounts {
    pub nft_list: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseNftList {
    type ArrangedAccounts = CloseNftListInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nft_list, payer, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseNftListInstructionAccounts {
            nft_list: nft_list.pubkey,
            payer: payer.pubkey,
        })
    }
}
