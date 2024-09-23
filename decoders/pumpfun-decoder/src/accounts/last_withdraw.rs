use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xcb12dc677891bb02")]
pub struct LastWithdraw {
    pub last_withdraw_timestamp: i64,
}