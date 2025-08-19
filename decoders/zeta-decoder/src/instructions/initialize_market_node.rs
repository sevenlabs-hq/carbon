use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x32761515b3f81780")]
pub struct InitializeMarketNode {
    pub args: InitializeMarketNodeArgs,
}

pub struct InitializeMarketNodeInstructionAccounts {
    pub zeta_group: solana_pubkey::Pubkey,
    pub market_node: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMarketNode {
    type ArrangedAccounts = InitializeMarketNodeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [zeta_group, market_node, greeks, payer, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(InitializeMarketNodeInstructionAccounts {
            zeta_group: zeta_group.pubkey,
            market_node: market_node.pubkey,
            greeks: greeks.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
