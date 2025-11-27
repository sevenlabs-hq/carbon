use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c247c1b806055c7")]
pub struct ReallocStakeList {
    pub capacity: u32,
}

pub struct ReallocStakeListInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin_authority: solana_pubkey::Pubkey,
    pub stake_list: solana_pubkey::Pubkey,
    pub rent_funds: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReallocStakeList {
    type ArrangedAccounts = ReallocStakeListInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            admin_authority,
            stake_list,
            rent_funds,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(ReallocStakeListInstructionAccounts {
            state: state.pubkey,
            admin_authority: admin_authority.pubkey,
            stake_list: stake_list.pubkey,
            rent_funds: rent_funds.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
