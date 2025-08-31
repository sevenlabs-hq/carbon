use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00c9e2e47f2d456e")]
pub struct ClaimStandardCreatorTradingFeeProtocolFees {
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimStandardCreatorTradingFeeProtocolFeesInstructionAccounts {
    pub swap: solana_pubkey::Pubkey,
    pub protocol_admin: solana_pubkey::Pubkey,
    pub protocol_admin_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimStandardCreatorTradingFeeProtocolFees {
    type ArrangedAccounts = ClaimStandardCreatorTradingFeeProtocolFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [swap, protocol_admin, protocol_admin_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(
            ClaimStandardCreatorTradingFeeProtocolFeesInstructionAccounts {
                swap: swap.pubkey,
                protocol_admin: protocol_admin.pubkey,
                protocol_admin_state: protocol_admin_state.pubkey,
            },
        )
    }
}
