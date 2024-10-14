use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Checks whether a given string slice is a valid Solana address
///
/// # Arguments
/// * `address` - A string slice representing an address to validate
///
/// # Returns
/// `true` if the input is a valid Solana address based on the following criteria:
/// - Have a length between 32 and 44 characters, inclusive
/// - Be parsable as a Solana 'Pubkey` without any errors
pub fn is_valid_solana_address(address: &str) -> bool {
    address.len() >= 32 && address.len() <= 44 && Pubkey::from_str(address).is_ok()
}
