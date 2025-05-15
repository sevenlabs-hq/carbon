use crate::PROGRAM_ID;

use super::RaydiumClmmDecoder;
pub mod close_position;
pub mod collect_fund_fee;
pub mod collect_personal_fee_event;
pub mod collect_protocol_fee;
pub mod collect_protocol_fee_event;
pub mod collect_remaining_rewards;
pub mod config_change_event;
pub mod create_amm_config;
pub mod create_operation_account;
pub mod create_personal_position_event;
pub mod create_pool;
pub mod decrease_liquidity;
pub mod decrease_liquidity_event;
pub mod decrease_liquidity_v2;
pub mod increase_liquidity;
pub mod increase_liquidity_event;
pub mod increase_liquidity_v2;
pub mod initialize_reward;
pub mod liquidity_calculate_event;
pub mod liquidity_change_event;
pub mod open_position;
pub mod open_position_v2;
pub mod open_position_with_token22_nft;
pub mod pool_created_event;
pub mod set_reward_params;
pub mod swap;
pub mod swap_event;
pub mod swap_router_base_in;
pub mod swap_v2;
pub mod transfer_reward_owner;
pub mod update_amm_config;
pub mod update_operation_account;
pub mod update_pool_status;
pub mod update_reward_infos;
pub mod update_reward_infos_event;

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
pub enum RaydiumClmmInstruction {
    CreateAmmConfig(create_amm_config::CreateAmmConfig),
    UpdateAmmConfig(update_amm_config::UpdateAmmConfig),
    CreatePool(create_pool::CreatePool),
    UpdatePoolStatus(update_pool_status::UpdatePoolStatus),
    CreateOperationAccount(create_operation_account::CreateOperationAccount),
    UpdateOperationAccount(update_operation_account::UpdateOperationAccount),
    TransferRewardOwner(transfer_reward_owner::TransferRewardOwner),
    InitializeReward(initialize_reward::InitializeReward),
    CollectRemainingRewards(collect_remaining_rewards::CollectRemainingRewards),
    UpdateRewardInfos(update_reward_infos::UpdateRewardInfos),
    SetRewardParams(set_reward_params::SetRewardParams),
    CollectProtocolFee(collect_protocol_fee::CollectProtocolFee),
    CollectFundFee(collect_fund_fee::CollectFundFee),
    OpenPosition(open_position::OpenPosition),
    OpenPositionV2(open_position_v2::OpenPositionV2),
    OpenPositionWithToken22Nft(open_position_with_token22_nft::OpenPositionWithToken22Nft),
    ClosePosition(close_position::ClosePosition),
    IncreaseLiquidity(increase_liquidity::IncreaseLiquidity),
    IncreaseLiquidityV2(increase_liquidity_v2::IncreaseLiquidityV2),
    DecreaseLiquidity(decrease_liquidity::DecreaseLiquidity),
    DecreaseLiquidityV2(decrease_liquidity_v2::DecreaseLiquidityV2),
    Swap(swap::Swap),
    SwapV2(swap_v2::SwapV2),
    SwapRouterBaseIn(swap_router_base_in::SwapRouterBaseIn),
    ConfigChangeEvent(config_change_event::ConfigChangeEvent),
    CreatePersonalPositionEvent(create_personal_position_event::CreatePersonalPositionEvent),
    IncreaseLiquidityEvent(increase_liquidity_event::IncreaseLiquidityEvent),
    DecreaseLiquidityEvent(decrease_liquidity_event::DecreaseLiquidityEvent),
    LiquidityCalculateEvent(liquidity_calculate_event::LiquidityCalculateEvent),
    CollectPersonalFeeEvent(collect_personal_fee_event::CollectPersonalFeeEvent),
    UpdateRewardInfosEvent(update_reward_infos_event::UpdateRewardInfosEvent),
    PoolCreatedEvent(pool_created_event::PoolCreatedEvent),
    CollectProtocolFeeEvent(collect_protocol_fee_event::CollectProtocolFeeEvent),
    SwapEvent(swap_event::SwapEvent),
    LiquidityChangeEvent(liquidity_change_event::LiquidityChangeEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for RaydiumClmmDecoder {
    type InstructionType = RaydiumClmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            RaydiumClmmInstruction::CreateAmmConfig => create_amm_config::CreateAmmConfig,
            RaydiumClmmInstruction::UpdateAmmConfig => update_amm_config::UpdateAmmConfig,
            RaydiumClmmInstruction::CreatePool => create_pool::CreatePool,
            RaydiumClmmInstruction::UpdatePoolStatus => update_pool_status::UpdatePoolStatus,
            RaydiumClmmInstruction::CreateOperationAccount => create_operation_account::CreateOperationAccount,
            RaydiumClmmInstruction::UpdateOperationAccount => update_operation_account::UpdateOperationAccount,
            RaydiumClmmInstruction::TransferRewardOwner => transfer_reward_owner::TransferRewardOwner,
            RaydiumClmmInstruction::InitializeReward => initialize_reward::InitializeReward,
            RaydiumClmmInstruction::CollectRemainingRewards => collect_remaining_rewards::CollectRemainingRewards,
            RaydiumClmmInstruction::UpdateRewardInfos => update_reward_infos::UpdateRewardInfos,
            RaydiumClmmInstruction::SetRewardParams => set_reward_params::SetRewardParams,
            RaydiumClmmInstruction::CollectProtocolFee => collect_protocol_fee::CollectProtocolFee,
            RaydiumClmmInstruction::CollectFundFee => collect_fund_fee::CollectFundFee,
            RaydiumClmmInstruction::OpenPosition => open_position::OpenPosition,
            RaydiumClmmInstruction::OpenPositionV2 => open_position_v2::OpenPositionV2,
            RaydiumClmmInstruction::OpenPositionWithToken22Nft => open_position_with_token22_nft::OpenPositionWithToken22Nft,
            RaydiumClmmInstruction::ClosePosition => close_position::ClosePosition,
            RaydiumClmmInstruction::IncreaseLiquidity => increase_liquidity::IncreaseLiquidity,
            RaydiumClmmInstruction::IncreaseLiquidityV2 => increase_liquidity_v2::IncreaseLiquidityV2,
            RaydiumClmmInstruction::DecreaseLiquidity => decrease_liquidity::DecreaseLiquidity,
            RaydiumClmmInstruction::DecreaseLiquidityV2 => decrease_liquidity_v2::DecreaseLiquidityV2,
            RaydiumClmmInstruction::Swap => swap::Swap,
            RaydiumClmmInstruction::SwapV2 => swap_v2::SwapV2,
            RaydiumClmmInstruction::SwapRouterBaseIn => swap_router_base_in::SwapRouterBaseIn,
            RaydiumClmmInstruction::ConfigChangeEvent => config_change_event::ConfigChangeEvent,
            RaydiumClmmInstruction::CreatePersonalPositionEvent => create_personal_position_event::CreatePersonalPositionEvent,
            RaydiumClmmInstruction::IncreaseLiquidityEvent => increase_liquidity_event::IncreaseLiquidityEvent,
            RaydiumClmmInstruction::DecreaseLiquidityEvent => decrease_liquidity_event::DecreaseLiquidityEvent,
            RaydiumClmmInstruction::LiquidityCalculateEvent => liquidity_calculate_event::LiquidityCalculateEvent,
            RaydiumClmmInstruction::CollectPersonalFeeEvent => collect_personal_fee_event::CollectPersonalFeeEvent,
            RaydiumClmmInstruction::UpdateRewardInfosEvent => update_reward_infos_event::UpdateRewardInfosEvent,
            RaydiumClmmInstruction::PoolCreatedEvent => pool_created_event::PoolCreatedEvent,
            RaydiumClmmInstruction::CollectProtocolFeeEvent => collect_protocol_fee_event::CollectProtocolFeeEvent,
            RaydiumClmmInstruction::SwapEvent => swap_event::SwapEvent,
            RaydiumClmmInstruction::LiquidityChangeEvent => liquidity_change_event::LiquidityChangeEvent,
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_instruction::AccountMeta;
    use solana_pubkey::pubkey;

    use crate::{
        instructions::{
            close_position::{ClosePosition, ClosePositionInstructionAccounts},
            collect_fund_fee::{CollectFundFee, CollectFundFeeInstructionAccounts},
            collect_protocol_fee::{CollectProtocolFee, CollectProtocolFeeInstructionAccounts},
            collect_remaining_rewards::{
                CollectRemainingRewards, CollectRemainingRewardsInstructionAccounts,
            },
            create_amm_config::{CreateAmmConfig, CreateAmmConfigInstructionAccounts},
            create_pool::{CreatePool, CreatePoolInstructionAccounts},
            decrease_liquidity::{DecreaseLiquidity, DecreaseLiquidityInstructionAccounts},
            decrease_liquidity_v2::{DecreaseLiquidityV2, DecreaseLiquidityV2InstructionAccounts},
            increase_liquidity::{IncreaseLiquidity, IncreaseLiquidityInstructionAccounts},
            increase_liquidity_v2::{IncreaseLiquidityV2, IncreaseLiquidityV2InstructionAccounts},
            initialize_reward::{InitializeReward, InitializeRewardInstructionAccounts},
            open_position::{OpenPosition, OpenPositionInstructionAccounts},
            open_position_v2::{OpenPositionV2, OpenPositionV2InstructionAccounts},
            open_position_with_token22_nft::{
                OpenPositionWithToken22Nft, OpenPositionWithToken22NftInstructionAccounts,
            },
            set_reward_params::{SetRewardParams, SetRewardParamsInstructionAccounts},
            swap::{Swap, SwapInstructionAccounts},
            swap_router_base_in::{SwapRouterBaseIn, SwapRouterBaseInInstructionAccounts},
            swap_v2::{SwapV2, SwapV2InstructionAccounts},
            update_amm_config::{UpdateAmmConfig, UpdateAmmConfigInstructionAccounts},
            update_pool_status::{UpdatePoolStatus, UpdatePoolStatusInstructionAccounts},
            update_reward_infos::{UpdateRewardInfos, UpdateRewardInfosInstructionAccounts},
        },
        types::InitializeRewardParam,
    };

    use super::*;

    #[test]
    fn test_decode_create_amm_config_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::CreateAmmConfig(CreateAmmConfig {
            fund_fee_rate: 40000,
            index: 18,
            protocol_fee_rate: 120000,
            tick_spacing: 120,
            trade_fee_rate: 30000,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("GThUX1Atko4tqhN2NaiTazWSeFWMuiUvfFnyJyUghFMJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CDpiwv9eLsRvvuzZEJ8CBtK14wdvkSnkub4vmGtzzdK8"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];
        let expected_arranged_accounts = CreateAmmConfigInstructionAccounts {
            owner: pubkey!("GThUX1Atko4tqhN2NaiTazWSeFWMuiUvfFnyJyUghFMJ"),
            amm_config: pubkey!("CDpiwv9eLsRvvuzZEJ8CBtK14wdvkSnkub4vmGtzzdK8"),
            system_program: pubkey!("11111111111111111111111111111111"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/create_amm_config_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CreateAmmConfig::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_update_amm_config_ix() {
        // Arrange
        let expected_ix =
            RaydiumClmmInstruction::UpdateAmmConfig(UpdateAmmConfig { param: 3, value: 0 });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("GThUX1Atko4tqhN2NaiTazWSeFWMuiUvfFnyJyUghFMJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CDpiwv9eLsRvvuzZEJ8CBtK14wdvkSnkub4vmGtzzdK8"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("projjosVCPQH49d5em7VYS7fJZzaqKixqKtus7yk416"),
                false,
            ),
        ];
        let expected_arranged_accounts = UpdateAmmConfigInstructionAccounts {
            owner: pubkey!("GThUX1Atko4tqhN2NaiTazWSeFWMuiUvfFnyJyUghFMJ"),
            amm_config: pubkey!("CDpiwv9eLsRvvuzZEJ8CBtK14wdvkSnkub4vmGtzzdK8"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/update_amm_config_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            UpdateAmmConfig::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_create_pool_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::CreatePool(CreatePool {
            open_time: 0,
            sqrt_price_x64: 3742598438193254786,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("7XiakBmwHdVVXmPKjT35JFDUMzP8aszSg3tU4SaL16KU"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7ypbduGATyfEhTCMCLhhGtCTsPudEKTbMgJfA5gS44Qh"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6NcdiK8B5KK2DzKvzvCfqi8EHaEqu48fyEzC8Mm9pump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3Rkt6v35Z3vXeBBYRFiF3GY6RX1DWB7RkL7AHmY9VGsM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FR8XJ3DhoBCV6M8Rc15V16N4Mpv76NG2ktsWpZqKUEM8"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HYivBDMTU5AaGY6NmGxwGjqLBsLEqVroQA2V3iQxi2vK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DsFfPkzPibWm734yasxctaT1A3Q6y27qbo6uYVTDtUqj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
        ];
        let expected_arranged_accounts = CreatePoolInstructionAccounts {
            pool_creator: pubkey!("7XiakBmwHdVVXmPKjT35JFDUMzP8aszSg3tU4SaL16KU"),
            amm_config: pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
            pool_state: pubkey!("7ypbduGATyfEhTCMCLhhGtCTsPudEKTbMgJfA5gS44Qh"),
            token_mint0: pubkey!("6NcdiK8B5KK2DzKvzvCfqi8EHaEqu48fyEzC8Mm9pump"),
            token_mint1: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            token_vault0: pubkey!("3Rkt6v35Z3vXeBBYRFiF3GY6RX1DWB7RkL7AHmY9VGsM"),
            token_vault1: pubkey!("FR8XJ3DhoBCV6M8Rc15V16N4Mpv76NG2ktsWpZqKUEM8"),
            observation_state: pubkey!("HYivBDMTU5AaGY6NmGxwGjqLBsLEqVroQA2V3iQxi2vK"),
            tick_array_bitmap: pubkey!("DsFfPkzPibWm734yasxctaT1A3Q6y27qbo6uYVTDtUqj"),
            token_program0: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program1: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/create_pool_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CreatePool::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_update_pool_status_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::UpdatePoolStatus(UpdatePoolStatus { status: 17 });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("GThUX1Atko4tqhN2NaiTazWSeFWMuiUvfFnyJyUghFMJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3tD34VtprDSkYCnATtQLCiVgTkECU3d12KtjupeR6N2X"),
                false,
            ),
        ];
        let expected_arranged_accounts = UpdatePoolStatusInstructionAccounts {
            authority: pubkey!("GThUX1Atko4tqhN2NaiTazWSeFWMuiUvfFnyJyUghFMJ"),
            pool_state: pubkey!("3tD34VtprDSkYCnATtQLCiVgTkECU3d12KtjupeR6N2X"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/update_pool_status_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            UpdatePoolStatus::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_reward_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::InitializeReward(InitializeReward {
            param: InitializeRewardParam {
                emissions_per_second_x64: 18446744073709551616,
                end_time: 1753877100,
                open_time: 1748693100,
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("7JLCEKwakzV2J9iHmku2jUfeqRtK2q23hdYFfcrBzZWF"),
                true,
            ),
            AccountMeta::new(
                pubkey!("FGgVG1Zpn8kbfD8BgzbkRiPLTh3phy2NCsrF8y1vkq95"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EuMCL5TYaNSg4FomojmFEmS4okjRYNNq7Ydvxw4yGdUC"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7EJTuZgJKbWZCX7R2cLm54asioKQk61VijX56s2RAGwN"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("92RUVMYCFYyFj5pYE6NSjRXBXvSe1iEmhDVjtY3EHwTf"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializeRewardInstructionAccounts {
            reward_funder: pubkey!("7JLCEKwakzV2J9iHmku2jUfeqRtK2q23hdYFfcrBzZWF"),
            funder_token_account: pubkey!("FGgVG1Zpn8kbfD8BgzbkRiPLTh3phy2NCsrF8y1vkq95"),
            amm_config: pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
            pool_state: pubkey!("EuMCL5TYaNSg4FomojmFEmS4okjRYNNq7Ydvxw4yGdUC"),
            operation_state: pubkey!("7EJTuZgJKbWZCX7R2cLm54asioKQk61VijX56s2RAGwN"),
            reward_token_mint: pubkey!("So11111111111111111111111111111111111111112"),
            reward_token_vault: pubkey!("92RUVMYCFYyFj5pYE6NSjRXBXvSe1iEmhDVjtY3EHwTf"),
            reward_token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_reward_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeReward::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_collect_remaining_rewards_ix() {
        // Arrange
        let expected_ix =
            RaydiumClmmInstruction::CollectRemainingRewards(CollectRemainingRewards {
                reward_index: 0,
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("CC6ofKsD2YE82ZzCDaCSvhm3NqbipSDF216ry1c5GcNE"),
                true,
            ),
            AccountMeta::new(
                pubkey!("HUQaPaUe83v6F3i6PyE5L4LAfp7wURYXmdtA6RdLMsgZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7QKDXuYPV7bWxBrX4bZozjNK8sztHw1gBzPUi5oRnG5t"),
                false,
            ),
            AccountMeta::new(
                pubkey!("JAuec9nZVzx6Ytmn6UHKro1JS3GckC6nLPaA8qUSwXnL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
        ];
        let expected_arranged_accounts = CollectRemainingRewardsInstructionAccounts {
            reward_funder: pubkey!("CC6ofKsD2YE82ZzCDaCSvhm3NqbipSDF216ry1c5GcNE"),
            funder_token_account: pubkey!("HUQaPaUe83v6F3i6PyE5L4LAfp7wURYXmdtA6RdLMsgZ"),
            pool_state: pubkey!("7QKDXuYPV7bWxBrX4bZozjNK8sztHw1gBzPUi5oRnG5t"),
            reward_token_vault: pubkey!("JAuec9nZVzx6Ytmn6UHKro1JS3GckC6nLPaA8qUSwXnL"),
            reward_vault_mint: pubkey!("So11111111111111111111111111111111111111112"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/collect_remaining_rewards_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CollectRemainingRewards::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_update_reward_infos_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::UpdateRewardInfos(UpdateRewardInfos {});
        let expected_accounts = vec![AccountMeta::new(
            pubkey!("EEkMbgYJzgYNUJsnURtL3jkDdkAws5RbDL7ukCUFNRMV"),
            false,
        )];
        let expected_arranged_accounts = UpdateRewardInfosInstructionAccounts {
            pool_state: pubkey!("EEkMbgYJzgYNUJsnURtL3jkDdkAws5RbDL7ukCUFNRMV"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/update_reward_infos_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            UpdateRewardInfos::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_set_reward_params_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::SetRewardParams(SetRewardParams {
            emissions_per_second_x64: 15250284452471520847000,
            end_time: 1750090500,
            open_time: 1747066500,
            reward_index: 0,
        });
        let expected_accounts = vec![
            AccountMeta::new(pubkey!("NCV2Uo3hfW5LSZXAJe19y6SpC5K98PuQwShCSZgTki3"), true),
            AccountMeta::new_readonly(
                pubkey!("9EeWRCL8CJnikDFCDzG8rtmBs5KQR1jEYKCR5rRZ2NEi"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8KbrpeSRYXYjWSSdG7gE1tR7Go8MmKKxKaei1gGc4U7Q"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7EJTuZgJKbWZCX7R2cLm54asioKQk61VijX56s2RAGwN"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ACLtBsUtRGT2rxnF8g9JV5UCG3ikJXR2gKVK1RrsY6fc"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4u5GmwvqKjeSRnaWnSnqU3DsdDV1mwD8YBn486LLF9rL"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R"),
                false,
            ),
        ];
        let expected_arranged_accounts = SetRewardParamsInstructionAccounts {
            authority: pubkey!("NCV2Uo3hfW5LSZXAJe19y6SpC5K98PuQwShCSZgTki3"),
            amm_config: pubkey!("9EeWRCL8CJnikDFCDzG8rtmBs5KQR1jEYKCR5rRZ2NEi"),
            pool_state: pubkey!("8KbrpeSRYXYjWSSdG7gE1tR7Go8MmKKxKaei1gGc4U7Q"),
            operation_state: pubkey!("7EJTuZgJKbWZCX7R2cLm54asioKQk61VijX56s2RAGwN"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/set_reward_params_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SetRewardParams::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_collect_protocol_fee_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::CollectProtocolFee(CollectProtocolFee {
            amount0_requested: 18446744073709551615,
            amount1_requested: 18446744073709551615,
        });
        let expected_accounts = vec![
            AccountMeta::new(pubkey!("projjosVCPQH49d5em7VYS7fJZzaqKixqKtus7yk416"), true),
            AccountMeta::new(
                pubkey!("GEacZ94pW7egZsPLgNWXrWBnA5qPTfbRUU5epdfjaLF2"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
                false,
            ),
            AccountMeta::new(
                pubkey!("74HfZjYRV3QM5STo8NyReh9tXk4c6EjMyiFgwVgncXXh"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6oLppZGXQdhCoNCKrwd6KUQCQ2ArMVDh8WiuuYUYXSyM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Hg8bKz4mvs8KNj9zew1cEF9tDw1x2GViB4RFZjVEmfrD"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8awFmuH8YGXpm4geYYRcqDsLiqrzPUnJrNCDi8rfmFPg"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EPKH1qtVPrmofbcHjqhz5R5D4fEC5GdgxXNCv5qqBts7"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
        ];
        let expected_arranged_accounts = CollectProtocolFeeInstructionAccounts {
            owner: pubkey!("projjosVCPQH49d5em7VYS7fJZzaqKixqKtus7yk416"),
            pool_state: pubkey!("GEacZ94pW7egZsPLgNWXrWBnA5qPTfbRUU5epdfjaLF2"),
            amm_config: pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
            token_vault0: pubkey!("74HfZjYRV3QM5STo8NyReh9tXk4c6EjMyiFgwVgncXXh"),
            token_vault1: pubkey!("6oLppZGXQdhCoNCKrwd6KUQCQ2ArMVDh8WiuuYUYXSyM"),
            vault0_mint: pubkey!("So11111111111111111111111111111111111111112"),
            vault1_mint: pubkey!("Hg8bKz4mvs8KNj9zew1cEF9tDw1x2GViB4RFZjVEmfrD"),
            recipient_token_account0: pubkey!("8awFmuH8YGXpm4geYYRcqDsLiqrzPUnJrNCDi8rfmFPg"),
            recipient_token_account1: pubkey!("EPKH1qtVPrmofbcHjqhz5R5D4fEC5GdgxXNCv5qqBts7"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/collect_protocol_fee_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CollectProtocolFee::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_collect_fund_fee_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::CollectFundFee(CollectFundFee {
            amount0_requested: 18446744073709551615,
            amount1_requested: 18446744073709551615,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("FundHfY8oo8J9KYGyfXFFuQCHe7Z1VBNmsj84eMcdYs4"),
                true,
            ),
            AccountMeta::new(
                pubkey!("x4ND6LEXnrj3ufeCTY8RSuo3qbktirsz4tqPus5SjrH"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
                false,
            ),
            AccountMeta::new(
                pubkey!("79wNxXqsdddGBUBVkrDpTikGBt8BXcNQhkM6oHGmQMcm"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6qV3wbnXWeZpyxseMzmTkusLxbP2aRmXYWg9aq8ADME5"),
                false,
            ),
            AccountMeta::new(
                pubkey!("boopkpWqe68MSxLqBGogs8ZbUDN4GXaLhFwNP7mpP1i"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("mpHsYZfrNeX8ui6wqoiMHuRHnsSNcepEY6gKqi87HwP"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8fukc2EkRmtSFwiRCWkAQDL38kHXVzd9khjVaHarDi83"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
        ];
        let expected_arranged_accounts = CollectFundFeeInstructionAccounts {
            owner: pubkey!("FundHfY8oo8J9KYGyfXFFuQCHe7Z1VBNmsj84eMcdYs4"),
            pool_state: pubkey!("x4ND6LEXnrj3ufeCTY8RSuo3qbktirsz4tqPus5SjrH"),
            amm_config: pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
            token_vault0: pubkey!("79wNxXqsdddGBUBVkrDpTikGBt8BXcNQhkM6oHGmQMcm"),
            token_vault1: pubkey!("6qV3wbnXWeZpyxseMzmTkusLxbP2aRmXYWg9aq8ADME5"),
            vault0_mint: pubkey!("boopkpWqe68MSxLqBGogs8ZbUDN4GXaLhFwNP7mpP1i"),
            vault1_mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            recipient_token_account0: pubkey!("mpHsYZfrNeX8ui6wqoiMHuRHnsSNcepEY6gKqi87HwP"),
            recipient_token_account1: pubkey!("8fukc2EkRmtSFwiRCWkAQDL38kHXVzd9khjVaHarDi83"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/collect_fund_fee_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CollectFundFee::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_open_position_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::OpenPosition(OpenPosition {
            amount0_max: 26479886,
            amount1_max: 3564247,
            liquidity: Default::default(),
            tick_array_lower_start_index: -20820,
            tick_array_upper_start_index: -20820,
            tick_lower_index: -20809,
            tick_upper_index: -20775,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AcF6hy6FySj53zjhxw1CLptec9qiLD3YUvGMNJzfZ9Yz"),
                true,
            ),
            AccountMeta::new(
                pubkey!("AcF6hy6FySj53zjhxw1CLptec9qiLD3YUvGMNJzfZ9Yz"),
                true,
            ),
            AccountMeta::new(
                pubkey!("39KYyfxEKSkPWs6CGULa3Z3sUbeXTWRn8UqA9QSM9WEF"),
                true,
            ),
            AccountMeta::new(
                pubkey!("HsPTo8LUWw13bP9X1Ypus5PUeoAcGYEKUvdVPvP7ZUfw"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Dq69PStoNUjpJVpNnLpNWAq6euW4fxgYLHKHnjnXVDW2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3ucNos4NbumPLZNWztqGHNFFgkHeRMBQAVemeeomsUxv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6A7a4aCH872N226EjmcfeEmxJKk4aVGrxirWTXHnru82"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9PbxB3tGEBuARqjB6C23uoAZpXDAxZN4jmay1Lb1y3Us"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9PbxB3tGEBuARqjB6C23uoAZpXDAxZN4jmay1Lb1y3Us"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CjkAeHib7o9skzKFEE9wVeJYQhPwNWytGnLcPmZ42fw3"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6cHjxhghqMQwn4dkageYQc7rxsSUrasDWTVdC28P1Z2a"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2J6dqfvSqYFTaeNznJ58Tj3yufzExy35yzLqFr95H294"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4ct7br2vTPzfdmY3S5HLtTxcGSBfn6pnw98hsS6v359A"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5it83u57VRrVgc51oNV19TTmAJuffPx5GtGwQr7gQNUo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ),
        ];
        let expected_arranged_accounts = OpenPositionInstructionAccounts {
            payer: pubkey!("AcF6hy6FySj53zjhxw1CLptec9qiLD3YUvGMNJzfZ9Yz"),
            position_nft_owner: pubkey!("AcF6hy6FySj53zjhxw1CLptec9qiLD3YUvGMNJzfZ9Yz"),
            position_nft_mint: pubkey!("39KYyfxEKSkPWs6CGULa3Z3sUbeXTWRn8UqA9QSM9WEF"),
            position_nft_account: pubkey!("HsPTo8LUWw13bP9X1Ypus5PUeoAcGYEKUvdVPvP7ZUfw"),
            metadata_account: pubkey!("Dq69PStoNUjpJVpNnLpNWAq6euW4fxgYLHKHnjnXVDW2"),
            pool_state: pubkey!("3ucNos4NbumPLZNWztqGHNFFgkHeRMBQAVemeeomsUxv"),
            protocol_position: pubkey!("6A7a4aCH872N226EjmcfeEmxJKk4aVGrxirWTXHnru82"),
            tick_array_lower: pubkey!("9PbxB3tGEBuARqjB6C23uoAZpXDAxZN4jmay1Lb1y3Us"),
            tick_array_upper: pubkey!("9PbxB3tGEBuARqjB6C23uoAZpXDAxZN4jmay1Lb1y3Us"),
            personal_position: pubkey!("CjkAeHib7o9skzKFEE9wVeJYQhPwNWytGnLcPmZ42fw3"),
            token_account0: pubkey!("6cHjxhghqMQwn4dkageYQc7rxsSUrasDWTVdC28P1Z2a"),
            token_account1: pubkey!("2J6dqfvSqYFTaeNznJ58Tj3yufzExy35yzLqFr95H294"),
            token_vault0: pubkey!("4ct7br2vTPzfdmY3S5HLtTxcGSBfn6pnw98hsS6v359A"),
            token_vault1: pubkey!("5it83u57VRrVgc51oNV19TTmAJuffPx5GtGwQr7gQNUo"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            metadata_program: pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/open_position_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            OpenPosition::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_open_position_v2_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::OpenPositionV2(OpenPositionV2 {
            amount0_max: 200000000000,
            amount1_max: 50000000000000,
            base_flag: Some(false),
            liquidity: Default::default(),
            tick_array_lower_start_index: -443640,
            tick_array_upper_start_index: 443580,
            tick_lower_index: -443630,
            tick_upper_index: 443630,
            with_metadata: true,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("CG2gaUEDTMAxjvimutjsjw59dbskgznmqAax77pbXLRT"),
                true,
            ),
            AccountMeta::new(
                pubkey!("CG2gaUEDTMAxjvimutjsjw59dbskgznmqAax77pbXLRT"),
                true,
            ),
            AccountMeta::new(
                pubkey!("Fav3BVLy2z6ASyvXB1gk7wJvdZHtYD63gM27XQSft4GD"),
                true,
            ),
            AccountMeta::new(
                pubkey!("E8or35BD2bgBtcNktFpcd8tQoKLX9HGnQpeMDADXFJfA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("H4wgEaDpKCSuNDQSTiAjpizFThnnWiMPipUZeQWRQi2o"),
                false,
            ),
            AccountMeta::new(
                pubkey!("945czrk5A4uSUH46xE8PLz4SMjqLXPDKqAyaGvA76bUg"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EcYjH4V9UFKPL5xTSpzDqJ8mxPCgnyybssfhrxpqK926"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HeV2hQbj6sxHZ8P3XW2Bho7iDzNpgC1MXiYmdTDvLpup"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3DGHSf9AYkon5ESJv6uJaVziuMVZ5Td7GBzuLU4w5cFt"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5GjaqVHrKEkHVxHZBato2rQoSPpYouiUotFaD3Rx9sq7"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BEHJiYPY9NVTAfju2mm8afL78SxJXJgkpNHReJQ1TVFF"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8ZstH9KBF6DV6GQ1vbZMTYBS8QKkgYbLFcWZXkMKTUVf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("anQHGFzjoUf6F663ZVzC1irEgvXdgHdgp35knPg3wNj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GkYAvt7ecy7juVHyPBssmJsw6DQLZ5r1YTzf58DMLFxu"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("3hiYB9c4VYhWbwkDgSbEv22cpsNvdYZHuVWMnnNgpump"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8yxZqXeguBG6fDSGLxTVTDMGEzUgatvhDzUC4CJv6KbQ"),
                false,
            ),
        ];
        let expected_arranged_accounts = OpenPositionV2InstructionAccounts {
            payer: pubkey!("CG2gaUEDTMAxjvimutjsjw59dbskgznmqAax77pbXLRT"),
            position_nft_owner: pubkey!("CG2gaUEDTMAxjvimutjsjw59dbskgznmqAax77pbXLRT"),
            position_nft_mint: pubkey!("Fav3BVLy2z6ASyvXB1gk7wJvdZHtYD63gM27XQSft4GD"),
            position_nft_account: pubkey!("E8or35BD2bgBtcNktFpcd8tQoKLX9HGnQpeMDADXFJfA"),
            metadata_account: pubkey!("H4wgEaDpKCSuNDQSTiAjpizFThnnWiMPipUZeQWRQi2o"),
            pool_state: pubkey!("945czrk5A4uSUH46xE8PLz4SMjqLXPDKqAyaGvA76bUg"),
            protocol_position: pubkey!("EcYjH4V9UFKPL5xTSpzDqJ8mxPCgnyybssfhrxpqK926"),
            tick_array_lower: pubkey!("HeV2hQbj6sxHZ8P3XW2Bho7iDzNpgC1MXiYmdTDvLpup"),
            tick_array_upper: pubkey!("3DGHSf9AYkon5ESJv6uJaVziuMVZ5Td7GBzuLU4w5cFt"),
            personal_position: pubkey!("5GjaqVHrKEkHVxHZBato2rQoSPpYouiUotFaD3Rx9sq7"),
            token_account0: pubkey!("BEHJiYPY9NVTAfju2mm8afL78SxJXJgkpNHReJQ1TVFF"),
            token_account1: pubkey!("8ZstH9KBF6DV6GQ1vbZMTYBS8QKkgYbLFcWZXkMKTUVf"),
            token_vault0: pubkey!("anQHGFzjoUf6F663ZVzC1irEgvXdgHdgp35knPg3wNj"),
            token_vault1: pubkey!("GkYAvt7ecy7juVHyPBssmJsw6DQLZ5r1YTzf58DMLFxu"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            metadata_program: pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            vault0_mint: pubkey!("So11111111111111111111111111111111111111112"),
            vault1_mint: pubkey!("3hiYB9c4VYhWbwkDgSbEv22cpsNvdYZHuVWMnnNgpump"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/open_position_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            OpenPositionV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_open_position_with_token22_nft_ix() {
        // Arrange
        let expected_ix =
            RaydiumClmmInstruction::OpenPositionWithToken22Nft(OpenPositionWithToken22Nft {
                amount0_max: 0,
                amount1_max: 970000000000000,
                base_flag: Some(false),
                liquidity: Default::default(),
                tick_array_lower_start_index: 39600,
                tick_array_upper_start_index: 39600,
                tick_lower_index: 42300,
                tick_upper_index: 42360,
                with_metadata: true,
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("H2octy9kmCKcVCMHvc52Pr9hqnCtdKvve5Uyu4Z5hT7X"),
                true,
            ),
            AccountMeta::new(
                pubkey!("H2octy9kmCKcVCMHvc52Pr9hqnCtdKvve5Uyu4Z5hT7X"),
                true,
            ),
            AccountMeta::new(
                pubkey!("CJrhrC4qGmuLjj6Fu6ScqxEEGU6t7bahsoezX5XxULHJ"),
                true,
            ),
            AccountMeta::new(
                pubkey!("Apg1nwThSAVAS9dfJs4AeKGs9JW2V3aSeYXb4cbjKMAs"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8hRSskMUXoPcWQtaikNY5L2Ws37y2i8BdqYo3xPhNjV4"),
                false,
            ),
            AccountMeta::new(
                pubkey!("J4u7UtCR8YK5bHDVhQNAj2edKV5LCHcgaBHRWnhn73gh"),
                false,
            ),
            AccountMeta::new(
                pubkey!("D4Qnmi5ckPF7PM3tiDZTmpeHNSbmfDB7Grim1Pq7HbEz"),
                false,
            ),
            AccountMeta::new(
                pubkey!("D4Qnmi5ckPF7PM3tiDZTmpeHNSbmfDB7Grim1Pq7HbEz"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BKEJeo3RHHyvfYMRzfXUPT6bfrSvAQpfpmkad5iCe1hB"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DxvY16W2GTgXb6ZV1UnoXRT8zUtNWCsv7veSfT4X7fVj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Er4T8wgw6pfygcn3V7hKWDhn8HqapdH1sTF1k44GjkWj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9A6LDovZBhssjHE3FgcG3M51K7NgrhMri2ziaf3R8MYq"),
                false,
            ),
            AccountMeta::new(
                pubkey!("hNoqb9J6tDRSXkmvQEundP9vqWmVnsNkSmcj9sdEWbP"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EwU8syStvemqcZ9A7Yioq6cPevQ6AVPjXAVzREs4QyC2"),
                false,
            ),
        ];
        let expected_arranged_accounts = OpenPositionWithToken22NftInstructionAccounts {
            payer: pubkey!("H2octy9kmCKcVCMHvc52Pr9hqnCtdKvve5Uyu4Z5hT7X"),
            position_nft_owner: pubkey!("H2octy9kmCKcVCMHvc52Pr9hqnCtdKvve5Uyu4Z5hT7X"),
            position_nft_mint: pubkey!("CJrhrC4qGmuLjj6Fu6ScqxEEGU6t7bahsoezX5XxULHJ"),
            position_nft_account: pubkey!("Apg1nwThSAVAS9dfJs4AeKGs9JW2V3aSeYXb4cbjKMAs"),
            pool_state: pubkey!("8hRSskMUXoPcWQtaikNY5L2Ws37y2i8BdqYo3xPhNjV4"),
            protocol_position: pubkey!("J4u7UtCR8YK5bHDVhQNAj2edKV5LCHcgaBHRWnhn73gh"),
            tick_array_lower: pubkey!("D4Qnmi5ckPF7PM3tiDZTmpeHNSbmfDB7Grim1Pq7HbEz"),
            tick_array_upper: pubkey!("D4Qnmi5ckPF7PM3tiDZTmpeHNSbmfDB7Grim1Pq7HbEz"),
            personal_position: pubkey!("BKEJeo3RHHyvfYMRzfXUPT6bfrSvAQpfpmkad5iCe1hB"),
            token_account0: pubkey!("DxvY16W2GTgXb6ZV1UnoXRT8zUtNWCsv7veSfT4X7fVj"),
            token_account1: pubkey!("Er4T8wgw6pfygcn3V7hKWDhn8HqapdH1sTF1k44GjkWj"),
            token_vault0: pubkey!("9A6LDovZBhssjHE3FgcG3M51K7NgrhMri2ziaf3R8MYq"),
            token_vault1: pubkey!("hNoqb9J6tDRSXkmvQEundP9vqWmVnsNkSmcj9sdEWbP"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            vault0_mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            vault1_mint: pubkey!("EwU8syStvemqcZ9A7Yioq6cPevQ6AVPjXAVzREs4QyC2"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/open_position_with_token22_nft_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            OpenPositionWithToken22Nft::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_close_position_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::ClosePosition(ClosePosition {});
        let expected_accounts = vec![
            AccountMeta::new(pubkey!("8yXy8QbeVEWFLTsfG9sZCpnkePxZDWfTkiT3pSvPCS1"), true),
            AccountMeta::new(
                pubkey!("7f7wprTVee5xZnfjNhGwKhPwH4bfzdkGezhbSvPF9SAE"),
                false,
            ),
            AccountMeta::new(
                pubkey!("G2jJSkDvTcH5Sd4dk5XkGKMZdfisU9pWoUqvcQPrXyZK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ExAA7ypa9knMqF22c2kgR1jUKvUXx97J7FjpKpUeMW6u"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
        ];
        let expected_arranged_accounts = ClosePositionInstructionAccounts {
            nft_owner: pubkey!("8yXy8QbeVEWFLTsfG9sZCpnkePxZDWfTkiT3pSvPCS1"),
            position_nft_mint: pubkey!("7f7wprTVee5xZnfjNhGwKhPwH4bfzdkGezhbSvPF9SAE"),
            position_nft_account: pubkey!("G2jJSkDvTcH5Sd4dk5XkGKMZdfisU9pWoUqvcQPrXyZK"),
            personal_position: pubkey!("ExAA7ypa9knMqF22c2kgR1jUKvUXx97J7FjpKpUeMW6u"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/close_position_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClosePosition::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_increase_liquidity_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::IncreaseLiquidity(IncreaseLiquidity {
            amount0_max: 2282862,
            amount1_max: 1913742,
            liquidity: 401645319,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("5XZZSAGds4q8fFbLbhpTTKRVqcjeiSDi3FK4JpVGUV4T"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Bw1X9wJzufooPWSE1D5s58x73n9qneaAmAEgaZKmQSwK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BZtgQEyS6eXUXicYPHecYQ7PybqodXQMvkjUbP4R8mUU"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EsVnksUtMXigpha6QyPmbbM4NXpwBgscG45c2sf7Vg9i"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FT2KHaZDuAFRpxwgQSgvbi4CEPP4hkGPHA9sQHyuEhST"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FULWc1hWdGMBGSB4Ut3QZBCU74muZmLmM9z9UqheWoUw"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Cxhs239pddZBnZc4adCjjfqUMRWG2JjGoRPrBvPmvGGa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("28VCpWUPgLM6VQToWsoYKM5ParBsPAN1XL3AfQ4q5h2s"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7XXCgU4QBrwArfNtayTeL8jxGygDALV1FKRNcXgQKzZo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4iJQGzZpys4N13rGXbsB3NqehPkrgrcmDUEcLw7D6GKL"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7NQjToK5NenQZvhPRc1nC1URuKJ4nuLxGPdTbpTiz9EN"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HSJS4BrYCZq9DmCNFNfctqq9o2sfTY5YQiy9n4W8djcB"),
                false,
            ),
        ];
        let expected_arranged_accounts = IncreaseLiquidityInstructionAccounts {
            nft_owner: pubkey!("5XZZSAGds4q8fFbLbhpTTKRVqcjeiSDi3FK4JpVGUV4T"),
            nft_account: pubkey!("Bw1X9wJzufooPWSE1D5s58x73n9qneaAmAEgaZKmQSwK"),
            pool_state: pubkey!("BZtgQEyS6eXUXicYPHecYQ7PybqodXQMvkjUbP4R8mUU"),
            protocol_position: pubkey!("EsVnksUtMXigpha6QyPmbbM4NXpwBgscG45c2sf7Vg9i"),
            personal_position: pubkey!("FT2KHaZDuAFRpxwgQSgvbi4CEPP4hkGPHA9sQHyuEhST"),
            tick_array_lower: pubkey!("FULWc1hWdGMBGSB4Ut3QZBCU74muZmLmM9z9UqheWoUw"),
            tick_array_upper: pubkey!("Cxhs239pddZBnZc4adCjjfqUMRWG2JjGoRPrBvPmvGGa"),
            token_account0: pubkey!("28VCpWUPgLM6VQToWsoYKM5ParBsPAN1XL3AfQ4q5h2s"),
            token_account1: pubkey!("7XXCgU4QBrwArfNtayTeL8jxGygDALV1FKRNcXgQKzZo"),
            token_vault0: pubkey!("4iJQGzZpys4N13rGXbsB3NqehPkrgrcmDUEcLw7D6GKL"),
            token_vault1: pubkey!("7NQjToK5NenQZvhPRc1nC1URuKJ4nuLxGPdTbpTiz9EN"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/increase_liquidity_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            IncreaseLiquidity::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_increase_liquidity_v2_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::IncreaseLiquidityV2(IncreaseLiquidityV2 {
            amount0_max: 224270885,
            amount1_max: 58753747,
            base_flag: None,
            liquidity: 2956211666,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("CvWwrGDV2Uw3wQaDfR3SiF8HVHk7xfZGVpxYAHPiZbiK"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("2n7rLPNmnv35BKn35nvP135HUf5CwCmbLNYAhQ7YGcx4"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3ucNos4NbumPLZNWztqGHNFFgkHeRMBQAVemeeomsUxv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9gvZ2UxwL7utj2gsvit3SNhPT4yuxSEqVJS4ZbuQkuZY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3B3sWjMhoE5CnRfzRpWoeDMqwEYnqY8rZehPrRzVtdxw"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9xJF7Gvgjv6nSUFVisYqtzipJKDmmHgDq1dAuHiozFAX"),
                false,
            ),
            AccountMeta::new(
                pubkey!("46ZrTAapCpUn9jQSMdkyELTvqCRUxbJ9zchFUwzYaye9"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2qzHVvqYqcHyQ5ZCg2peQs5iVmpPL7aTp47Lm92eqr7Z"),
                false,
            ),
            AccountMeta::new(
                pubkey!("H8ZmA5JkRXKFGJLPmHiZkcmGZkvyHSGHva6rJ81kapup"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4ct7br2vTPzfdmY3S5HLtTxcGSBfn6pnw98hsS6v359A"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5it83u57VRrVgc51oNV19TTmAJuffPx5GtGwQr7gQNUo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
        ];
        let expected_arranged_accounts = IncreaseLiquidityV2InstructionAccounts {
            nft_owner: pubkey!("CvWwrGDV2Uw3wQaDfR3SiF8HVHk7xfZGVpxYAHPiZbiK"),
            nft_account: pubkey!("2n7rLPNmnv35BKn35nvP135HUf5CwCmbLNYAhQ7YGcx4"),
            pool_state: pubkey!("3ucNos4NbumPLZNWztqGHNFFgkHeRMBQAVemeeomsUxv"),
            protocol_position: pubkey!("9gvZ2UxwL7utj2gsvit3SNhPT4yuxSEqVJS4ZbuQkuZY"),
            personal_position: pubkey!("3B3sWjMhoE5CnRfzRpWoeDMqwEYnqY8rZehPrRzVtdxw"),
            tick_array_lower: pubkey!("9xJF7Gvgjv6nSUFVisYqtzipJKDmmHgDq1dAuHiozFAX"),
            tick_array_upper: pubkey!("46ZrTAapCpUn9jQSMdkyELTvqCRUxbJ9zchFUwzYaye9"),
            token_account0: pubkey!("2qzHVvqYqcHyQ5ZCg2peQs5iVmpPL7aTp47Lm92eqr7Z"),
            token_account1: pubkey!("H8ZmA5JkRXKFGJLPmHiZkcmGZkvyHSGHva6rJ81kapup"),
            token_vault0: pubkey!("4ct7br2vTPzfdmY3S5HLtTxcGSBfn6pnw98hsS6v359A"),
            token_vault1: pubkey!("5it83u57VRrVgc51oNV19TTmAJuffPx5GtGwQr7gQNUo"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            vault0_mint: pubkey!("So11111111111111111111111111111111111111112"),
            vault1_mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/increase_liquidity_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            IncreaseLiquidityV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_decrease_liquidity_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::DecreaseLiquidity(DecreaseLiquidity {
            amount0_min: 0,
            amount1_min: 0,
            liquidity: Default::default(),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("5XZZSAGds4q8fFbLbhpTTKRVqcjeiSDi3FK4JpVGUV4T"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Bw1X9wJzufooPWSE1D5s58x73n9qneaAmAEgaZKmQSwK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FT2KHaZDuAFRpxwgQSgvbi4CEPP4hkGPHA9sQHyuEhST"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BZtgQEyS6eXUXicYPHecYQ7PybqodXQMvkjUbP4R8mUU"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EsVnksUtMXigpha6QyPmbbM4NXpwBgscG45c2sf7Vg9i"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4iJQGzZpys4N13rGXbsB3NqehPkrgrcmDUEcLw7D6GKL"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7NQjToK5NenQZvhPRc1nC1URuKJ4nuLxGPdTbpTiz9EN"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FULWc1hWdGMBGSB4Ut3QZBCU74muZmLmM9z9UqheWoUw"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Cxhs239pddZBnZc4adCjjfqUMRWG2JjGoRPrBvPmvGGa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("28VCpWUPgLM6VQToWsoYKM5ParBsPAN1XL3AfQ4q5h2s"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7XXCgU4QBrwArfNtayTeL8jxGygDALV1FKRNcXgQKzZo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2aSRZzpyqcii2TD27gZnb5emgJ1cJZ9E3WebwHbb9Lbw"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ErQK4fXFz6gPP3K6YRrzL1MimoTZ133d52GDVnvBCRrd"),
                false,
            ),
        ];
        let expected_arranged_accounts = DecreaseLiquidityInstructionAccounts {
            nft_owner: pubkey!("5XZZSAGds4q8fFbLbhpTTKRVqcjeiSDi3FK4JpVGUV4T"),
            nft_account: pubkey!("Bw1X9wJzufooPWSE1D5s58x73n9qneaAmAEgaZKmQSwK"),
            personal_position: pubkey!("FT2KHaZDuAFRpxwgQSgvbi4CEPP4hkGPHA9sQHyuEhST"),
            pool_state: pubkey!("BZtgQEyS6eXUXicYPHecYQ7PybqodXQMvkjUbP4R8mUU"),
            protocol_position: pubkey!("EsVnksUtMXigpha6QyPmbbM4NXpwBgscG45c2sf7Vg9i"),
            token_vault0: pubkey!("4iJQGzZpys4N13rGXbsB3NqehPkrgrcmDUEcLw7D6GKL"),
            token_vault1: pubkey!("7NQjToK5NenQZvhPRc1nC1URuKJ4nuLxGPdTbpTiz9EN"),
            tick_array_lower: pubkey!("FULWc1hWdGMBGSB4Ut3QZBCU74muZmLmM9z9UqheWoUw"),
            tick_array_upper: pubkey!("Cxhs239pddZBnZc4adCjjfqUMRWG2JjGoRPrBvPmvGGa"),
            recipient_token_account0: pubkey!("28VCpWUPgLM6VQToWsoYKM5ParBsPAN1XL3AfQ4q5h2s"),
            recipient_token_account1: pubkey!("7XXCgU4QBrwArfNtayTeL8jxGygDALV1FKRNcXgQKzZo"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/decrease_liquidity_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            DecreaseLiquidity::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_decrease_liquidity_v2_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::DecreaseLiquidityV2(DecreaseLiquidityV2 {
            amount0_min: 0,
            amount1_min: 0,
            liquidity: Default::default(),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("9juL1BGy8E57DQfxK87gqg3XRtE9Dj1nC8ZaLTgXJLpB"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("FaWZAmYCPNTkJVGJX3fdhdkby7nwKkL2LHg4s8B4ADEB"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EEcRPuBVhxhaC6gXH9j9XxtQPRuyY1hr6481nVLY54Ph"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3ucNos4NbumPLZNWztqGHNFFgkHeRMBQAVemeeomsUxv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BM7U6An7UsvPJkCt8WFozttxhVGr8cLDs4tHnc8pjt9K"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4ct7br2vTPzfdmY3S5HLtTxcGSBfn6pnw98hsS6v359A"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5it83u57VRrVgc51oNV19TTmAJuffPx5GtGwQr7gQNUo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ZfQjiCzkJsLeLcsBSZPrWQ94E5PMjHoKUMyMJUQnY16"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2aVwTKhZummBWAmGT8W6iKSYeZWVSHvnn5KoyCG4kQm5"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5noPVcKSevWzgNK7XYAwVvL4GXR3isZsa334R8zfgmZN"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CPfSCPPDFYSBuQhin8KLimr3cCJVBvXeM98rgvmASLsg"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HsBUudV9Y2Z2dJTieWFgK3zhrpX4ELvnfHcAwSBVqDGX"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BudyEHmsSEKB8sGQPkioaJpw156xBwYAANL5CtwFUhkB"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R"),
                false,
            ),
        ];
        let expected_arranged_accounts = DecreaseLiquidityV2InstructionAccounts {
            nft_owner: pubkey!("9juL1BGy8E57DQfxK87gqg3XRtE9Dj1nC8ZaLTgXJLpB"),
            nft_account: pubkey!("FaWZAmYCPNTkJVGJX3fdhdkby7nwKkL2LHg4s8B4ADEB"),
            personal_position: pubkey!("EEcRPuBVhxhaC6gXH9j9XxtQPRuyY1hr6481nVLY54Ph"),
            pool_state: pubkey!("3ucNos4NbumPLZNWztqGHNFFgkHeRMBQAVemeeomsUxv"),
            protocol_position: pubkey!("BM7U6An7UsvPJkCt8WFozttxhVGr8cLDs4tHnc8pjt9K"),
            token_vault0: pubkey!("4ct7br2vTPzfdmY3S5HLtTxcGSBfn6pnw98hsS6v359A"),
            token_vault1: pubkey!("5it83u57VRrVgc51oNV19TTmAJuffPx5GtGwQr7gQNUo"),
            tick_array_lower: pubkey!("ZfQjiCzkJsLeLcsBSZPrWQ94E5PMjHoKUMyMJUQnY16"),
            tick_array_upper: pubkey!("2aVwTKhZummBWAmGT8W6iKSYeZWVSHvnn5KoyCG4kQm5"),
            recipient_token_account0: pubkey!("5noPVcKSevWzgNK7XYAwVvL4GXR3isZsa334R8zfgmZN"),
            recipient_token_account1: pubkey!("CPfSCPPDFYSBuQhin8KLimr3cCJVBvXeM98rgvmASLsg"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            vault0_mint: pubkey!("So11111111111111111111111111111111111111112"),
            vault1_mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/decrease_liquidity_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            DecreaseLiquidityV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::Swap(Swap {
            amount: 1863648,
            is_base_input: true,
            other_amount_threshold: 0,
            sqrt_price_limit_x64: Default::default(),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("G55uUb8EpRrFzudVaYe2QGpRHj6WT6erm4H3UttnJ1oC"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6GiB4gYn9ZMKK5r654rZAnpYR747EjKX3KwKBxPMy98b"),
                false,
            ),
            AccountMeta::new(
                pubkey!("E7p2yWDZZDErC4hNsYKazgLfjqBwcnYohNno4fSpGWhv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8VrjZTc99eXswEkv9eTk7soVjajTejsgmBHzzenNbjEm"),
                false,
            ),
            AccountMeta::new(
                pubkey!("B7T7idkMzAbxtMYTcTNLVAcFwozZqGWnqfwwA5h7ZiJn"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FWDmGjcFhKKgW8mR3VFoxnHqFTX8D9oewQX73CkgPMJd"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ZFYeyGsqNK3qGjjqCPJqtLxzriwWWBAYztrrn3CYPvg"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2JSG8ATm5DWcdwtvokvgFyHGrGrTM4iKtkceMaYRJRLg"),
                false,
            ),
            AccountMeta::new(
                pubkey!("H1UQZrJXsDQ7qcfZGsvGhe1v9R3pi1nNwpNaRF363W4V"),
                false,
            ),
        ];
        let expected_arranged_accounts = SwapInstructionAccounts {
            payer: pubkey!("G55uUb8EpRrFzudVaYe2QGpRHj6WT6erm4H3UttnJ1oC"),
            amm_config: pubkey!("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp"),
            pool_state: pubkey!("6GiB4gYn9ZMKK5r654rZAnpYR747EjKX3KwKBxPMy98b"),
            input_token_account: pubkey!("E7p2yWDZZDErC4hNsYKazgLfjqBwcnYohNno4fSpGWhv"),
            output_token_account: pubkey!("8VrjZTc99eXswEkv9eTk7soVjajTejsgmBHzzenNbjEm"),
            input_vault: pubkey!("B7T7idkMzAbxtMYTcTNLVAcFwozZqGWnqfwwA5h7ZiJn"),
            output_vault: pubkey!("FWDmGjcFhKKgW8mR3VFoxnHqFTX8D9oewQX73CkgPMJd"),
            observation_state: pubkey!("ZFYeyGsqNK3qGjjqCPJqtLxzriwWWBAYztrrn3CYPvg"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            tick_array: pubkey!("2JSG8ATm5DWcdwtvokvgFyHGrGrTM4iKtkceMaYRJRLg"),
            remaining_accounts: vec![AccountMeta::new(
                pubkey!("H1UQZrJXsDQ7qcfZGsvGhe1v9R3pi1nNwpNaRF363W4V"),
                false,
            )],
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/swap_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            Swap::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap_v2_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::SwapV2(SwapV2 {
            amount: 4345224260,
            is_base_input: Default::default(),
            other_amount_threshold: 500000000000000,
            sqrt_price_limit_x64: 79226673521066979257578248090,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("5ukRnMzZfZUa3gNy2ECDsE63HSuMqL52pWa2ryAcnzPW"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x"),
                false,
            ),
            AccountMeta::new(
                pubkey!("945czrk5A4uSUH46xE8PLz4SMjqLXPDKqAyaGvA76bUg"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DBVe5zJdfMDtoZ8y93Nz7aKCmraq1XNKQodMfKiWPdAG"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6QRRR7s1KThgYxSBzaFbVTBnkR9XpbVhYumfJMK9hv3x"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GkYAvt7ecy7juVHyPBssmJsw6DQLZ5r1YTzf58DMLFxu"),
                false,
            ),
            AccountMeta::new(
                pubkey!("anQHGFzjoUf6F663ZVzC1irEgvXdgHdgp35knPg3wNj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2uiwPb51HMsNPfk6C2MF7mx9vcjN2coDGct1cYM96nCt"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("3hiYB9c4VYhWbwkDgSbEv22cpsNvdYZHuVWMnnNgpump"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8yxZqXeguBG6fDSGLxTVTDMGEzUgatvhDzUC4CJv6KbQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3DGHSf9AYkon5ESJv6uJaVziuMVZ5Td7GBzuLU4w5cFt"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HeV2hQbj6sxHZ8P3XW2Bho7iDzNpgC1MXiYmdTDvLpup"),
                false,
            ),
        ];
        let expected_arranged_accounts = SwapV2InstructionAccounts {
            payer: pubkey!("5ukRnMzZfZUa3gNy2ECDsE63HSuMqL52pWa2ryAcnzPW"),
            amm_config: pubkey!("9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x"),
            pool_state: pubkey!("945czrk5A4uSUH46xE8PLz4SMjqLXPDKqAyaGvA76bUg"),
            input_token_account: pubkey!("DBVe5zJdfMDtoZ8y93Nz7aKCmraq1XNKQodMfKiWPdAG"),
            output_token_account: pubkey!("6QRRR7s1KThgYxSBzaFbVTBnkR9XpbVhYumfJMK9hv3x"),
            input_vault: pubkey!("GkYAvt7ecy7juVHyPBssmJsw6DQLZ5r1YTzf58DMLFxu"),
            output_vault: pubkey!("anQHGFzjoUf6F663ZVzC1irEgvXdgHdgp35knPg3wNj"),
            observation_state: pubkey!("2uiwPb51HMsNPfk6C2MF7mx9vcjN2coDGct1cYM96nCt"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            input_vault_mint: pubkey!("3hiYB9c4VYhWbwkDgSbEv22cpsNvdYZHuVWMnnNgpump"),
            output_vault_mint: pubkey!("So11111111111111111111111111111111111111112"),
            remaining_accounts: vec![
                AccountMeta::new(
                    pubkey!("8yxZqXeguBG6fDSGLxTVTDMGEzUgatvhDzUC4CJv6KbQ"),
                    false,
                ),
                AccountMeta::new(
                    pubkey!("3DGHSf9AYkon5ESJv6uJaVziuMVZ5Td7GBzuLU4w5cFt"),
                    false,
                ),
                AccountMeta::new(
                    pubkey!("HeV2hQbj6sxHZ8P3XW2Bho7iDzNpgC1MXiYmdTDvLpup"),
                    false,
                ),
            ],
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/swap_v2_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SwapV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap_router_base_in_ix() {
        // Arrange
        let expected_ix = RaydiumClmmInstruction::SwapRouterBaseIn(SwapRouterBaseIn {
            amount_in: 200000000,
            amount_out_minimum: 0,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("3ndiEELaGFnzf5gH8MtZQNAoAsiYkzjLGogFYfsugKdX"),
                true,
            ),
            AccountMeta::new(
                pubkey!("6A9EP2fdrtoNYcFQBjRimzJ4zmUijPm5nd2xappexRR4"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4TB5TXF6VJTgYMsYrubEAvtiNKVgNE2KRfw6TiruWvFp"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
        ];
        let expected_arranged_accounts = SwapRouterBaseInInstructionAccounts {
            payer: pubkey!("3ndiEELaGFnzf5gH8MtZQNAoAsiYkzjLGogFYfsugKdX"),
            input_token_account: pubkey!("6A9EP2fdrtoNYcFQBjRimzJ4zmUijPm5nd2xappexRR4"),
            input_token_mint: pubkey!("4TB5TXF6VJTgYMsYrubEAvtiNKVgNE2KRfw6TiruWvFp"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program2022: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/swap_router_base_in_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SwapRouterBaseIn::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
