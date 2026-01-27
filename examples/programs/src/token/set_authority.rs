//! Instruction to set a new authority on an account.

use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::{SetAuthority, AuthorityType};

/// Processes the `SetAuthority` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `authority_type` - The type of authority to set.
/// * `new_authority` - Optional new authority.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The mint or account to change the authority of. |
/// | 1     | ✗        | ✓      | The current authority of the mint or account. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 2 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If `account_to_update` is not writable.
/// * [`ProgramError::MissingRequiredSignature`] - If `current_authority` is not a signer.
pub fn process_set_authority<'a>(
    accounts: &'a [AccountView],
    authority_type: AuthorityType,
    new_authority: Option<&Address>,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let account_to_update = &accounts[0];
    let current_authority = &accounts[1];

    if !account_to_update.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !current_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    SetAuthority {
        account: account_to_update,
        authority: current_authority,
        authority_type,
        new_authority,
    }
    .invoke_signed(signers)
}
