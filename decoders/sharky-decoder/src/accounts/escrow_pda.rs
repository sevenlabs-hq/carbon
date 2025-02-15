use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xa8d91e0ef8c57680")]
pub struct EscrowPda {
    pub bump: u8,
}
