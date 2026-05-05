use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

/// Returns the next account's pubkey from the iterator, or `None` if there are no more accounts.
///
/// # Usage
/// - Use with `?` to indicate the account is required:
///   ```
///   use carbon_core::account_utils::next_account;
///   use solana_instruction::AccountMeta;
///   use solana_pubkey::Pubkey;
///
///   fn first_account(accounts: &[AccountMeta]) -> Option<Pubkey> {
///       let mut iter = accounts.iter();
///       let required = next_account(&mut iter)?;
///       Some(required)
///   }
///   ```
///   This will propagate `None` if the account is missing.
/// - Use without `?` to handle optional accounts:
///   ```
///   use carbon_core::account_utils::next_account;
///   use solana_instruction::AccountMeta;
///   use solana_pubkey::Pubkey;
///
///   let accounts = vec![AccountMeta::new_readonly(Pubkey::new_unique(), false)];
///   let mut iter = accounts.iter();
///   let optional = next_account(&mut iter);
///   assert_eq!(optional, Some(accounts[0].pubkey));
///   ```
///   This returns `Option<Pubkey>` that you can match or use directly.
///
/// # Example
/// ```
/// use carbon_core::account_utils::next_account;
/// use solana_instruction::AccountMeta;
/// use solana_pubkey::Pubkey;
///
/// let accounts = vec![AccountMeta::new_readonly(Pubkey::new_unique(), false)];
/// let mut iter = accounts.iter();
/// let required = next_account(&mut iter).expect("required account");
/// let optional = next_account(&mut iter);            // optional account
/// assert_eq!(optional, None);
/// assert_eq!(required, accounts[0].pubkey);
/// ```
pub fn next_account<'a>(iter: &mut impl Iterator<Item = &'a AccountMeta>) -> Option<Pubkey> {
    Some(iter.next()?.pubkey)
}
