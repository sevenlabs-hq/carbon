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
///   let accounts = vec![
///       AccountMeta::new(Pubkey::new_unique(), false),
///       AccountMeta::new(Pubkey::new_unique(), false),
///   ];
///   let mut iter = accounts.iter();
///   let required = next_account(&mut iter).ok_or("missing account")?; // propagates None if missing
///   Ok::<(), &'static str>(())
///   ```
/// - Use without `?` to handle optional accounts:
///   ```
///   use carbon_core::account_utils::next_account;
///   use solana_instruction::AccountMeta;
///   use solana_pubkey::Pubkey;
///
///   let accounts = vec![AccountMeta::new(Pubkey::new_unique(), false)];
///   let mut iter = accounts.iter();
///   let optional = next_account(&mut iter); // Option<Pubkey>
///   Ok::<(), &'static str>(())
///   ```
///
/// # Example
/// ```
/// use carbon_core::account_utils::next_account;
/// use solana_instruction::AccountMeta;
/// use solana_pubkey::Pubkey;
///
/// let accounts = vec![
///     AccountMeta::new(Pubkey::new_unique(), false),
///     AccountMeta::new(Pubkey::new_unique(), false),
/// ];
/// let mut iter = accounts.iter();
/// let required = next_account(&mut iter).ok_or("missing account")?;           // required account
/// let optional = next_account(&mut iter);                                    // optional account
/// Ok::<(), &'static str>(())
/// ```
pub fn next_account<'a>(iter: &mut impl Iterator<Item = &'a AccountMeta>) -> Option<Pubkey> {
    Some(iter.next()?.pubkey)
}
