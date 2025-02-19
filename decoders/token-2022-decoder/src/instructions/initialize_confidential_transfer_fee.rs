use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x25")]
pub struct InitializeConfidentialTransferFee {
    pub confidential_transfer_fee_discriminator: u8,
    pub authority: Option<solana_sdk::pubkey::Pubkey>,
    pub withdraw_withheld_authority_el_gamal_pubkey: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeConfidentialTransferFeeInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeConfidentialTransferFee {
    type ArrangedAccounts = InitializeConfidentialTransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeConfidentialTransferFeeInstructionAccounts { mint: mint.pubkey })
    }
}
