use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x20fc7ad3421f2ff1")]
pub struct UpdateDiscountMint {
    pub discount_mint: solana_pubkey::Pubkey,
}

pub struct UpdateDiscountMintInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateDiscountMint {
    type ArrangedAccounts = UpdateDiscountMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateDiscountMintInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
