use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

/// Returns the next account's pubkey from the iterator, or `None` if there are no more accounts.
///
/// # Usage
/// - Use with `?` to indicate the account is required:
///   ```ignore
///   let required = next_account(&mut iter)?;
///   ```
///   This will propagate `None` if the account is missing.
/// - Use without `?` to handle optional accounts:
///   ```ignore
///   let optional = next_account(&mut iter);
///   ```
///   This returns `Option<Pubkey>` that you can match or use directly.
///
/// # Example
/// ```ignore
/// let mut iter = accounts.iter();
/// let required = next_account(&mut iter)?;           // required account
/// let optional = next_account(&mut iter);            // optional account
/// ```
pub fn next_account<'a>(iter: &mut impl Iterator<Item = &'a AccountMeta>) -> Option<Pubkey> {
    Some(iter.next()?.pubkey)
}
