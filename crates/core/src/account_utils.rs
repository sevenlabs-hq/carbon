use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

/// Returns the next account's pubkey from the iterator, or `None` if there are no more accounts.
///
/// # Usage
/// - Use with `?` (or `expect`) when the account must be present—`None` will
///   propagate and signal a missing required account.
/// - Call it without `?` when the account is optional and you want to handle
///   the returned `Option<Pubkey>` yourself.
///
/// # Example
/// ```
/// use carbon_core::account_utils::next_account;
/// use solana_instruction::AccountMeta;
/// use solana_pubkey::Pubkey;
///
/// let accounts = vec![
///     AccountMeta::new(Pubkey::new_unique(), false),
///     AccountMeta::new_readonly(Pubkey::new_unique(), false),
/// ];
/// let mut iter = accounts.iter();
///
/// let required = next_account(&mut iter).expect("required account");
/// let optional = next_account(&mut iter);
///
/// assert_ne!(required, Pubkey::default());
/// assert!(optional.is_some());
/// ```
pub fn next_account<'a>(iter: &mut impl Iterator<Item = &'a AccountMeta>) -> Option<Pubkey> {
    Some(iter.next()?.pubkey)
}
