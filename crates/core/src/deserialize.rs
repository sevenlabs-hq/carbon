pub trait CarbonDeserialize
where
    Self: Sized + crate::borsh::BorshDeserialize,
{
    fn deserialize(data: &[u8]) -> Option<Self>;
}

pub fn extract_discriminator(length: usize, data: &[u8]) -> Option<(&[u8], &[u8])> {
    log::trace!(
        "extract_discriminator(length: {:?}, data: {:?})",
        length,
        data
    );

    if data.len() < length {
        return None;
    }

    Some((&data[..length], &data[length..]))
}

pub trait ArrangeAccounts {
    type ArrangedAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts>;
}
