use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3657a51345e3dae0")]
pub struct CreateLockEscrow {}

pub struct CreateLockEscrowInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub lock_escrow: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateLockEscrow {
    type ArrangedAccounts = CreateLockEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, lock_escrow, owner, lp_mint, payer, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(CreateLockEscrowInstructionAccounts {
            pool: pool.pubkey,
            lock_escrow: lock_escrow.pubkey,
            owner: owner.pubkey,
            lp_mint: lp_mint.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
