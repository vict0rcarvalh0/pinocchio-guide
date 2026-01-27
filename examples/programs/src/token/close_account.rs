//! Instruction to close a token account.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::CloseAccount;

/// Processes the `CloseAccount` instruction.
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
/// | 0     | ✓        | ✗      | The account to close. |
/// | 1     | ✓        | ✗      | The destination account. |
/// | 2     | ✗        | ✓      | The account's owner. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 3 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If accounts are not writable.
/// * [`ProgramError::MissingRequiredSignature`] - If `authority_account` is not a signer.
pub fn process_close_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let close_account = &accounts[0];
    let destination_account = &accounts[1];
    let authority_account = &accounts[2];

    if !close_account.is_writable() || !destination_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    CloseAccount {
        account: close_account,
        destination: destination_account,
        authority: authority_account,    }
    .invoke_signed(signers)
}
