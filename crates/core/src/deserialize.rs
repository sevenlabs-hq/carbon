//! Provides traits and utility functions for deserialization and account arrangement within the `carbon-core` framework.
//!
//! This module includes the `CarbonDeserialize` trait for custom deserialization of data types,
//! the `extract_discriminator` function for splitting data slices by a discriminator length,
//! and the `ArrangeAccounts` trait for defining of Solana account metadata.
//!
//! # Overview
//!
//! - **`CarbonDeserialize`**: A trait for custom deserialization of data structures from byte slices.
//! - **`extract_discriminator`**: A function that separates a discriminator from the rest of a byte slice,
//!   used for parsing data with prefixed discriminators.
//! - **`ArrangeAccounts`**: A trait that allows for defining a specific arrangement of accounts, suitable
//!   for handling Solana account metadata in a customized way.
//!
//! # Notes
//!
//! - The `CarbonDeserialize` trait requires implementers to also implement `borsh::BorshDeserialize`.
//! - Ensure that `extract_discriminator` is used with data slices large enough to avoid runtime errors.
//! - Implement `ArrangeAccounts` when you need to access account metadata for Solana instructions.

/// A trait for custom deserialization of types from byte slices.
///
/// The `CarbonDeserialize` trait provides a method for deserializing instances of a type from
/// raw byte slices. This is essential for parsing binary data into structured types within the
/// `carbon-core` framework. Types implementing this trait should also implement `BorshDeserialize`
/// to support Borsh-based serialization.
///
/// # Notes
///
/// - Implementing this trait enables custom deserialization logic for types, which is useful
///   for processing raw blockchain data.
/// - Ensure the data slice passed to `deserialize` is valid and of appropriate length to avoid errors.

pub trait CarbonDeserialize
where
    Self: Sized + crate::borsh::BorshDeserialize,
{
    fn deserialize(data: &[u8]) -> Option<Self>;
}

/// Extracts a discriminator from the beginning of a byte slice and returns the discriminator and remaining data.
///
/// The `extract_discriminator` function takes a slice of bytes and separates a portion of it,
/// specified by the `length` parameter, from the rest of the data. This is commonly used
/// in scenarios where data is prefixed with a discriminator value, such as Solana transactions
/// and accounts.
///
/// # Parameters
///
/// - `length`: The length of the discriminator prefix to extract.
/// - `data`: The full data slice from which to extract the discriminator.
///
/// # Returns
///
/// Returns an `Option` containing a tuple of slices:
/// - The first slice is the discriminator of the specified length.
/// - The second slice is the remaining data following the discriminator.
/// Returns `None` if the `data` slice is shorter than the specified `length`.
///
/// # Notes
///
/// - Ensure that `data` is at least as long as `length` to avoid `None` being returned.
/// - This function is particularly useful for decoding prefixed data structures, such as
///   those commonly found in Solana transactions.
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

/// A trait for defining a custom arrangement of Solana account metadata.
///
/// The `ArrangeAccounts` trait provides an interface for structuring account
/// metadata in a custom format.
///
/// # Associated Types
///
/// - `ArrangedAccounts`: The output type representing the custom arrangement of accounts.
///
pub trait ArrangeAccounts {
    type ArrangedAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts>;
}
