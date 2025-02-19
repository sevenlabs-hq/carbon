use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x15")]
pub struct GetAccountDataSize {}

pub struct GetAccountDataSizeInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetAccountDataSize {
    type ArrangedAccounts = GetAccountDataSizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(GetAccountDataSizeInstructionAccounts { mint: mint.pubkey })
    }
}
