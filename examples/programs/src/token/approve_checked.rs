//! Instruction to approve a delegate with decimal validation.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::ApproveChecked;

/// Processes the `ApproveChecked` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `amount` - The amount of tokens to approve.
/// * `decimals` - The number of decimals for the token.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The source account. |
/// | 1     | ✗        | ✗      | The token mint. |
/// | 2     | ✗        | ✗      | The delegate account. |
/// | 3     | ✗        | ✓      | The source account owner. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 4 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If `source_account` is not writable.
/// * [`ProgramError::MissingRequiredSignature`] - If `authority_account` is not a signer.
pub fn process_approve_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let source_account = &accounts[0];
    let mint_account = &accounts[1];
    let delegate_account = &accounts[2];
    let authority_account = &accounts[3];

    if !source_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    ApproveChecked {
        source: source_account,
        mint: mint_account,
        delegate: delegate_account,
        authority: authority_account,
        amount,
        decimals,
    }
    .invoke_signed(signers)
}
