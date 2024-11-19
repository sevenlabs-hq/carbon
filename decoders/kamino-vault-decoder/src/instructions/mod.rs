use super::KaminoVaultDecoder;
pub mod deposit;
pub mod give_up_pending_fees;
pub mod init_vault;
pub mod initialize_shares_metadata;
pub mod invest;
pub mod update_admin;
pub mod update_reserve_allocation;
pub mod update_shares_metadata;
pub mod update_vault_config;
pub mod withdraw;
pub mod withdraw_from_available;
pub mod withdraw_pending_fees;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum KaminoVaultInstruction {
    InitVault(init_vault::InitVault),
    UpdateReserveAllocation(update_reserve_allocation::UpdateReserveAllocation),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    Invest(invest::Invest),
    UpdateVaultConfig(update_vault_config::UpdateVaultConfig),
    WithdrawPendingFees(withdraw_pending_fees::WithdrawPendingFees),
    UpdateAdmin(update_admin::UpdateAdmin),
    GiveUpPendingFees(give_up_pending_fees::GiveUpPendingFees),
    InitializeSharesMetadata(initialize_shares_metadata::InitializeSharesMetadata),
    UpdateSharesMetadata(update_shares_metadata::UpdateSharesMetadata),
    WithdrawFromAvailable(withdraw_from_available::WithdrawFromAvailable),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for KaminoVaultDecoder {
    type InstructionType = KaminoVaultInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            KaminoVaultInstruction::InitVault => init_vault::InitVault,
            KaminoVaultInstruction::UpdateReserveAllocation => update_reserve_allocation::UpdateReserveAllocation,
            KaminoVaultInstruction::Deposit => deposit::Deposit,
            KaminoVaultInstruction::Withdraw => withdraw::Withdraw,
            KaminoVaultInstruction::Invest => invest::Invest,
            KaminoVaultInstruction::UpdateVaultConfig => update_vault_config::UpdateVaultConfig,
            KaminoVaultInstruction::WithdrawPendingFees => withdraw_pending_fees::WithdrawPendingFees,
            KaminoVaultInstruction::UpdateAdmin => update_admin::UpdateAdmin,
            KaminoVaultInstruction::GiveUpPendingFees => give_up_pending_fees::GiveUpPendingFees,
            KaminoVaultInstruction::InitializeSharesMetadata => initialize_shares_metadata::InitializeSharesMetadata,
            KaminoVaultInstruction::UpdateSharesMetadata => update_shares_metadata::UpdateSharesMetadata,
            KaminoVaultInstruction::WithdrawFromAvailable => withdraw_from_available::WithdrawFromAvailable,
        )
    }
}
