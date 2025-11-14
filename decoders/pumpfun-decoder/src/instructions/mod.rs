use carbon_core::deserialize::CarbonDeserialize;
use solana_instruction::Instruction;

use crate::PROGRAM_ID;

use super::PumpfunDecoder;

pub mod admin_set_creator;
pub mod admin_set_creator_event;
pub mod admin_set_idl_authority;
pub mod admin_set_idl_authority_event;
pub mod admin_update_token_incentives;
pub mod admin_update_token_incentives_event;
pub mod buy;
pub mod buy_exact_sol_in;
pub mod claim_token_incentives;
pub mod claim_token_incentives_event;
pub mod close_user_volume_accumulator;
pub mod close_user_volume_accumulator_event;
pub mod collect_creator_fee;
pub mod collect_creator_fee_event;
pub mod complete_event;
pub mod complete_pump_amm_migration_event;
pub mod create;
pub mod create_event;
pub mod create_v2;
pub mod extend_account;
pub mod extend_account_event;
pub mod init_user_volume_accumulator;
pub mod init_user_volume_accumulator_event;
pub mod initialize;
pub mod migrate;
pub mod sell;
pub mod set_creator;
pub mod set_creator_event;
pub mod set_metaplex_creator;
pub mod set_metaplex_creator_event;
pub mod set_params;
pub mod set_params_event;
pub mod sync_user_volume_accumulator;
pub mod sync_user_volume_accumulator_event;
pub mod trade_event;
pub mod update_global_authority;
pub mod update_global_authority_event;

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
pub enum PumpfunInstruction {
    AdminSetCreator(admin_set_creator::AdminSetCreator),
    AdminSetIdlAuthority(admin_set_idl_authority::AdminSetIdlAuthority),
    AdminUpdateTokenIncentives(admin_update_token_incentives::AdminUpdateTokenIncentives),
    Buy(buy::Buy),
    ClaimTokenIncentives(claim_token_incentives::ClaimTokenIncentives),
    CloseUserVolumeAccumulator(close_user_volume_accumulator::CloseUserVolumeAccumulator),
    CollectCreatorFee(collect_creator_fee::CollectCreatorFee),
    Create(create::Create),
    CreateV2(create_v2::CreateV2),
    ExtendAccount(extend_account::ExtendAccount),
    InitUserVolumeAccumulator(init_user_volume_accumulator::InitUserVolumeAccumulator),
    Initialize(initialize::Initialize),
    Migrate(migrate::Migrate),
    Sell(sell::Sell),
    SetCreator(set_creator::SetCreator),
    SetMetaplexCreator(set_metaplex_creator::SetMetaplexCreator),
    SetParams(set_params::SetParams),
    SyncUserVolumeAccumulator(sync_user_volume_accumulator::SyncUserVolumeAccumulator),
    UpdateGlobalAuthority(update_global_authority::UpdateGlobalAuthority),
    AdminSetCreatorEvent(admin_set_creator_event::AdminSetCreatorEvent),
    AdminSetIdlAuthorityEvent(admin_set_idl_authority_event::AdminSetIdlAuthorityEvent),
    AdminUpdateTokenIncentivesEvent(
        admin_update_token_incentives_event::AdminUpdateTokenIncentivesEvent,
    ),
    ClaimTokenIncentivesEvent(claim_token_incentives_event::ClaimTokenIncentivesEvent),
    CloseUserVolumeAccumulatorEvent(
        close_user_volume_accumulator_event::CloseUserVolumeAccumulatorEvent,
    ),
    CollectCreatorFeeEvent(collect_creator_fee_event::CollectCreatorFeeEvent),
    CompleteEvent(complete_event::CompleteEvent),
    CompletePumpAmmMigrationEvent(complete_pump_amm_migration_event::CompletePumpAmmMigrationEvent),
    CreateEvent(create_event::CreateEvent),
    ExtendAccountEvent(extend_account_event::ExtendAccountEvent),
    InitUserVolumeAccumulatorEvent(
        init_user_volume_accumulator_event::InitUserVolumeAccumulatorEvent,
    ),
    SetCreatorEvent(set_creator_event::SetCreatorEvent),
    SetMetaplexCreatorEvent(set_metaplex_creator_event::SetMetaplexCreatorEvent),
    SetParamsEvent(set_params_event::SetParamsEvent),
    SyncUserVolumeAccumulatorEvent(
        sync_user_volume_accumulator_event::SyncUserVolumeAccumulatorEvent,
    ),
    TradeEvent(trade_event::TradeEvent),
    UpdateGlobalAuthorityEvent(update_global_authority_event::UpdateGlobalAuthorityEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for PumpfunDecoder {
    type InstructionType = PumpfunInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        let instruction = if !instruction.data.is_empty()
            && instruction.data[..8] == *buy::Buy::DISCRIMINATOR
            && instruction.data.len() == 24
        {
            let mut data = instruction.data.clone();
            data.push(0);
            &Instruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts.clone(),
                data,
            }
        } else {
            instruction
        };
        carbon_core::try_decode_instructions!(instruction,
            PumpfunInstruction::AdminSetCreator => admin_set_creator::AdminSetCreator,
            PumpfunInstruction::AdminSetIdlAuthority => admin_set_idl_authority::AdminSetIdlAuthority,
            PumpfunInstruction::AdminUpdateTokenIncentives => admin_update_token_incentives::AdminUpdateTokenIncentives,
            PumpfunInstruction::Buy => buy::Buy,
            PumpfunInstruction::ClaimTokenIncentives => claim_token_incentives::ClaimTokenIncentives,
            PumpfunInstruction::CloseUserVolumeAccumulator => close_user_volume_accumulator::CloseUserVolumeAccumulator,
            PumpfunInstruction::CollectCreatorFee => collect_creator_fee::CollectCreatorFee,
            PumpfunInstruction::Create => create::Create,
            PumpfunInstruction::CreateV2 => create_v2::CreateV2,
            PumpfunInstruction::ExtendAccount => extend_account::ExtendAccount,
            PumpfunInstruction::InitUserVolumeAccumulator => init_user_volume_accumulator::InitUserVolumeAccumulator,
            PumpfunInstruction::Initialize => initialize::Initialize,
            PumpfunInstruction::Migrate => migrate::Migrate,
            PumpfunInstruction::Sell => sell::Sell,
            PumpfunInstruction::SetCreator => set_creator::SetCreator,
            PumpfunInstruction::SetMetaplexCreator => set_metaplex_creator::SetMetaplexCreator,
            PumpfunInstruction::SetParams => set_params::SetParams,
            PumpfunInstruction::SyncUserVolumeAccumulator => sync_user_volume_accumulator::SyncUserVolumeAccumulator,
            PumpfunInstruction::UpdateGlobalAuthority => update_global_authority::UpdateGlobalAuthority,
            PumpfunInstruction::AdminSetCreatorEvent => admin_set_creator_event::AdminSetCreatorEvent,
            PumpfunInstruction::AdminSetIdlAuthorityEvent => admin_set_idl_authority_event::AdminSetIdlAuthorityEvent,
            PumpfunInstruction::AdminUpdateTokenIncentivesEvent => admin_update_token_incentives_event::AdminUpdateTokenIncentivesEvent,
            PumpfunInstruction::ClaimTokenIncentivesEvent => claim_token_incentives_event::ClaimTokenIncentivesEvent,
            PumpfunInstruction::CloseUserVolumeAccumulatorEvent => close_user_volume_accumulator_event::CloseUserVolumeAccumulatorEvent,
            PumpfunInstruction::CollectCreatorFeeEvent => collect_creator_fee_event::CollectCreatorFeeEvent,
            PumpfunInstruction::CompleteEvent => complete_event::CompleteEvent,
            PumpfunInstruction::CompletePumpAmmMigrationEvent => complete_pump_amm_migration_event::CompletePumpAmmMigrationEvent,
            PumpfunInstruction::CreateEvent => create_event::CreateEvent,
            PumpfunInstruction::ExtendAccountEvent => extend_account_event::ExtendAccountEvent,
            PumpfunInstruction::InitUserVolumeAccumulatorEvent => init_user_volume_accumulator_event::InitUserVolumeAccumulatorEvent,
            PumpfunInstruction::SetCreatorEvent => set_creator_event::SetCreatorEvent,
            PumpfunInstruction::SetMetaplexCreatorEvent => set_metaplex_creator_event::SetMetaplexCreatorEvent,
            PumpfunInstruction::SetParamsEvent => set_params_event::SetParamsEvent,
            PumpfunInstruction::SyncUserVolumeAccumulatorEvent => sync_user_volume_accumulator_event::SyncUserVolumeAccumulatorEvent,
            PumpfunInstruction::TradeEvent => trade_event::TradeEvent,
            PumpfunInstruction::UpdateGlobalAuthorityEvent => update_global_authority_event::UpdateGlobalAuthorityEvent,
        )
    }
}
#[cfg(test)]
mod tests {
    use alloc::{borrow::ToOwned, vec};
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_instruction::AccountMeta;
    use solana_pubkey::pubkey;

