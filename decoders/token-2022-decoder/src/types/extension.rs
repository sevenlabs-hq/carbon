use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum Extension {
    Uninitialized,
    TransferFeeConfig {
        transfer_fee_config_authority: solana_sdk::pubkey::Pubkey,
        withdraw_withheld_authority: solana_sdk::pubkey::Pubkey,
        withheld_amount: u64,
        older_transfer_fee: TransferFee,
        newer_transfer_fee: TransferFee,
    },
    TransferFeeAmount {
        withheld_amount: u64,
    },
    MintCloseAuthority {
        close_authority: solana_sdk::pubkey::Pubkey,
    },
    ConfidentialTransferMint {
        authority: Option<solana_sdk::pubkey::Pubkey>,
        auto_approve_new_accounts: bool,
        auditor_elgamal_pubkey: Option<solana_sdk::pubkey::Pubkey>,
    },
    ConfidentialTransferAccount {
        approved: bool,
        elgamal_pubkey: solana_sdk::pubkey::Pubkey,
        #[serde(with = "BigArray")]
        pending_balance_low: [u8; 64],
        #[serde(with = "BigArray")]
        pending_balance_high: [u8; 64],
        #[serde(with = "BigArray")]
        available_balance: [u8; 64],
        #[serde(with = "BigArray")]
        decryptable_available_balance: [u8; 36],
        allow_confidential_credits: bool,
        allow_non_confidential_credits: bool,
        pending_balance_credit_counter: u64,
        maximum_pending_balance_credit_counter: u64,
        expected_pending_balance_credit_counter: u64,
        actual_pending_balance_credit_counter: u64,
    },
    DefaultAccountState {
        state: AccountState,
    },
    ImmutableOwner {},
    MemoTransfer {
        require_incoming_transfer_memos: bool,
    },
    NonTransferable {},
    InterestBearingConfig {
        rate_authority: solana_sdk::pubkey::Pubkey,
        initialization_timestamp: u64,
        pre_update_average_rate: i16,
        last_update_timestamp: u64,
        current_rate: i16,
    },
    CpiGuard {
        lock_cpi: bool,
    },
    PermanentDelegate {
        delegate: solana_sdk::pubkey::Pubkey,
    },
    NonTransferableAccount {},
    TransferHook {
        authority: solana_sdk::pubkey::Pubkey,
        program_id: solana_sdk::pubkey::Pubkey,
    },
    TransferHookAccount {
        transferring: bool,
    },
    ConfidentialTransferFee {
        authority: Option<solana_sdk::pubkey::Pubkey>,
        elgamal_pubkey: solana_sdk::pubkey::Pubkey,
        harvest_to_mint_enabled: bool,
        #[serde(with = "BigArray")]
        withheld_amount: [u8; 64],
    },
    ConfidentialTransferFeeAmount {
        #[serde(with = "BigArray")]
        withheld_amount: [u8; 64],
    },
    MetadataPointer {
        authority: Option<solana_sdk::pubkey::Pubkey>,
        metadata_address: Option<solana_sdk::pubkey::Pubkey>,
    },
    TokenMetadata {
        update_authority: Option<solana_sdk::pubkey::Pubkey>,
        mint: solana_sdk::pubkey::Pubkey,
        name: String,
        symbol: String,
        uri: String,
        additional_metadata: Vec<(String, String)>,
    },
    GroupPointer {
        authority: Option<solana_sdk::pubkey::Pubkey>,
        group_address: Option<solana_sdk::pubkey::Pubkey>,
    },
    TokenGroup {
        update_authority: Option<solana_sdk::pubkey::Pubkey>,
        mint: solana_sdk::pubkey::Pubkey,
        size: u64,
        max_size: u64,
    },
    GroupMemberPointer {
        authority: Option<solana_sdk::pubkey::Pubkey>,
        member_address: Option<solana_sdk::pubkey::Pubkey>,
    },
    TokenGroupMember {
        mint: solana_sdk::pubkey::Pubkey,
        group: solana_sdk::pubkey::Pubkey,
        member_number: u64,
    },
}
