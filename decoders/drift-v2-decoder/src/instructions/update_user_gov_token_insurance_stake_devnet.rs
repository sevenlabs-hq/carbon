use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x81b9f3b7e46f40af")]
pub struct UpdateUserGovTokenInsuranceStakeDevnet {
    pub gov_stake_amount: u64,
}

pub struct UpdateUserGovTokenInsuranceStakeDevnetInstructionAccounts {
    pub user_stats: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserGovTokenInsuranceStakeDevnet {
    type ArrangedAccounts = UpdateUserGovTokenInsuranceStakeDevnetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user_stats, signer, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserGovTokenInsuranceStakeDevnetInstructionAccounts {
            user_stats: user_stats.pubkey,
            signer: signer.pubkey,
        })
    }
}
