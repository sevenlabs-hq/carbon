use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3d9464468f6b110d")]
pub struct UpdateReserveConfig {
    pub mode: u64,
    pub value: Vec<u8>,
    pub skip_validation: bool,
}

pub struct UpdateReserveConfigInstructionAccounts {
    pub lending_market_owner: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateReserveConfig {
    type ArrangedAccounts = UpdateReserveConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market_owner, lending_market, reserve, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateReserveConfigInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
        })
    }
}
