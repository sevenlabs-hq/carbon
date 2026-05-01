//! Positional `AccountMeta` iteration helpers used by generated decoders.

use {solana_instruction::AccountMeta, solana_pubkey::Pubkey};

pub fn next_account<'a>(iter: &mut impl Iterator<Item = &'a AccountMeta>) -> Option<Pubkey> {
    Some(iter.next()?.pubkey)
}
