//! Instruction to revoke a delegate's authority.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Revoke;

/// Processes the `Revoke` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The source account. |
/// | 1     | ✗        | ✓      | The source account owner. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 2 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If `source_account` is not writable.
/// * [`ProgramError::MissingRequiredSignature`] - If `owner_account` is not a signer.
pub fn process_revoke<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let source_account = &accounts[0];
    let owner_account = &accounts[1];

    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !owner_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Revoke {
        source: source_account,
        authority: owner_account,    }
    .invoke_signed(signers)
}
