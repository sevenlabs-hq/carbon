use crate::update::AccountUpdate;

/// An account update and its decoded data.
#[derive(Debug)]
pub struct AccountProcessorInput<'a, T> {
    pub(crate) update: &'a AccountUpdate,
    pub(crate) decoded: T,
}

impl<'a, T> AccountProcessorInput<'a, T> {
    pub fn update(&self) -> &'a AccountUpdate {
        self.update
    }

    pub fn decoded(&self) -> &T {
        &self.decoded
    }
}

#[cfg(test)]
mod tests {
    use {super::*, solana_account::Account, solana_pubkey::Pubkey};

    #[test]
    fn input_borrows_update_and_owns_decoded_data() {
        let update =
            AccountUpdate::new(Pubkey::new_unique(), Account::default(), 7).with_write_version(12);
        let decoded = String::from("decoded account");
        let decoded_ptr = decoded.as_ptr();
        let input = AccountProcessorInput {
            update: &update,
            decoded,
        };

        assert!(std::ptr::eq(input.update(), &update));
        assert_eq!(input.decoded(), "decoded account");
        assert_eq!(input.decoded().as_ptr(), decoded_ptr);
        assert_eq!(input.update().write_version(), Some(12));
    }
}
