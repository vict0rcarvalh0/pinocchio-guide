//! Instruction to initialize a nonce account.

use pinocchio::{
    AccountView,
    Address,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::InitializeNonceAccount;

/// Processes the `InitializeNonceAccount` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `authority` - The public key of the entity authorized to manage the Nonce account.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The Nonce account. |
/// | 1     | ✗        | ✗      | The recent blockhashes sysvar. |
/// | 2     | ✗        | ✗      | The rent sysvar. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 3 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If `nonce_account` is not writable.
pub fn process_initialize_nonce_account<'a>(
    accounts: &'a [AccountView],
    authority: &Address,
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let recent_blockhashes_sysvar = &accounts[1];
    let rent_sysvar = &accounts[2];

    if !nonce_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    InitializeNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        rent_sysvar,
        authority,
    }
    .invoke()
}
