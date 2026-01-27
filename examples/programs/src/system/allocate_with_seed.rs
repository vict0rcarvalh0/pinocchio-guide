//! Instruction to allocate space with a seed-derived address.

use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AllocateWithSeed;

/// Processes the `AllocateWithSeed` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `seed` - The seed used to derive the account's address.
/// * `space` - The number of bytes to allocate.
/// * `owner` - The program that will own the allocated account.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The allocated account. |
/// | 1     | ✗        | ✓      | The base account used to derive the allocated account. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 2 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If `base_account` is not a signer.
pub fn process_allocate_with_seed<'a>(
    accounts: &'a [AccountView],
    seed: &str,
    space: u64,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let allocated_account = &accounts[0];
    let base_account = &accounts[1];

    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    AllocateWithSeed {
        account: allocated_account,
        base: base_account,
        seed,
        space,
        owner,
    }
    .invoke_signed(signers)
}
