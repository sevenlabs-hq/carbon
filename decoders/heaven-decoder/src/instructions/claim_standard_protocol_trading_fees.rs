use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x54ce8cf53fd440ed")]
pub struct ClaimStandardProtocolTradingFees {
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimStandardProtocolTradingFeesInstructionAccounts {
    pub swap: solana_pubkey::Pubkey,
    pub protocol_admin: solana_pubkey::Pubkey,
    pub protocol_admin_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimStandardProtocolTradingFees {
    type ArrangedAccounts = ClaimStandardProtocolTradingFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [swap, protocol_admin, protocol_admin_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ClaimStandardProtocolTradingFeesInstructionAccounts {
            swap: swap.pubkey,
            protocol_admin: protocol_admin.pubkey,
            protocol_admin_state: protocol_admin_state.pubkey,
        })
    }
}
