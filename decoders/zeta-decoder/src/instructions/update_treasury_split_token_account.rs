use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0b4ee9b8a2995dcf")]
pub struct UpdateTreasurySplitTokenAccount {
    pub treasury_split_percentage: u8,
}

pub struct UpdateTreasurySplitTokenAccountInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub treasury_split_token_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateTreasurySplitTokenAccount {
    type ArrangedAccounts = UpdateTreasurySplitTokenAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, treasury_split_token_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTreasurySplitTokenAccountInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            treasury_split_token_account: treasury_split_token_account.pubkey,
        })
    }
}
