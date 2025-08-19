use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1a88e1d916155314")]
pub struct IncreasePositionPreSwap {
    pub params: IncreasePositionPreSwapParams,
}

pub struct IncreasePositionPreSwapInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub keeper_ata: solana_pubkey::Pubkey,
    pub position_request: solana_pubkey::Pubkey,
    pub position_request_ata: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub collateral_custody: solana_pubkey::Pubkey,
    pub collateral_custody_token_account: solana_pubkey::Pubkey,
    pub instruction: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreasePositionPreSwap {
    type ArrangedAccounts = IncreasePositionPreSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, keeper_ata, position_request, position_request_ata, position, collateral_custody, collateral_custody_token_account, instruction, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(IncreasePositionPreSwapInstructionAccounts {
            keeper: keeper.pubkey,
            keeper_ata: keeper_ata.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            position: position.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            instruction: instruction.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
