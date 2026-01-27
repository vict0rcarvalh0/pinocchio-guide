//! Instruction to transfer lamports between accounts.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

/// Processes the `Transfer` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `lamports` - The number of lamports to transfer.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✓      | The source account. |
/// | 1     | ✓        | ✗      | The destination account. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 2 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If accounts are not writable or source is not a signer.
pub fn process_transfer<'a>(
    accounts: &'a [AccountView],
    lamports: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let from_account = &accounts[0];
    let to_account = &accounts[1];

    if !from_account.is_writable() || !from_account.is_signer() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    Transfer {
        from: from_account,
        to: to_account,
        lamports,    }
    .invoke_signed(signers)
}
