//! Instruction to authorize a new nonce account authority.

use pinocchio::{
    AccountView,
    Address,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AuthorizeNonceAccount;

/// Processes the `AuthorizeNonceAccount` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `new_authority` - The public key of the new authority.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The Nonce account. |
/// | 1     | ✗        | ✓      | The current Nonce authority. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 2 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If `nonce_authority` is not a signer.
pub fn process_authorize_nonce_account<'a>(
    accounts: &'a [AccountView],
    new_authority: &Address,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let nonce_authority = &accounts[1];

    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    AuthorizeNonceAccount {
        account: nonce_account,
        authority: nonce_authority,
        new_authority,
    }
    .invoke_signed(signers)
}
