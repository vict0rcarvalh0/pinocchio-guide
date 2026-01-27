//! Instruction to sync native token account balance.

use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::SyncNative;

/// Processes the `SyncNative` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The native token account to sync. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If no accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If `native_token_account` is not writable.
pub fn process_sync_native<'a>(
    accounts: &'a [AccountView],
) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let native_token_account = &accounts[0];

    if !native_token_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    SyncNative {
        native_token: native_token_account,
    }
    .invoke()
}
