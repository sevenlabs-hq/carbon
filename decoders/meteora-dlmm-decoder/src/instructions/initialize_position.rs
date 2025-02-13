use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdbc0ea47bebf6650")]
pub struct InitializePosition {
    pub lower_bin_id: i32,
    pub width: i32,
}

pub struct InitializePositionInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePosition {
    type ArrangedAccounts = InitializePositionInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let position = accounts.get(1)?;
        let lb_pair = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let rent = accounts.get(5)?;
        let event_authority = accounts.get(6)?;
        let program = accounts.get(7)?;

        Some(InitializePositionInstructionAccounts {
            payer: payer.pubkey,
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
