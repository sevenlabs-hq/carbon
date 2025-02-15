use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2e2729876fb7c840")]
pub struct InitializeCustomizablePermissionlessLbPair {
    pub params: CustomizableParams,
}

pub struct InitializeCustomizablePermissionlessLbPairInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub token_mint_x: solana_sdk::pubkey::Pubkey,
    pub token_mint_y: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub user_token_x: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCustomizablePermissionlessLbPair {
    type ArrangedAccounts = InitializeCustomizablePermissionlessLbPairInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let bin_array_bitmap_extension = accounts.get(1)?;
        let token_mint_x = accounts.get(2)?;
        let token_mint_y = accounts.get(3)?;
        let reserve_x = accounts.get(4)?;
        let reserve_y = accounts.get(5)?;
        let oracle = accounts.get(6)?;
        let user_token_x = accounts.get(7)?;
        let funder = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let rent = accounts.get(11)?;
        let event_authority = accounts.get(12)?;
        let program = accounts.get(13)?;

        Some(
            InitializeCustomizablePermissionlessLbPairInstructionAccounts {
                lb_pair: lb_pair.pubkey,
                bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
                token_mint_x: token_mint_x.pubkey,
                token_mint_y: token_mint_y.pubkey,
                reserve_x: reserve_x.pubkey,
                reserve_y: reserve_y.pubkey,
                oracle: oracle.pubkey,
                user_token_x: user_token_x.pubkey,
                funder: funder.pubkey,
                token_program: token_program.pubkey,
                system_program: system_program.pubkey,
                rent: rent.pubkey,
                event_authority: event_authority.pubkey,
                program: program.pubkey,
            },
        )
    }
}
