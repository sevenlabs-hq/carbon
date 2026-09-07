use {
    super::{AccountClosureUpdate, Update},
    solana_account::Account,
    solana_clock::Slot,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
};

/// An account's state at a slot.
#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pub(super) pubkey: Pubkey,
    pub(super) account: Account,
    pub(super) slot: Slot,
    pub(super) transaction_signature: Option<Signature>,
    pub(super) write_version: Option<u64>,
}

impl AccountUpdate {
    pub fn new(pubkey: Pubkey, account: Account, slot: Slot) -> Self {
        Self {
            pubkey,
            account,
            slot,
            transaction_signature: None,
            write_version: None,
        }
    }

    pub fn with_transaction_signature(mut self, signature: Signature) -> Self {
        self.transaction_signature = Some(signature);
        self
    }

    /// Sets the provider's write version, which is not comparable across providers.
    pub fn with_write_version(mut self, write_version: u64) -> Self {
        self.write_version = Some(write_version);
        self
    }

    pub fn pubkey(&self) -> &Pubkey {
        &self.pubkey
    }

    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn slot(&self) -> Slot {
        self.slot
    }

    pub fn transaction_signature(&self) -> Option<&Signature> {
        self.transaction_signature.as_ref()
    }

    pub fn write_version(&self) -> Option<u64> {
        self.write_version
    }

    /// Converts zero-lamport accounts to closure updates.
    pub fn into_update(self) -> Update {
        if self.account.lamports == 0 {
            return Update::AccountClosure(AccountClosureUpdate {
                pubkey: self.pubkey,
                account: self.account,
                slot: self.slot,
                transaction_signature: self.transaction_signature,
                write_version: self.write_version,
            });
        }

        Update::Account(self)
    }
}

impl From<AccountUpdate> for Update {
    fn from(update: AccountUpdate) -> Self {
        update.into_update()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account(lamports: u64) -> Account {
        Account {
            lamports,
            data: vec![1, 2, 3],
            owner: Pubkey::new_unique(),
            executable: true,
            rent_epoch: 42,
        }
    }

    #[test]
    fn account_defaults_to_unknown_optional_values() {
        let pubkey = Pubkey::new_unique();
        let account = account(1);
        let update = AccountUpdate::new(pubkey, account.clone(), 0);

        assert_eq!(update.pubkey(), &pubkey);
        assert_eq!(update.account(), &account);
        assert_eq!(update.slot(), 0);
        assert_eq!(update.transaction_signature(), None);
        assert_eq!(update.write_version(), None);
    }

    #[test]
    fn account_conversion_preserves_state_and_selects_variant() {
        for lamports in [0, 1] {
            let pubkey = Pubkey::new_unique();
            let account = account(lamports);
            let signature = Signature::new_unique();
            let value = AccountUpdate::new(pubkey, account.clone(), 7)
                .with_transaction_signature(signature)
                .with_write_version(0);

            for update in [value.clone().into_update(), Update::from(value)] {
                match update {
                    Update::Account(update) => {
                        assert_ne!(lamports, 0);
                        assert_eq!(update.pubkey(), &pubkey);
                        assert_eq!(update.account(), &account);
                        assert_eq!(update.slot(), 7);
                        assert_eq!(update.transaction_signature(), Some(&signature));
                        assert_eq!(update.write_version(), Some(0));
                    }
                    Update::AccountClosure(update) => {
                        assert_eq!(lamports, 0);
                        assert_eq!(update.pubkey(), &pubkey);
                        assert_eq!(update.account(), &account);
                        assert_eq!(update.slot(), 7);
                        assert_eq!(update.transaction_signature(), Some(&signature));
                        assert_eq!(update.write_version(), Some(0));
                    }
                    _ => panic!("expected an account or closure update"),
                }
            }
        }
    }
}
