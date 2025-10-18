use {
    super::HeavenDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod liquidity_pool_state;
pub mod msol_ticket_sol_spent;
pub mod protocol_admin_state;
pub mod protocol_config;
pub mod protocol_owner_state;

#[allow(clippy::large_enum_variant)]
pub enum HeavenAccount {
    LiquidityPoolState(liquidity_pool_state::LiquidityPoolState),
    MsolTicketSolSpent(msol_ticket_sol_spent::MsolTicketSolSpent),
    ProtocolAdminState(protocol_admin_state::ProtocolAdminState),
    ProtocolConfig(protocol_config::ProtocolConfig),
    ProtocolOwnerState(protocol_owner_state::ProtocolOwnerState),
}

impl AccountDecoder<'_> for HeavenDecoder {
    type AccountType = HeavenAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            liquidity_pool_state::LiquidityPoolState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenAccount::LiquidityPoolState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            msol_ticket_sol_spent::MsolTicketSolSpent::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenAccount::MsolTicketSolSpent(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_admin_state::ProtocolAdminState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenAccount::ProtocolAdminState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_config::ProtocolConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenAccount::ProtocolConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_owner_state::ProtocolOwnerState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenAccount::ProtocolOwnerState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
