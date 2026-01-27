//! Instruction to allocate space in an account.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Allocate;

/// Processes the `Allocate` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `space` - The number of bytes to allocate.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The account to allocate space for. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 1 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If `allocate_account` is not a signer.
pub fn process_allocate<'a>(
    accounts: &'a [AccountView],
    space: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 1 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let allocate_account = &accounts[0];

    if !allocate_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Allocate {
        account: allocate_account,
        space,    }
    .invoke_signed(signers)
}
