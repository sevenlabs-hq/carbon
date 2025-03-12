

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d5c65069ef898f13c")]
pub struct OrderDisplayEvent{
    pub initial_input_amount: u64,
    pub expected_output_amount: u64,
    pub remaining_input_amount: u64,
    pub filled_output_amount: u64,
    pub tip_amount: u64,
    pub number_of_fills: u64,
    pub on_event_output_amount_filled: u64,
    pub on_event_tip_amount: u64,
    pub order_type: u8,
    pub status: u8,
    pub last_updated_timestamp: u64,
}
