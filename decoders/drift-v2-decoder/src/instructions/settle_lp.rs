use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9be7747161e58b8d")]
pub struct SettleLp {
    pub market_index: u16,
}

pub struct SettleLpInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleLp {
    type ArrangedAccounts = SettleLpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SettleLpInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
        })
    }
}
