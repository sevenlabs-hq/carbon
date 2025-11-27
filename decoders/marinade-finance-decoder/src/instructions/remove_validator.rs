use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1960d39ba10ea8bc")]
pub struct RemoveValidator {
    pub index: u32,
    pub validator_vote: solana_pubkey::Pubkey,
}

pub struct RemoveValidatorInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub manager_authority: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
    pub duplication_flag: solana_pubkey::Pubkey,
    pub operational_sol_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveValidator {
    type ArrangedAccounts = RemoveValidatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            manager_authority,
            validator_list,
            duplication_flag,
            operational_sol_account,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(RemoveValidatorInstructionAccounts {
            state: state.pubkey,
            manager_authority: manager_authority.pubkey,
            validator_list: validator_list.pubkey,
            duplication_flag: duplication_flag.pubkey,
            operational_sol_account: operational_sol_account.pubkey,
        })
    }
}
