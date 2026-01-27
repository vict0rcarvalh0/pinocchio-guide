//! Instruction to initialize a token account.

use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::InitializeAccount;

/// Processes the `InitializeAccount` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The account to initialize. |
/// | 1     | ✗        | ✗      | The mint this account will be associated with. |
/// | 2     | ✗        | ✗      | The new account's owner/multisignature. |
/// | 3     | ✗        | ✗      | Rent sysvar. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 4 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If `account_to_initialize` is not writable.
pub fn process_initialize_account<'a>(
    accounts: &'a [AccountView],
) -> ProgramResult {
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_initialize = &accounts[0];
    let mint_account = &accounts[1];
    let owner_account = &accounts[2];
    let rent_sysvar = &accounts[3];

    if !account_to_initialize.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    InitializeAccount {
        account: account_to_initialize,
        mint: mint_account,
        owner: owner_account,
        rent_sysvar,
    }
    .invoke()
}
