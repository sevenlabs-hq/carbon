use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x99a23254b6c94ab3")]
pub struct SetNewAccountAuthority {}

pub struct SetNewAccountAuthorityInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
    pub marginfi_group: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub new_authority: solana_pubkey::Pubkey,
    pub fee_payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetNewAccountAuthority {
    type ArrangedAccounts = SetNewAccountAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_account, marginfi_group, signer, new_authority, fee_payer, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetNewAccountAuthorityInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
            marginfi_group: marginfi_group.pubkey,
            signer: signer.pubkey,
            new_authority: new_authority.pubkey,
            fee_payer: fee_payer.pubkey,
        })
    }
}
