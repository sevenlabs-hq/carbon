use {
    super::{AccountUpdate, Update, UpdateValidationError},
    solana_account::Account,
    solana_clock::Slot,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
};

/// A closed account's final state.
#[derive(Debug, Clone)]
pub struct AccountClosureUpdate {
    pub(super) pubkey: Pubkey,
    pub(super) account: Account,
    pub(super) slot: Slot,
    pub(super) transaction_signature: Option<Signature>,
    pub(super) write_version: Option<u64>,
}

impl AccountClosureUpdate {
    /// Creates an account closure update.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateValidationError::AccountNotClosed`] for nonzero lamports.
    pub fn new(
        pubkey: Pubkey,
        account: Account,
        slot: Slot,
    ) -> Result<Self, UpdateValidationError> {
        Self::try_from(AccountUpdate::new(pubkey, account, slot))
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
}

impl TryFrom<AccountUpdate> for AccountClosureUpdate {
    type Error = UpdateValidationError;

    fn try_from(update: AccountUpdate) -> Result<Self, Self::Error> {
        if update.account.lamports != 0 {
            return Err(UpdateValidationError::AccountNotClosed {
                lamports: update.account.lamports,
            });
        }

        Ok(Self {
            pubkey: update.pubkey,
            account: update.account,
            slot: update.slot,
            transaction_signature: update.transaction_signature,
            write_version: update.write_version,
        })
    }
}

impl From<AccountClosureUpdate> for Update {
    fn from(update: AccountClosureUpdate) -> Self {
        Self::AccountClosure(update)
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
    fn closure_constructors_reject_nonzero_lamports() {
        for lamports in [1, u64::MAX] {
            let pubkey = Pubkey::new_unique();
            let expected = UpdateValidationError::AccountNotClosed { lamports };
            assert_eq!(
                AccountClosureUpdate::new(pubkey, account(lamports), 7).unwrap_err(),
                expected,
            );
            assert_eq!(
                AccountClosureUpdate::try_from(AccountUpdate::new(pubkey, account(lamports), 7))
                    .unwrap_err(),
                expected,
            );
        }
    }

    #[test]
    fn closure_constructors_preserve_state() {
        let pubkey = Pubkey::new_unique();
        let account = account(0);
        let signature = Signature::new_unique();
        let closure = AccountClosureUpdate::new(pubkey, account.clone(), 7).unwrap();
        assert_eq!(closure.transaction_signature(), None);
        assert_eq!(closure.write_version(), None);

        let direct = closure
            .with_transaction_signature(signature)
            .with_write_version(11);
        let converted = AccountClosureUpdate::try_from(
            AccountUpdate::new(pubkey, account.clone(), 7)
                .with_transaction_signature(signature)
                .with_write_version(11),
        )
        .unwrap();

        for closure in [direct, converted] {
            let Update::AccountClosure(closure) = Update::from(closure) else {
                panic!("expected a closure update");
            };
            assert_eq!(closure.pubkey(), &pubkey);
            assert_eq!(closure.account(), &account);
            assert_eq!(closure.slot(), 7);
            assert_eq!(closure.transaction_signature(), Some(&signature));
            assert_eq!(closure.write_version(), Some(11));
        }
    }
}
