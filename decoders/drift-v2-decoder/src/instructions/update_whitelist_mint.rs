use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa10fa21394789097")]
pub struct UpdateWhitelistMint {
    pub whitelist_mint: solana_sdk::pubkey::Pubkey,
}

pub struct UpdateWhitelistMintInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateWhitelistMint {
    type ArrangedAccounts = UpdateWhitelistMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateWhitelistMintInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
