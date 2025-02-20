use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdaedaaf5278fa621")]
pub struct PostPythLazerOracleUpdate {
    pub pyth_message: Vec<u8>,
}

pub struct PostPythLazerOracleUpdateInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub pyth_lazer_storage: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PostPythLazerOracleUpdate {
    type ArrangedAccounts = PostPythLazerOracleUpdateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, pyth_lazer_storage, ix_sysvar, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PostPythLazerOracleUpdateInstructionAccounts {
            keeper: keeper.pubkey,
            pyth_lazer_storage: pyth_lazer_storage.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
