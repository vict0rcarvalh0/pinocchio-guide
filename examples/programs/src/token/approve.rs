//! Instruction to approve a delegate for a token account.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::Approve;

/// Processes the `Approve` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `amount` - The amount of tokens to approve.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The token account. |
/// | 1     | ✗        | ✗      | The delegate account. |
/// | 2     | ✗        | ✓      | The source account owner. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 3 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If `source_account` is not writable.
/// * [`ProgramError::MissingRequiredSignature`] - If `authority_account` is not a signer.
pub fn process_approve<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let source_account = &accounts[0];
    let delegate_account = &accounts[1];
    let authority_account = &accounts[2];

    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Approve {
        source: source_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,    }
    .invoke_signed(signers)
}
