use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf54b5b00ec611303")]
pub struct SocializeLoss {
    pub liquidity_amount: u64,
}

pub struct SocializeLossInstructionAccounts {
    pub risk_council: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SocializeLoss {
    type ArrangedAccounts = SocializeLossInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [risk_council, obligation, lending_market, reserve, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SocializeLossInstructionAccounts {
            risk_council: risk_council.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}