    use crate::types::OptionBool;

    use super::*;

    #[test]
    fn test_decode_buy() {
        let expected_ix = PumpfunInstruction::Buy(buy::Buy {
            amount: 1690358,
            max_sol_cost: 195,
            track_volume: OptionBool(false),
        });

        let expected_accounts = vec![
            AccountMeta {
                pubkey: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("7uUTzNs4UFgarqY6TyRnW3ZnMFpcsw1iookL8R5KCGwA"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("CxHfPAgH6PNLnfq5n5mq4tZK1aQCsMdeygwqaUSjsceN"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("3AnftTe9GeUkp3HQJTm7tWLeVkojdFCyd33u4XEAMTpe"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("A1C8VqECGc8CSjmiEpaaNTWeJVqVM3tQXnxcaSkfngxT"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("9zR4L1w4cCvLndDYYiCBUvNfBPeZBAQWL57wQgW97Pau"),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("11111111111111111111111111111111"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("34D2HZjdrSfxhc4j6aduiwbN6uzXKaw73h2jjVBQfJ9p"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("Hq2wp8uJ9jCPsYgNHex8RtqdvMPfVGoYwjvF1ATiwn2Y"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("3cm5x9jFRrZQzZxnYkm7jYkUy5srwamyMZZTsRPZqfgD"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"),
                is_signer: false,
                is_writable: false,
            },
        ];
        let expected_arranged_accounts = buy::BuyInstructionAccounts {
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: pubkey!("7uUTzNs4UFgarqY6TyRnW3ZnMFpcsw1iookL8R5KCGwA"),
            bonding_curve: pubkey!("CxHfPAgH6PNLnfq5n5mq4tZK1aQCsMdeygwqaUSjsceN"),
            associated_bonding_curve: pubkey!("3AnftTe9GeUkp3HQJTm7tWLeVkojdFCyd33u4XEAMTpe"),
            associated_user: pubkey!("A1C8VqECGc8CSjmiEpaaNTWeJVqVM3tQXnxcaSkfngxT"),
            user: pubkey!("9zR4L1w4cCvLndDYYiCBUvNfBPeZBAQWL57wQgW97Pau"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            creator_vault: pubkey!("34D2HZjdrSfxhc4j6aduiwbN6uzXKaw73h2jjVBQfJ9p"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
            global_volume_accumulator: pubkey!("Hq2wp8uJ9jCPsYgNHex8RtqdvMPfVGoYwjvF1ATiwn2Y"),
            user_volume_accumulator: pubkey!("3cm5x9jFRrZQzZxnYkm7jYkUy5srwamyMZZTsRPZqfgD"),
            fee_config: pubkey!("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt"),
            fee_program: pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"),
        };

        // Act
        let decoder = PumpfunDecoder;
        // The buy fixture is from devnet
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/buy_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            buy::Buy::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_sell() {
        // Arrange
        let expected_ix = PumpfunInstruction::Sell(sell::Sell {
            amount: 376599365021,
            min_sol_output: 14065038,
        });
        let expected_accounts = vec![
            AccountMeta {
                pubkey: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("7gym364csXoyZHvN8tswevxxXcYRZ9hHdP1PaS99XWtf"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("7kEiL6XFg24xdPbTWnae8URUud9NxnDZX95rQyxKGQvB"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("ANq2idorZ1Ha9bjzWzUquKLho9H4ZpK5AHH7P3oTsRQA"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("HVAi5g9JEMHTjBKFaBaNebtVNVVisRh8CAAPrddFVx5G"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("GGrcscAqEq4qJ1apGnQMv3XkFL7Dp68C6Cm8wx4ytjuf"),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("11111111111111111111111111111111"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("7stsnMwqhVnmUKdBdbKKHbLYm59V2W3NiihibhTwXq2b"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"),
                is_signer: false,
                is_writable: false,
            },
        ];

        let expected_arranged_accounts = sell::SellInstructionAccounts {
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: pubkey!("7gym364csXoyZHvN8tswevxxXcYRZ9hHdP1PaS99XWtf"),
            bonding_curve: pubkey!("7kEiL6XFg24xdPbTWnae8URUud9NxnDZX95rQyxKGQvB"),
            associated_bonding_curve: pubkey!("ANq2idorZ1Ha9bjzWzUquKLho9H4ZpK5AHH7P3oTsRQA"),
            associated_user: pubkey!("HVAi5g9JEMHTjBKFaBaNebtVNVVisRh8CAAPrddFVx5G"),
            user: pubkey!("GGrcscAqEq4qJ1apGnQMv3XkFL7Dp68C6Cm8wx4ytjuf"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            creator_vault: pubkey!("7stsnMwqhVnmUKdBdbKKHbLYm59V2W3NiihibhTwXq2b"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
            fee_config: pubkey!("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt"),
            fee_program: pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ"),
        };

        // Act
        let decoder = PumpfunDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/sell_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            sell::Sell::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_create() {
        // Arrange
        let expected_ix = PumpfunInstruction::Create(create::Create {
            name: "Mystical Cat".to_owned(),
            symbol: "MC".to_owned(),
            uri: "https://ipfs.io/ipfs/QmeXR4TB9NZVK279DAVDKevSLzrWSi9papnaknVPJ8AvHe".to_owned(),
            creator: pubkey!("CkdtUhQdH2sHXJYTJTNFbF1K5W33WVgVHG7zffaMkEmv"),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AC69oJv1m7843mdRfoQDneZuyRxYrMq86i2mARMtpump"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("623TpUDcZjKdmd9wybMveLKSSbgRs2hvwFjygzi4g15B"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5rQKu3z4SXShvQkNKSJu9mtsVmgM8AvLoeNbJGvTyQv6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ),
            AccountMeta::new(
                pubkey!("311RNHU1xDgJfYvQLszzgnBjQ1fDg5Ddr2yR3ymmUCed"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CkdtUhQdH2sHXJYTJTNFbF1K5W33WVgVHG7zffaMkEmv"),
                true,
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
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = create::CreateInstructionAccounts {
            mint: pubkey!("AC69oJv1m7843mdRfoQDneZuyRxYrMq86i2mARMtpump"),
            mint_authority: pubkey!("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"),
            bonding_curve: pubkey!("623TpUDcZjKdmd9wybMveLKSSbgRs2hvwFjygzi4g15B"),
            associated_bonding_curve: pubkey!("5rQKu3z4SXShvQkNKSJu9mtsVmgM8AvLoeNbJGvTyQv6"),
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            mpl_token_metadata: pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            metadata: pubkey!("311RNHU1xDgJfYvQLszzgnBjQ1fDg5Ddr2yR3ymmUCed"),
            user: pubkey!("CkdtUhQdH2sHXJYTJTNFbF1K5W33WVgVHG7zffaMkEmv"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
        };

        // Act
        let decoder = PumpfunDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/create_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            create::Create::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
