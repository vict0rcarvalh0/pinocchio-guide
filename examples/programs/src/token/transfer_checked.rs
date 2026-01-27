//! Instruction to transfer tokens with decimal validation.

use pinocchio::{
    AccountView,
    cpi::Signer,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::TransferChecked;

/// Processes the `TransferChecked` instruction.
///
/// # Arguments
///
/// * `accounts` - The accounts required for the instruction.
/// * `amount` - The amount of tokens to transfer.
/// * `decimals` - The number of decimal places for the token.
/// * `signers` - The signers array needed to authorize the transaction.
///
/// # Accounts
///
/// | Index | Writable | Signer | Description |
/// |-------|----------|--------|-------------|
/// | 0     | ✓        | ✗      | The source account. |
/// | 1     | ✗        | ✗      | The token mint. |
/// | 2     | ✓        | ✗      | The destination account. |
/// | 3     | ✗        | ✓      | The source account's owner/delegate. |
///
/// # Errors
///
/// * [`ProgramError::NotEnoughAccountKeys`] - If fewer than 4 accounts are provided.
/// * [`ProgramError::InvalidAccountData`] - If source or destination are not writable.
/// * [`ProgramError::MissingRequiredSignature`] - If `authority_account` is not a signer.
pub fn process_transfer_checked<'a>(
    accounts: &'a [AccountView],
    amount: u64,
    decimals: u8,
    signers: &[Signer],
) -> ProgramResult {
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let from_account = &accounts[0];
    let mint_account = &accounts[1];
    let to_account = &accounts[2];
    let authority_account = &accounts[3];

    if !from_account.is_writable() || !to_account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    TransferChecked {
        from: from_account,
        mint: mint_account,
        to: to_account,
        authority: authority_account,
        amount,
        decimals,
    }
    .invoke_signed(signers)
}
