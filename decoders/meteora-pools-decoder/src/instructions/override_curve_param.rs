use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6256cc335e4745bb")]
pub struct OverrideCurveParam {
    pub curve_type: CurveType,
}

pub struct OverrideCurveParamInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OverrideCurveParam {
    type ArrangedAccounts = OverrideCurveParamInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(OverrideCurveParamInstructionAccounts {
            pool: pool.pubkey,
            admin: admin.pubkey,
        })
    }
}
