use crate::PROGRAM_ID;

use super::WavebreakDecoder;
pub mod authority_config_grant;
pub mod authority_config_initialize;
pub mod authority_config_revoke;
pub mod bonding_curve_close;
pub mod bonding_curve_collect_fees;
pub mod bonding_curve_graduate;
pub mod bonding_curve_initialize;
pub mod create_launch;
pub mod create_lockedlaunch;
pub mod create_presale;
pub mod graduate_manual;
pub mod graduate_whirlpool;
pub mod mint_config_close;
pub mod mint_config_initialize;
pub mod mint_config_update;
pub mod permission_config_close;
pub mod permission_config_initialize;
pub mod permission_config_update;
pub mod permission_consume_cpi;
pub mod permission_consume_top_level;
pub mod permission_refund;
pub mod permission_revoke;
pub mod reserved_authority_config_a;
pub mod reserved_authority_config_b;
pub mod reserved_authority_config_c;
pub mod reserved_authority_config_y;
pub mod reserved_authority_config_z;
pub mod reserved_bonding_curve_a;
pub mod reserved_bonding_curve_x;
pub mod reserved_bonding_curve_y;
pub mod reserved_bonding_curve_z;
pub mod reserved_create_a;
pub mod reserved_create_b;
pub mod reserved_create_c;
pub mod reserved_create_y;
pub mod reserved_create_z;
pub mod reserved_graduate_a;
pub mod reserved_graduate_b;
pub mod reserved_graduate_c;
pub mod reserved_graduate_x;
pub mod reserved_graduate_y;
pub mod reserved_graduate_z;
pub mod reserved_mint_config_a;
pub mod reserved_mint_config_b;
pub mod reserved_mint_config_c;
pub mod reserved_mint_config_y;
pub mod reserved_mint_config_z;
pub mod reserved_permission_a;
pub mod reserved_token_a;
pub mod reserved_token_y;
pub mod reserved_token_z;
pub mod token_buy_exact_in;
pub mod token_buy_exact_out;
pub mod token_refund;
pub mod token_sell_exact_in;
pub mod token_sell_exact_out;

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
pub enum WavebreakInstruction {
    PermissionConsumeTopLevel(permission_consume_top_level::PermissionConsumeTopLevel),
    PermissionConsumeCpi(permission_consume_cpi::PermissionConsumeCpi),
    PermissionConfigInitialize(permission_config_initialize::PermissionConfigInitialize),
    PermissionConfigUpdate(permission_config_update::PermissionConfigUpdate),
    PermissionConfigClose(permission_config_close::PermissionConfigClose),
    PermissionRevoke(permission_revoke::PermissionRevoke),
    PermissionRefund(permission_refund::PermissionRefund),
    ReservedPermissionA(reserved_permission_a::ReservedPermissionA),
    TokenBuyExactIn(token_buy_exact_in::TokenBuyExactIn),
    TokenBuyExactOut(token_buy_exact_out::TokenBuyExactOut),
    TokenSellExactIn(token_sell_exact_in::TokenSellExactIn),
    TokenSellExactOut(token_sell_exact_out::TokenSellExactOut),
    TokenRefund(token_refund::TokenRefund),
    ReservedTokenY(reserved_token_y::ReservedTokenY),
    ReservedTokenZ(reserved_token_z::ReservedTokenZ),
    ReservedTokenA(reserved_token_a::ReservedTokenA),
    AuthorityConfigInitialize(authority_config_initialize::AuthorityConfigInitialize),
    AuthorityConfigGrant(authority_config_grant::AuthorityConfigGrant),
    AuthorityConfigRevoke(authority_config_revoke::AuthorityConfigRevoke),
    ReservedAuthorityConfigY(reserved_authority_config_y::ReservedAuthorityConfigY),
    ReservedAuthorityConfigZ(reserved_authority_config_z::ReservedAuthorityConfigZ),
    ReservedAuthorityConfigA(reserved_authority_config_a::ReservedAuthorityConfigA),
    ReservedAuthorityConfigB(reserved_authority_config_b::ReservedAuthorityConfigB),
    ReservedAuthorityConfigC(reserved_authority_config_c::ReservedAuthorityConfigC),
    MintConfigInitialize(mint_config_initialize::MintConfigInitialize),
    MintConfigClose(mint_config_close::MintConfigClose),
    MintConfigUpdate(mint_config_update::MintConfigUpdate),
    ReservedMintConfigY(reserved_mint_config_y::ReservedMintConfigY),
    ReservedMintConfigZ(reserved_mint_config_z::ReservedMintConfigZ),
    ReservedMintConfigA(reserved_mint_config_a::ReservedMintConfigA),
    ReservedMintConfigB(reserved_mint_config_b::ReservedMintConfigB),
    ReservedMintConfigC(reserved_mint_config_c::ReservedMintConfigC),
    GraduateWhirlpool(graduate_whirlpool::GraduateWhirlpool),
    GraduateManual(graduate_manual::GraduateManual),
    ReservedGraduateX(reserved_graduate_x::ReservedGraduateX),
    ReservedGraduateY(reserved_graduate_y::ReservedGraduateY),
    ReservedGraduateZ(reserved_graduate_z::ReservedGraduateZ),
    ReservedGraduateA(reserved_graduate_a::ReservedGraduateA),
    ReservedGraduateB(reserved_graduate_b::ReservedGraduateB),
    ReservedGraduateC(reserved_graduate_c::ReservedGraduateC),
    CreateLockedlaunch(create_lockedlaunch::CreateLockedlaunch),
    CreateLaunch(create_launch::CreateLaunch),
    CreatePresale(create_presale::CreatePresale),
    ReservedCreateY(reserved_create_y::ReservedCreateY),
    ReservedCreateZ(reserved_create_z::ReservedCreateZ),
    ReservedCreateA(reserved_create_a::ReservedCreateA),
    ReservedCreateB(reserved_create_b::ReservedCreateB),
    ReservedCreateC(reserved_create_c::ReservedCreateC),
    BondingCurveInitialize(bonding_curve_initialize::BondingCurveInitialize),
    BondingCurveCollectFees(bonding_curve_collect_fees::BondingCurveCollectFees),
    BondingCurveGraduate(bonding_curve_graduate::BondingCurveGraduate),
    BondingCurveClose(bonding_curve_close::BondingCurveClose),
    ReservedBondingCurveX(reserved_bonding_curve_x::ReservedBondingCurveX),
    ReservedBondingCurveY(reserved_bonding_curve_y::ReservedBondingCurveY),
    ReservedBondingCurveZ(reserved_bonding_curve_z::ReservedBondingCurveZ),
    ReservedBondingCurveA(reserved_bonding_curve_a::ReservedBondingCurveA),
}

