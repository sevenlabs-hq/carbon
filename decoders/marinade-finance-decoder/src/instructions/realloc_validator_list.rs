use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd73bda855d8a3c7b")]
pub struct ReallocValidatorList {
    pub capacity: u32,
}

pub struct ReallocValidatorListInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin_authority: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
    pub rent_funds: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReallocValidatorList {
    type ArrangedAccounts = ReallocValidatorListInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            admin_authority,
            validator_list,
            rent_funds,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(ReallocValidatorListInstructionAccounts {
            state: state.pubkey,
            admin_authority: admin_authority.pubkey,
            validator_list: validator_list.pubkey,
            rent_funds: rent_funds.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
