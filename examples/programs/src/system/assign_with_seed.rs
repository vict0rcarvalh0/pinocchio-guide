//! Instruction to assign ownership with a seed-derived address.

use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AssignWithSeed;

/// Processes the `AssignWithSeed` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `base` - The base address used to derive the allocated account.
/// * `seed` - The seed used to derive the account.
/// * `owner` - The public key of the new program owner.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to be reassigned. |
/// | 1     | ✗        | ✓      | The base account used to derive the reassigned account. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 2 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If `base_account` is not a signer.
pub fn process_assign_with_seed<'a>(
    accounts: &'a [AccountView],
    base: &Address,
    seed: &str,
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let assigned_account = &accounts[0];
    let base_account = &accounts[1];

    if !base_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    AssignWithSeed {
        account: assigned_account,
        base: base_account,
        seed,
        owner,
    }
    .invoke_signed(signers)
}