impl carbon_core::instruction::InstructionDecoder<'_> for WavebreakDecoder {
    type InstructionType = WavebreakInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            WavebreakInstruction::PermissionConsumeTopLevel => permission_consume_top_level::PermissionConsumeTopLevel,
            WavebreakInstruction::PermissionConsumeCpi => permission_consume_cpi::PermissionConsumeCpi,
            WavebreakInstruction::PermissionConfigInitialize => permission_config_initialize::PermissionConfigInitialize,
            WavebreakInstruction::PermissionConfigUpdate => permission_config_update::PermissionConfigUpdate,
            WavebreakInstruction::PermissionConfigClose => permission_config_close::PermissionConfigClose,
            WavebreakInstruction::PermissionRevoke => permission_revoke::PermissionRevoke,
            WavebreakInstruction::PermissionRefund => permission_refund::PermissionRefund,
            WavebreakInstruction::ReservedPermissionA => reserved_permission_a::ReservedPermissionA,
            WavebreakInstruction::TokenBuyExactIn => token_buy_exact_in::TokenBuyExactIn,
            WavebreakInstruction::TokenBuyExactOut => token_buy_exact_out::TokenBuyExactOut,
            WavebreakInstruction::TokenSellExactIn => token_sell_exact_in::TokenSellExactIn,
            WavebreakInstruction::TokenSellExactOut => token_sell_exact_out::TokenSellExactOut,
            WavebreakInstruction::TokenRefund => token_refund::TokenRefund,
            WavebreakInstruction::ReservedTokenY => reserved_token_y::ReservedTokenY,
            WavebreakInstruction::ReservedTokenZ => reserved_token_z::ReservedTokenZ,
            WavebreakInstruction::ReservedTokenA => reserved_token_a::ReservedTokenA,
            WavebreakInstruction::AuthorityConfigInitialize => authority_config_initialize::AuthorityConfigInitialize,
            WavebreakInstruction::AuthorityConfigGrant => authority_config_grant::AuthorityConfigGrant,
            WavebreakInstruction::AuthorityConfigRevoke => authority_config_revoke::AuthorityConfigRevoke,
            WavebreakInstruction::ReservedAuthorityConfigY => reserved_authority_config_y::ReservedAuthorityConfigY,
            WavebreakInstruction::ReservedAuthorityConfigZ => reserved_authority_config_z::ReservedAuthorityConfigZ,
            WavebreakInstruction::ReservedAuthorityConfigA => reserved_authority_config_a::ReservedAuthorityConfigA,
            WavebreakInstruction::ReservedAuthorityConfigB => reserved_authority_config_b::ReservedAuthorityConfigB,
            WavebreakInstruction::ReservedAuthorityConfigC => reserved_authority_config_c::ReservedAuthorityConfigC,
            WavebreakInstruction::MintConfigInitialize => mint_config_initialize::MintConfigInitialize,
            WavebreakInstruction::MintConfigClose => mint_config_close::MintConfigClose,
            WavebreakInstruction::MintConfigUpdate => mint_config_update::MintConfigUpdate,
            WavebreakInstruction::ReservedMintConfigY => reserved_mint_config_y::ReservedMintConfigY,
            WavebreakInstruction::ReservedMintConfigZ => reserved_mint_config_z::ReservedMintConfigZ,
            WavebreakInstruction::ReservedMintConfigA => reserved_mint_config_a::ReservedMintConfigA,
            WavebreakInstruction::ReservedMintConfigB => reserved_mint_config_b::ReservedMintConfigB,
            WavebreakInstruction::ReservedMintConfigC => reserved_mint_config_c::ReservedMintConfigC,
            WavebreakInstruction::GraduateWhirlpool => graduate_whirlpool::GraduateWhirlpool,
            WavebreakInstruction::GraduateManual => graduate_manual::GraduateManual,
            WavebreakInstruction::ReservedGraduateX => reserved_graduate_x::ReservedGraduateX,
            WavebreakInstruction::ReservedGraduateY => reserved_graduate_y::ReservedGraduateY,
            WavebreakInstruction::ReservedGraduateZ => reserved_graduate_z::ReservedGraduateZ,
            WavebreakInstruction::ReservedGraduateA => reserved_graduate_a::ReservedGraduateA,
            WavebreakInstruction::ReservedGraduateB => reserved_graduate_b::ReservedGraduateB,
            WavebreakInstruction::ReservedGraduateC => reserved_graduate_c::ReservedGraduateC,
            WavebreakInstruction::CreateLockedlaunch => create_lockedlaunch::CreateLockedlaunch,
            WavebreakInstruction::CreateLaunch => create_launch::CreateLaunch,
            WavebreakInstruction::CreatePresale => create_presale::CreatePresale,
            WavebreakInstruction::ReservedCreateY => reserved_create_y::ReservedCreateY,
            WavebreakInstruction::ReservedCreateZ => reserved_create_z::ReservedCreateZ,
            WavebreakInstruction::ReservedCreateA => reserved_create_a::ReservedCreateA,
            WavebreakInstruction::ReservedCreateB => reserved_create_b::ReservedCreateB,
            WavebreakInstruction::ReservedCreateC => reserved_create_c::ReservedCreateC,
            WavebreakInstruction::BondingCurveInitialize => bonding_curve_initialize::BondingCurveInitialize,
            WavebreakInstruction::BondingCurveCollectFees => bonding_curve_collect_fees::BondingCurveCollectFees,
            WavebreakInstruction::BondingCurveGraduate => bonding_curve_graduate::BondingCurveGraduate,
            WavebreakInstruction::BondingCurveClose => bonding_curve_close::BondingCurveClose,
            WavebreakInstruction::ReservedBondingCurveX => reserved_bonding_curve_x::ReservedBondingCurveX,
            WavebreakInstruction::ReservedBondingCurveY => reserved_bonding_curve_y::ReservedBondingCurveY,
            WavebreakInstruction::ReservedBondingCurveZ => reserved_bonding_curve_z::ReservedBondingCurveZ,
            WavebreakInstruction::ReservedBondingCurveA => reserved_bonding_curve_a::ReservedBondingCurveA,
        )
    }
}
