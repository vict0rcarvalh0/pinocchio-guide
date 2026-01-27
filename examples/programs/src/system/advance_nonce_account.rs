//! Instruction to advance a nonce account.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::AdvanceNonceAccount;

/// Processes the `AdvanceNonceAccount` instruction.
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
/// | 0     | ✓        | ✗      | The Nonce account. |
/// | 1     | ✗        | ✗      | The recent blockhashes sysvar. |
/// | 2     | ✗        | ✓      | The Nonce authority. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 3 accounts are provided.
/// * [`ProgramError::MissingRequiredSignature`] - If `nonce_authority` is not a signer.
pub fn process_advance_nonce_account<'a>(
    accounts: &'a [AccountView],
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let nonce_account = &accounts[0];
    let recent_blockhashes_sysvar = &accounts[1];
    let nonce_authority = &accounts[2];

    if !nonce_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    AdvanceNonceAccount {
        account: nonce_account,
        recent_blockhashes_sysvar,
        authority: nonce_authority,    }
    .invoke_signed(signers)
}
