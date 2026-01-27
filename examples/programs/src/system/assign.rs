//! Instruction to assign ownership of an account.

use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Assign;

/// Processes the `Assign` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `owner` - The public key of the new program owner.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The account to be reassigned. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 1 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If `assigned_account` is not a signer.
pub fn process_assign<'a>(
    accounts: &'a [AccountView],
    owner: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 1 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let assigned_account = &accounts[0];

    if !assigned_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Assign {
        account: assigned_account,
        owner,    }
    .invoke_signed(signers)
}
