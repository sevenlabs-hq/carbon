use solana_sdk::signature::Keypair;

/// Generates a specified number of keypairs
///
/// # Arguments
/// * `amount` - The number of keypairs to generate
///
/// # Returns
/// A vector of `Keypair`
pub fn make_keypairs(amount: usize) -> Vec<Keypair> {
    (0..amount).map(|_| Keypair::new()).collect()
}
