use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe8fdc3f794d449de")]
pub struct UpdateFee {
    pub maker_fee: u64,
    pub maker_stable_fee: u64,
    pub taker_fee: u64,
    pub taker_stable_fee: u64,
}

pub struct UpdateFeeInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFee {
    type ArrangedAccounts = UpdateFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, fee_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateFeeInstructionAccounts {
            keeper: keeper.pubkey,
            fee_authority: fee_authority.pubkey,
        })
    }
}
