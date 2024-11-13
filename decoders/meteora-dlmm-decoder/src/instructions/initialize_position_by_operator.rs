use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfbbdbef475fe2394")]
pub struct InitializePositionByOperator {
    pub lower_bin_id: i32,
    pub width: i32,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_owner: solana_sdk::pubkey::Pubkey,
}

pub struct InitializePositionByOperatorInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub base: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub operator: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePositionByOperator {
    type ArrangedAccounts = InitializePositionByOperatorInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let base = accounts.get(1)?;
        let position = accounts.get(2)?;
        let lb_pair = accounts.get(3)?;
        let operator = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let rent = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(InitializePositionByOperatorInstructionAccounts {
            payer: payer.pubkey,
            base: base.pubkey,
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            operator: operator.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
