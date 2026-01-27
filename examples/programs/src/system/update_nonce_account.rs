//! Instruction to upgrade a nonce account.

use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::UpgradeNonceAccount;

/// Processes the `UpgradeNonceAccount` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The Nonce account to upgrade. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 1 account is provided.
/// * [`ProgramError::InvalidAccountData`] - If `nonce_account` is not writable.
pub fn process_update_nonce_account<'a>(
    accounts: &'a [AccountView],
) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];

    if !nonce_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    UpgradeNonceAccount {
        account: nonce_account,
    }
    .invoke()
}
